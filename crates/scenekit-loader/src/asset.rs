use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use scenekit_core::{
    AnimationClipId, AssetId, CameraId, LightId, MaterialId, MeshId, ScenixError, SkinId, TextureId,
};
use scenekit_material::{PbrMaterial, PhysicalMaterial, UnlitMaterial};
use scenekit_math::{Mat4, Vec2, Vec3};
use scenekit_mesh::{Geometry, MorphTarget};
use scenekit_scene::SceneGraph;
use scenekit_texture::{Sampler, Texture2D, TextureCube};

use crate::gltf::{GltfAsset, LoadedCamera, LoadedLight};

/// 产生 [`AssetPackage`] 的来源。
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssetSource {
    /// 本地文件路径。
    Path(PathBuf),
    /// 远程 URL。
    Url(String),
    /// 调用者拥有的字节缓冲区。
    Bytes { label: String, len: usize },
}

/// 资产管理器的文件或字节请求。
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetRequest {
    /// 稳定的请求标识符。
    pub id: AssetId,
    /// 要加载的来源。
    pub source: AssetSource,
}

impl AssetRequest {
    /// 创建路径请求。
    #[inline]
    pub fn path(id: AssetId, path: impl Into<PathBuf>) -> Self {
        Self {
            id,
            source: AssetSource::Path(path.into()),
        }
    }

    /// 创建 URL 请求。
    #[inline]
    pub fn url(id: AssetId, url: impl Into<String>) -> Self {
        Self {
            id,
            source: AssetSource::Url(url.into()),
        }
    }

    /// 创建嵌入式字节请求。
    #[inline]
    pub fn bytes(id: AssetId, label: impl Into<String>, len: usize) -> Self {
        Self {
            id,
            source: AssetSource::Bytes {
                label: label.into(),
                len,
            },
        }
    }
}

/// 导入诊断的严重性级别。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssetDiagnosticSeverity {
    /// 信息性元数据。
    Info,
    /// 使用回退或部分映射导入的功能。
    Warning,
    /// 当前构建无法导入的功能。
    Unsupported,
}

/// 结构化的导入/导出诊断。
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetDiagnostic {
    /// 严重性。
    pub severity: AssetDiagnosticSeverity,
    /// 机器可读的代码。
    pub code: String,
    /// 人类可读的消息。
    pub message: String,
}

impl AssetDiagnostic {
    /// 创建信息性诊断。
    #[inline]
    pub fn info(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(AssetDiagnosticSeverity::Info, code, message)
    }

    /// 创建警告诊断。
    #[inline]
    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(AssetDiagnosticSeverity::Warning, code, message)
    }

    /// 创建不支持功能的诊断。
    #[inline]
    pub fn unsupported(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(AssetDiagnosticSeverity::Unsupported, code, message)
    }

    /// 创建诊断。
    #[inline]
    pub fn new(
        severity: AssetDiagnosticSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code: code.into(),
            message: message.into(),
        }
    }
}

/// 一个被追踪的资产依赖项。
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetDependency {
    /// 源路径、URL 或字节标签。
    pub source: AssetSource,
    /// 尽力估算的字节大小。
    pub bytes: usize,
    /// 本地路径的最后修改时间戳。
    #[cfg_attr(feature = "serde", serde(skip))]
    pub modified: Option<SystemTime>,
}

/// 用于失效和热重载的依赖图。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetDependencyGraph {
    /// 按确定性源顺序排列的依赖项。
    pub dependencies: Vec<AssetDependency>,
}

impl AssetDependencyGraph {
    /// 创建空图。
    #[inline]
    pub const fn new() -> Self {
        Self {
            dependencies: Vec::new(),
        }
    }

    /// 记录本地路径。
    pub fn record_path(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        let metadata = std::fs::metadata(path).ok();
        self.dependencies.push(AssetDependency {
            source: AssetSource::Path(path.to_path_buf()),
            bytes: metadata.as_ref().map_or(0, |m| m.len() as usize),
            modified: metadata.and_then(|m| m.modified().ok()),
        });
    }

    /// 记录远程 URL。
    pub fn record_url(&mut self, url: impl Into<String>, bytes: usize) {
        self.dependencies.push(AssetDependency {
            source: AssetSource::Url(url.into()),
            bytes,
            modified: None,
        });
    }

    /// 记录嵌入式字节依赖项。
    pub fn record_bytes(&mut self, label: impl Into<String>, bytes: usize) {
        self.dependencies.push(AssetDependency {
            source: AssetSource::Bytes {
                label: label.into(),
                len: bytes,
            },
            bytes,
            modified: None,
        });
    }

    /// 返回总的追踪依赖项字节数。
    #[inline]
    pub fn total_bytes(&self) -> usize {
        self.dependencies.iter().map(|dep| dep.bytes).sum()
    }

    /// 返回任何本地依赖项是否在磁盘上发生了变更。
    pub fn is_stale(&self) -> bool {
        self.dependencies.iter().any(|dep| {
            let AssetSource::Path(path) = &dep.source else {
                return false;
            };
            let Ok(metadata) = std::fs::metadata(path) else {
                return true;
            };
            metadata.modified().ok() != dep.modified
        })
    }
}

/// 从 `KHR_texture_transform` 导入的纹理变换。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextureTransform {
    /// UV 偏移。
    pub offset: Vec2,
    /// UV 旋转（弧度）。
    pub rotation: f32,
    /// UV 缩放。
    pub scale: Vec2,
    /// 可选的覆盖纹理坐标集。
    pub tex_coord: Option<u32>,
}

impl Default for TextureTransform {
    #[inline]
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
            tex_coord: None,
        }
    }
}

/// 从 `KHR_materials_variants` 导入的材质变体元数据。
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MaterialVariant {
    /// 源变体索引。
    pub index: u32,
    /// 源变体名称（如果可用）。
    pub name: String,
}

/// 资产管线导入的渲染器无关材质。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoadedMaterial {
    /// 金属度-粗糙度 PBR 材质。
    Pbr(PbrMaterial),
    /// 从 glTF 扩展映射的高级物理材质。
    Physical(PhysicalMaterial),
    /// 从 `KHR_materials_unlit` 映射的无光照材质。
    Unlit(UnlitMaterial),
}

impl LoadedMaterial {
    /// 返回 PBR 兼容的基础材质。
    #[inline]
    pub fn base_pbr(&self) -> PbrMaterial {
        match self {
            Self::Pbr(material) => material.clone(),
            Self::Physical(material) => material.base.clone(),
            Self::Unlit(material) => PbrMaterial::new()
                .named(material.name.clone())
                .albedo(material.color)
                .alpha_mode(material.alpha_mode)
                .double_sided(material.double_sided),
        }
    }
}

/// 导入的骨架/蒙皮元数据。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LoadedSkin {
    /// 蒙皮 ID。
    pub id: SkinId,
    /// 人类可读的名称。
    pub name: String,
    /// 源关节节点索引。
    pub joints: Vec<usize>,
    /// 源骨架根节点索引。
    pub skeleton_root: Option<usize>,
    /// 按关节顺序排列的逆绑定矩阵。
    pub inverse_bind_matrices: Vec<Mat4>,
}

/// 为一个网格图元导入的逐顶点蒙皮属性。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LoadedMeshSkinAttributes {
    /// 第一个 glTF 关节集中的关节索引。
    pub joints: Vec<[u16; 4]>,
    /// 第一个 glTF 权重集中的关节权重。
    pub weights: Vec<[f32; 4]>,
    /// 实例化此网格的节点所使用的蒙皮（如果已知）。
    pub skin: Option<SkinId>,
}

/// 导入的动画目标属性。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoadedAnimationProperty {
    /// 节点平移。
    Translation,
    /// 节点旋转。
    Rotation,
    /// 节点缩放。
    Scale,
    /// 变形目标权重。
    MorphTargetWeights,
}

/// 导入的动画插值方式。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoadedAnimationInterpolation {
    /// 线性插值。
    Linear,
    /// 阶跃插值。
    Step,
    /// 三次样条插值。
    CubicSpline,
}

/// 一个导入的动画通道。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LoadedAnimationChannel {
    /// 源节点索引。
    pub node_index: usize,
    /// 目标属性。
    pub property: LoadedAnimationProperty,
    /// 插值模式。
    pub interpolation: LoadedAnimationInterpolation,
    /// 以秒为单位的关键帧时间。
    pub times: Vec<f32>,
    /// 按每个关键帧 `output_components` 个打包的已解码输出值（v1.4.0）。
    ///
    /// 对于 `CubicSpline` 插值，每个关键帧的布局为
    /// `[in_tangent, value, out_tangent]`，与 glTF 规范一致。
    pub output: Vec<f32>,
    /// 一个关键帧值的输出分量数。
    pub output_components: usize,
}

/// 导入的动画剪辑元数据和关键帧时间。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LoadedAnimationClip {
    /// 剪辑 ID。
    pub id: AnimationClipId,
    /// 人类可读的名称。
    pub name: String,
    /// 以秒为单位的持续时间。
    pub duration: f32,
    /// 按源顺序排列的通道。
    pub channels: Vec<LoadedAnimationChannel>,
}

/// v1.3 资产管线生成的 CPU 端包。
pub struct AssetPackage {
    /// 稳定的包 ID。
    pub id: AssetId,
    /// 人类可读的标签。
    pub label: String,
    /// 产生此包的来源。
    pub source: Option<AssetSource>,
    /// 可渲染的场景图。
    pub scene: SceneGraph,
    /// 导入的网格几何体。
    pub meshes: BTreeMap<MeshId, Geometry>,
    /// 用于现有渲染器路径的 PBR 兼容材质。
    pub materials: BTreeMap<MaterialId, PbrMaterial>,
    /// 完整导入的材质变体。
    pub loaded_materials: BTreeMap<MaterialId, LoadedMaterial>,
    /// 2D 纹理。
    pub textures: BTreeMap<TextureId, Texture2D>,
    /// 立方体贴图。
    pub texture_cubes: BTreeMap<TextureId, TextureCube>,
    /// 按纹理 ID 键控的采样器。
    pub samplers: BTreeMap<TextureId, Sampler>,
    /// 导入的光源。
    pub lights: BTreeMap<LightId, LoadedLight>,
    /// 导入的相机。
    pub cameras: BTreeMap<CameraId, LoadedCamera>,
    /// 按网格 ID 键控的变形目标。
    pub morph_targets: BTreeMap<MeshId, Vec<MorphTarget>>,
    /// 按网格 ID 键控的蒙皮属性。
    pub mesh_skin_attributes: BTreeMap<MeshId, LoadedMeshSkinAttributes>,
    /// 导入的蒙皮。
    pub skins: BTreeMap<SkinId, LoadedSkin>,
    /// 导入的动画剪辑。
    pub animations: BTreeMap<AnimationClipId, LoadedAnimationClip>,
    /// 按材质 ID 和材质槽位名称键控的纹理变换。
    pub texture_transforms: BTreeMap<(MaterialId, String), TextureTransform>,
    /// 材质变体。
    pub material_variants: Vec<MaterialVariant>,
    /// 依赖图。
    pub dependency_graph: AssetDependencyGraph,
    /// 导入诊断。
    pub diagnostics: Vec<AssetDiagnostic>,
}

impl AssetPackage {
    /// 创建空包。
    pub fn empty(id: AssetId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            source: None,
            scene: SceneGraph::new(),
            meshes: BTreeMap::new(),
            materials: BTreeMap::new(),
            loaded_materials: BTreeMap::new(),
            textures: BTreeMap::new(),
            texture_cubes: BTreeMap::new(),
            samplers: BTreeMap::new(),
            lights: BTreeMap::new(),
            cameras: BTreeMap::new(),
            morph_targets: BTreeMap::new(),
            mesh_skin_attributes: BTreeMap::new(),
            skins: BTreeMap::new(),
            animations: BTreeMap::new(),
            texture_transforms: BTreeMap::new(),
            material_variants: Vec::new(),
            dependency_graph: AssetDependencyGraph::new(),
            diagnostics: Vec::new(),
        }
    }

    /// 将稳定的 v1 glTF 资产转换为包。
    pub fn from_gltf_asset(id: AssetId, label: impl Into<String>, asset: GltfAsset) -> Self {
        let loaded_materials = asset
            .materials
            .iter()
            .map(|(id, material)| (*id, LoadedMaterial::Pbr(material.clone())))
            .collect();
        Self {
            id,
            label: label.into(),
            source: None,
            scene: asset.scene,
            meshes: asset.meshes,
            materials: asset.materials,
            loaded_materials,
            textures: asset.textures,
            texture_cubes: BTreeMap::new(),
            samplers: asset.samplers,
            lights: asset.lights,
            cameras: asset.cameras,
            morph_targets: BTreeMap::new(),
            mesh_skin_attributes: BTreeMap::new(),
            skins: BTreeMap::new(),
            animations: BTreeMap::new(),
            texture_transforms: BTreeMap::new(),
            material_variants: Vec::new(),
            dependency_graph: AssetDependencyGraph::new(),
            diagnostics: Vec::new(),
        }
    }

    /// 将此包转换回稳定的 glTF 资产形态。
    pub fn into_gltf_asset(self) -> GltfAsset {
        GltfAsset {
            scene: self.scene,
            meshes: self.meshes,
            materials: self.materials,
            textures: self.textures,
            samplers: self.samplers,
            lights: self.lights,
            cameras: self.cameras,
        }
    }

    /// 返回场景图。
    #[inline]
    pub const fn scene(&self) -> &SceneGraph {
        &self.scene
    }

    /// 返回此包是否包含不支持功能的诊断。
    #[inline]
    pub fn has_unsupported_features(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == AssetDiagnosticSeverity::Unsupported)
    }

    /// 返回包拥有的资产缓冲区占用的大致 CPU 内存。
    pub fn memory_bytes(&self) -> usize {
        let meshes = self.meshes.values().map(geometry_bytes).sum::<usize>();
        let morphs = self
            .morph_targets
            .values()
            .flatten()
            .map(morph_bytes)
            .sum::<usize>();
        let textures = self
            .textures
            .values()
            .map(|texture| texture.data.len())
            .sum::<usize>();
        let cubes = self
            .texture_cubes
            .values()
            .flat_map(|cube| cube.faces.iter())
            .map(Vec::len)
            .sum::<usize>();
        meshes + morphs + textures + cubes + self.dependency_graph.total_bytes()
    }

    /// 添加诊断并返回包以支持链式调用。
    #[inline]
    pub fn with_diagnostic(mut self, diagnostic: AssetDiagnostic) -> Self {
        self.diagnostics.push(diagnostic);
        self
    }
}

/// 由 [`AssetManager`](crate::AssetManager) 返回的共享包句柄。
pub type SharedAssetPackage = Arc<AssetPackage>;

/// 异步加载状态。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssetLoadStatus {
    /// 已排队但未开始。
    Pending,
    /// 进行中，尽力报告 `0.0..=1.0` 进度。
    Loading { progress: f32 },
    /// 已成功完成。
    Loaded,
    /// 被调用者取消。
    Cancelled,
    /// 因 scenekit 错误而失败。
    Failed(ScenixError),
}

pub(crate) struct AsyncAssetState {
    pub status: AssetLoadStatus,
    pub package: Option<SharedAssetPackage>,
    pub cancel_requested: bool,
}

/// 后台资产加载的句柄。
#[derive(Clone)]
pub struct AssetLoadHandle {
    id: AssetId,
    pub(crate) state: Arc<Mutex<AsyncAssetState>>,
}

impl AssetLoadHandle {
    pub(crate) fn new(id: AssetId, state: Arc<Mutex<AsyncAssetState>>) -> Self {
        Self { id, state }
    }

    /// 返回此加载的 ID。
    #[inline]
    pub const fn id(&self) -> AssetId {
        self.id
    }

    /// 请求取消。
    pub fn cancel(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.cancel_requested = true;
            state.status = AssetLoadStatus::Cancelled;
        }
    }

    /// 返回当前状态。
    pub fn status(&self) -> AssetLoadStatus {
        self.state
            .lock()
            .map(|state| state.status.clone())
            .unwrap_or(AssetLoadStatus::Failed(ScenixError::Load(
                scenekit_core::LoadError::Io,
            )))
    }

    /// 如果加载已成功完成，返回已完成的包。
    pub fn package(&self) -> Option<SharedAssetPackage> {
        self.state
            .lock()
            .ok()
            .and_then(|state| state.package.as_ref().map(Arc::clone))
    }
}

/// 文件系列的加载器支持级别。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssetFormatSupport {
    /// 完全解码为 scenekit CPU 数据。
    Full,
    /// 解码元数据或聚焦的子集。
    Partial,
    /// 已识别但此构建未解码。
    DiagnosticOnly,
}

/// 资产格式支持矩阵中的一行。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AssetFormatInfo {
    /// 人类可读的格式系列名称。
    pub name: &'static str,
    /// 常见的文件扩展名。
    pub extensions: &'static [&'static str],
    /// 支持级别。
    pub support: AssetFormatSupport,
}

/// v1.3 资产管线支持矩阵。
pub const ASSET_FORMATS: &[AssetFormatInfo] = &[
    AssetFormatInfo {
        name: "glTF / GLB",
        extensions: &["gltf", "glb"],
        support: AssetFormatSupport::Full,
    },
    AssetFormatInfo {
        name: "OBJ / MTL",
        extensions: &["obj"],
        support: AssetFormatSupport::Full,
    },
    AssetFormatInfo {
        name: "STL",
        extensions: &["stl"],
        support: AssetFormatSupport::Full,
    },
    AssetFormatInfo {
        name: "Images",
        extensions: &["png", "jpg", "jpeg", "webp", "tga", "tif", "tiff", "exr"],
        support: AssetFormatSupport::Full,
    },
    AssetFormatInfo {
        name: "KTX2 / DDS / HDR",
        extensions: &["ktx2", "dds", "hdr"],
        support: AssetFormatSupport::Partial,
    },
    AssetFormatInfo {
        name: "PLY / VOX / SVG / IES / LUT",
        extensions: &["ply", "vox", "svg", "ies", "cube", "3dl"],
        support: AssetFormatSupport::Partial,
    },
    AssetFormatInfo {
        name: "Collada / 3MF / VTK / LDraw / TTF",
        extensions: &["dae", "3mf", "vtk", "ldr", "dat", "ttf", "otf"],
        support: AssetFormatSupport::DiagnosticOnly,
    },
    AssetFormatInfo {
        name: "FBX / USD / USDZ / Rhino / UltraHDR",
        extensions: &["fbx", "usd", "usdz", "3dm", "uhdr"],
        support: AssetFormatSupport::DiagnosticOnly,
    },
];

/// 返回路径扩展名的支持信息。
pub fn support_for_extension(extension: &str) -> Option<AssetFormatInfo> {
    let normalized = extension.trim_start_matches('.').to_ascii_lowercase();
    ASSET_FORMATS
        .iter()
        .copied()
        .find(|info| info.extensions.iter().any(|ext| *ext == normalized))
}

fn geometry_bytes(geometry: &Geometry) -> usize {
    geometry.positions.len() * core::mem::size_of::<Vec3>()
        + geometry.normals.len() * core::mem::size_of::<Vec3>()
        + geometry.uvs.len() * core::mem::size_of::<Vec2>()
        + geometry.uvs2.len() * core::mem::size_of::<Vec2>()
        + geometry.colors.len() * core::mem::size_of::<scenekit_core::Color>()
        + geometry.indices.len() * core::mem::size_of::<u32>()
        + geometry.tangents.len() * core::mem::size_of::<scenekit_math::Vec4>()
}

fn morph_bytes(morph: &MorphTarget) -> usize {
    morph.positions_delta.len() * core::mem::size_of::<Vec3>()
        + morph.normals_delta.len() * core::mem::size_of::<Vec3>()
}
