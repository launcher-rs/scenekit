use std::collections::HashMap;

use scenekit_core::{Color, LightId, MaterialId, MeshId, TextureId, ValidationError};
use scenekit_light::{
    AmbientLight, AreaLight, DirectionalLight, HemisphereLight, LightProbe, PointLight, SpotLight,
};
use scenekit_material::{
    LambertMaterial, Material, NormalMaterial, PbrMaterial, PhysicalMaterial, PipelineKey,
    ToonMaterial, UnlitMaterial, WireframeMaterial,
};
use scenekit_math::{Aabb, Mat4, Vec2, Vec3, Vec4};
use scenekit_mesh::Geometry;
use scenekit_texture::{
    AddressMode, CompareFunction, FilterMode, Sampler, Texture2D, Texture3D, TextureCube,
    TextureFormat,
};
use wgpu::util::DeviceExt;

/// v0.6 渲染器使用的交错 GPU 顶点布局。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PackedVertex {
    /// 顶点位置。
    pub position: [f32; 3],
    /// 顶点法线。
    pub normal: [f32; 3],
    /// 主纹理坐标。
    pub uv: [f32; 2],
    /// 顶点颜色。
    pub color: [f32; 4],
    /// 切线向量和手性。
    pub tangent: [f32; 4],
}

impl PackedVertex {
    /// 返回 wgpu 顶点缓冲区布局。
    pub const fn layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x3,
            2 => Float32x2,
            3 => Float32x4,
            4 => Float32x4
        ];
        wgpu::VertexBufferLayout {
            array_stride: core::mem::size_of::<PackedVertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

/// 打包几何体选择的索引类型。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GpuIndexFormat {
    /// 16 位索引缓冲区。
    Uint16,
    /// 32 位索引缓冲区。
    Uint32,
}

impl GpuIndexFormat {
    /// 返回匹配的 wgpu 索引格式。
    #[inline]
    pub const fn to_wgpu(self) -> wgpu::IndexFormat {
        match self {
            Self::Uint16 => wgpu::IndexFormat::Uint16,
            Self::Uint32 => wgpu::IndexFormat::Uint32,
        }
    }
}

/// CPU 端打包的几何体，准备上传到 GPU。
#[derive(Clone, Debug, PartialEq)]
pub struct PackedGeometry {
    /// 交错顶点。
    pub vertices: Vec<PackedVertex>,
    /// `index_format` 格式的原始索引字节。
    pub index_bytes: Vec<u8>,
    /// 索引数量。
    pub index_count: u32,
    /// 索引存储格式。
    pub index_format: GpuIndexFormat,
    /// 局部空间几何体边界。
    pub aabb: Aabb,
}

/// 已上传的 GPU 网格缓冲区。
#[derive(Debug)]
pub struct GpuMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    packed: PackedGeometry,
}

impl GpuMesh {
    /// 返回顶点缓冲区。
    #[inline]
    pub const fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    /// 返回索引缓冲区。
    #[inline]
    pub const fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    /// 返回打包几何体元数据。
    #[inline]
    pub const fn packed(&self) -> &PackedGeometry {
        &self.packed
    }
}

/// 渲染器端的材质注册表条目。
#[derive(Clone, Debug, PartialEq)]
pub enum RendererMaterial {
    /// 金属-粗糙度材质。
    Pbr(PbrMaterial),
    /// 高级物理材质。
    Physical(PhysicalMaterial),
    /// 恒定颜色无光照材质。
    Unlit(UnlitMaterial),
    /// 漫反射 Lambert 材质。
    Lambert(LambertMaterial),
    /// 卡通着色材质。
    Toon(ToonMaterial),
    /// 线框/调试预览材质。
    Wireframe(WireframeMaterial),
    /// 法线可视化材质。
    Normal(NormalMaterial),
}

impl RendererMaterial {
    /// 返回材质管线键。
    #[inline]
    pub fn pipeline_key(&self) -> PipelineKey {
        match self {
            Self::Pbr(material) => material.pipeline_key(),
            Self::Physical(material) => material.pipeline_key(),
            Self::Unlit(material) => material.pipeline_key(),
            Self::Lambert(material) => material.pipeline_key(),
            Self::Toon(material) => material.pipeline_key(),
            Self::Wireframe(material) => material.pipeline_key(),
            Self::Normal(material) => material.pipeline_key(),
        }
    }

    /// 返回材质是否应与透明绘制进行深度排序。
    #[inline]
    pub fn is_transparent(&self) -> bool {
        match self {
            Self::Pbr(material) => material.is_transparent(),
            Self::Physical(material) => material.is_transparent(),
            Self::Unlit(material) => material.is_transparent(),
            Self::Lambert(material) => material.is_transparent(),
            Self::Toon(material) => material.is_transparent(),
            Self::Wireframe(material) => material.is_transparent(),
            Self::Normal(material) => material.is_transparent(),
        }
    }

    /// 返回稳定 v1 渲染器路径使用的基色预览颜色。
    #[inline]
    pub fn preview_color(&self) -> Color {
        match self {
            Self::Pbr(material) => material.albedo,
            Self::Physical(material) => material.base.albedo,
            Self::Unlit(material) => material.color,
            Self::Lambert(material) => material.color,
            Self::Toon(material) => material.color,
            Self::Wireframe(material) => Color {
                a: material.opacity,
                ..material.color
            },
            Self::Normal(_) => Color::WHITE,
        }
    }

    /// 返回共享 v1 预览着色器的紧凑着色器族代码。
    #[inline]
    pub fn preview_shader_code(&self) -> f32 {
        match self {
            Self::Pbr(_) => 0.0,
            Self::Physical(_) => 1.0,
            Self::Unlit(_) => 2.0,
            Self::Lambert(_) => 3.0,
            Self::Toon(_) => 4.0,
            Self::Wireframe(_) => 5.0,
            Self::Normal(_) => 6.0,
        }
    }
}

/// 渲染器端的纹理元数据。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GpuTexture {
    /// 纹理宽度。
    pub width: u32,
    /// 纹理高度。
    pub height: u32,
    /// CPU 纹理格式。
    pub format: TextureFormat,
    /// 支持时的匹配 wgpu 纹理格式。
    pub wgpu_format: Option<wgpu::TextureFormat>,
    /// 采样器元数据。
    pub sampler: Sampler,
    /// CPU 纹理中存储的 mip 级别数。
    pub mip_levels: u32,
}

/// `GpuMaterial` 使用的纹理元数据存储。
pub type TextureStore = HashMap<TextureId, GpuTexture>;

/// 渲染器端的光源注册表条目。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RendererLight {
    /// 环境光。
    Ambient(AmbientLight),
    /// 方向光。
    Directional(DirectionalLight),
    /// 点光源。
    Point(PointLight),
    /// 聚光灯。
    Spot(SpotLight),
    /// 半球渐变光。
    Hemisphere(HemisphereLight),
    /// 矩形区域光。
    Area(AreaLight),
    /// 球谐函数光照探针。
    Probe(LightProbe),
}

/// 从场景节点生成的可见绘制提交。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawSubmission {
    /// 网格资源 ID。
    pub mesh_id: MeshId,
    /// 材质资源 ID。
    pub material_id: MaterialId,
    /// 节点世界变换矩阵。
    pub world_matrix: Mat4,
    /// 用于剔除的世界空间边界。
    pub world_aabb: Aabb,
    /// 从相机位置到边界中心的距离。
    pub distance_to_camera: f32,
    /// 此绘制是否需要透明排序。
    pub transparent: bool,
    /// 稳定的渲染顺序。
    pub render_order: u32,
}

/// 渲染器拥有的 GPU 场景资源和 CPU 元数据。
#[derive(Debug, Default)]
pub struct GpuScene {
    meshes: HashMap<MeshId, GpuMesh>,
    materials: HashMap<MaterialId, RendererMaterial>,
    textures: TextureStore,
    lights: HashMap<LightId, RendererLight>,
}

impl GpuScene {
    /// 创建空的 GPU 场景注册表。
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 将几何体打包为渲染器交错顶点/索引布局。
    pub fn pack_geometry(geometry: &Geometry) -> Result<PackedGeometry, ValidationError> {
        geometry.validate()?;
        let vertex_count = geometry.positions.len();
        let mut vertices = Vec::with_capacity(vertex_count);
        for index in 0..vertex_count {
            let position = geometry.positions[index];
            let normal = geometry.normals.get(index).copied().unwrap_or(Vec3::Y);
            let uv = geometry.uvs.get(index).copied().unwrap_or(Vec2::ZERO);
            let color = geometry.colors.get(index).copied().unwrap_or(Color::WHITE);
            let tangent = geometry
                .tangents
                .get(index)
                .copied()
                .unwrap_or(Vec4::new(1.0, 0.0, 0.0, 1.0));
            vertices.push(PackedVertex {
                position: [position.x, position.y, position.z],
                normal: [normal.x, normal.y, normal.z],
                uv: [uv.x, uv.y],
                color: color.to_array(),
                tangent: [tangent.x, tangent.y, tangent.z, tangent.w],
            });
        }

        let source_indices: Vec<u32> = if geometry.indices.is_empty() {
            (0..vertex_count as u32).collect()
        } else {
            geometry.indices.clone()
        };
        let can_use_u16 = vertex_count <= u16::MAX as usize
            && source_indices.iter().all(|index| *index <= u16::MAX as u32);
        let (index_bytes, index_format) = if can_use_u16 {
            let indices: Vec<u16> = source_indices.iter().map(|index| *index as u16).collect();
            (
                bytemuck::cast_slice(&indices).to_vec(),
                GpuIndexFormat::Uint16,
            )
        } else {
            (
                bytemuck::cast_slice(source_indices.as_slice()).to_vec(),
                GpuIndexFormat::Uint32,
            )
        };

        Ok(PackedGeometry {
            vertices,
            index_bytes,
            index_count: source_indices.len() as u32,
            index_format,
            aabb: geometry.aabb(),
        })
    }

    /// 上传并存储网格。
    pub fn register_mesh(
        &mut self,
        device: &wgpu::Device,
        mesh_id: MeshId,
        geometry: &Geometry,
    ) -> Result<(), ValidationError> {
        if mesh_id.is_null() {
            return Err(ValidationError::InvalidId);
        }
        let packed = Self::pack_geometry(geometry)?;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("scenekit.mesh.vertices"),
            contents: bytemuck::cast_slice(packed.vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("scenekit.mesh.indices"),
            contents: packed.index_bytes.as_slice(),
            usage: wgpu::BufferUsages::INDEX,
        });
        self.meshes.insert(
            mesh_id,
            GpuMesh {
                vertex_buffer,
                index_buffer,
                packed,
            },
        );
        Ok(())
    }

    /// 注册 PBR 材质。
    pub fn register_pbr_material(
        &mut self,
        material_id: MaterialId,
        material: &PbrMaterial,
    ) -> Result<(), ValidationError> {
        self.register_material(material_id, RendererMaterial::Pbr(material.clone()))
    }

    /// 注册物理材质。
    pub fn register_physical_material(
        &mut self,
        material_id: MaterialId,
        material: &PhysicalMaterial,
    ) -> Result<(), ValidationError> {
        self.register_material(material_id, RendererMaterial::Physical(material.clone()))
    }

    /// 注册无光照材质。
    pub fn register_unlit_material(
        &mut self,
        material_id: MaterialId,
        material: &UnlitMaterial,
    ) -> Result<(), ValidationError> {
        self.register_material(material_id, RendererMaterial::Unlit(material.clone()))
    }

    /// 注册 Lambert 材质。
    pub fn register_lambert_material(
        &mut self,
        material_id: MaterialId,
        material: &LambertMaterial,
    ) -> Result<(), ValidationError> {
        self.register_material(material_id, RendererMaterial::Lambert(material.clone()))
    }

    /// 注册卡通材质。
    pub fn register_toon_material(
        &mut self,
        material_id: MaterialId,
        material: &ToonMaterial,
    ) -> Result<(), ValidationError> {
        self.register_material(material_id, RendererMaterial::Toon(material.clone()))
    }

    /// 注册线框预览材质。
    pub fn register_wireframe_material(
        &mut self,
        material_id: MaterialId,
        material: &WireframeMaterial,
    ) -> Result<(), ValidationError> {
        self.register_material(material_id, RendererMaterial::Wireframe(*material))
    }

    /// 注册法线可视化材质。
    pub fn register_normal_material(
        &mut self,
        material_id: MaterialId,
        material: &NormalMaterial,
    ) -> Result<(), ValidationError> {
        self.register_material(material_id, RendererMaterial::Normal(*material))
    }

    /// 注册渲染器材质。
    pub fn register_material(
        &mut self,
        material_id: MaterialId,
        material: RendererMaterial,
    ) -> Result<(), ValidationError> {
        if material_id.is_null() {
            return Err(ValidationError::InvalidId);
        }
        self.materials.insert(material_id, material);
        Ok(())
    }

    /// 注册已验证的 2D 纹理元数据和采样器状态。
    pub fn register_texture2d(
        &mut self,
        texture_id: TextureId,
        texture: &Texture2D,
        sampler: Sampler,
    ) -> Result<(), ValidationError> {
        if texture_id.is_null() {
            return Err(ValidationError::InvalidId);
        }
        texture.validate()?;
        self.textures.insert(
            texture_id,
            GpuTexture {
                width: texture.width,
                height: texture.height,
                format: texture.format,
                wgpu_format: to_wgpu_texture_format(texture.format),
                sampler,
                mip_levels: texture.mip_levels.max(1),
            },
        );
        Ok(())
    }

    /// 注册已验证的立方体纹理元数据和采样器状态。
    pub fn register_texture_cube(
        &mut self,
        texture_id: TextureId,
        texture: &TextureCube,
        sampler: Sampler,
    ) -> Result<(), ValidationError> {
        if texture_id.is_null() {
            return Err(ValidationError::InvalidId);
        }
        texture.validate()?;
        self.textures.insert(
            texture_id,
            GpuTexture {
                width: texture.size,
                height: texture.size,
                format: texture.format,
                wgpu_format: to_wgpu_texture_format(texture.format),
                sampler,
                mip_levels: texture.mip_levels.max(1),
            },
        );
        Ok(())
    }

    /// 注册已验证的 3D 纹理元数据和采样器状态。
    pub fn register_texture3d(
        &mut self,
        texture_id: TextureId,
        texture: &Texture3D,
        sampler: Sampler,
    ) -> Result<(), ValidationError> {
        if texture_id.is_null() {
            return Err(ValidationError::InvalidId);
        }
        texture.validate()?;
        self.textures.insert(
            texture_id,
            GpuTexture {
                width: texture.width,
                height: texture.height,
                format: texture.format,
                wgpu_format: to_wgpu_texture_format(texture.format),
                sampler,
                mip_levels: texture.mip_levels.max(1),
            },
        );
        Ok(())
    }

    /// 注册光源。
    pub fn register_light(
        &mut self,
        light_id: LightId,
        light: RendererLight,
    ) -> Result<(), ValidationError> {
        if light_id.is_null() {
            return Err(ValidationError::InvalidId);
        }
        self.lights.insert(light_id, light);
        Ok(())
    }

    /// 从注册表移除一个网格。
    #[inline]
    pub fn unregister_mesh(&mut self, mesh_id: MeshId) -> bool {
        self.meshes.remove(&mesh_id).is_some()
    }

    /// 从注册表移除一个材质。
    #[inline]
    pub fn unregister_material(&mut self, material_id: MaterialId) -> bool {
        self.materials.remove(&material_id).is_some()
    }

    /// 从注册表移除一个纹理元数据条目。
    #[inline]
    pub fn unregister_texture(&mut self, texture_id: TextureId) -> bool {
        self.textures.remove(&texture_id).is_some()
    }

    /// 从注册表移除一个光源。
    #[inline]
    pub fn unregister_light(&mut self, light_id: LightId) -> bool {
        self.lights.remove(&light_id).is_some()
    }

    /// 清除所有网格资源。
    #[inline]
    pub fn clear_meshes(&mut self) {
        self.meshes.clear();
    }

    /// 清除所有材质资源。
    #[inline]
    pub fn clear_materials(&mut self) {
        self.materials.clear();
    }

    /// 清除所有纹理元数据。
    #[inline]
    pub fn clear_textures(&mut self) {
        self.textures.clear();
    }

    /// 清除所有光源资源。
    #[inline]
    pub fn clear_lights(&mut self) {
        self.lights.clear();
    }

    /// 按 ID 返回网格。
    #[inline]
    pub fn mesh(&self, mesh_id: MeshId) -> Option<&GpuMesh> {
        self.meshes.get(&mesh_id)
    }

    /// 按 ID 返回材质。
    #[inline]
    pub fn material(&self, material_id: MaterialId) -> Option<&RendererMaterial> {
        self.materials.get(&material_id)
    }

    /// 按 ID 返回纹理元数据。
    #[inline]
    pub fn texture(&self, texture_id: TextureId) -> Option<&GpuTexture> {
        self.textures.get(&texture_id)
    }

    /// 返回已注册的纹理元数据。
    #[inline]
    pub const fn textures(&self) -> &TextureStore {
        &self.textures
    }

    /// 返回已注册的光源。
    #[inline]
    pub fn lights(&self) -> impl Iterator<Item = (&LightId, &RendererLight)> {
        self.lights.iter()
    }

    /// 返回已注册的光源数量。
    #[inline]
    pub fn light_count(&self) -> usize {
        self.lights.len()
    }

    /// 返回已注册的网格数量。
    #[inline]
    pub fn mesh_count(&self) -> usize {
        self.meshes.len()
    }

    /// 返回已注册的材质数量。
    #[inline]
    pub fn material_count(&self) -> usize {
        self.materials.len()
    }

    /// 返回网格顶点和索引缓冲区使用的近似 GPU 字节数。
    #[inline]
    pub fn geometry_memory_bytes(&self) -> u64 {
        self.meshes
            .values()
            .map(|mesh| {
                bytemuck::cast_slice::<PackedVertex, u8>(mesh.packed.vertices.as_slice()).len()
                    + mesh.packed.index_bytes.len()
            })
            .sum::<usize>() as u64
    }
}

/// 将 scenekit 纹理格式元数据转换为 wgpu 格式。
pub const fn to_wgpu_texture_format(format: TextureFormat) -> Option<wgpu::TextureFormat> {
    match format {
        TextureFormat::Rgba8Unorm => Some(wgpu::TextureFormat::Rgba8Unorm),
        TextureFormat::Rgba8UnormSrgb => Some(wgpu::TextureFormat::Rgba8UnormSrgb),
        TextureFormat::Rgba16Float => Some(wgpu::TextureFormat::Rgba16Float),
        TextureFormat::Depth32Float => Some(wgpu::TextureFormat::Depth32Float),
        TextureFormat::Bc7RgbaUnorm => Some(wgpu::TextureFormat::Bc7RgbaUnorm),
        TextureFormat::Astc4x4RgbaUnorm => Some(wgpu::TextureFormat::Astc {
            block: wgpu::AstcBlock::B4x4,
            channel: wgpu::AstcChannel::Unorm,
        }),
        TextureFormat::Etc2Rgba8Unorm => Some(wgpu::TextureFormat::Etc2Rgba8Unorm),
    }
}

/// 将采样器过滤模式转换为 wgpu 过滤模式。
pub const fn to_wgpu_filter_mode(filter: FilterMode) -> wgpu::FilterMode {
    match filter {
        FilterMode::Nearest => wgpu::FilterMode::Nearest,
        FilterMode::Linear => wgpu::FilterMode::Linear,
    }
}

/// 将采样器寻址模式转换为 wgpu 寻址模式。
pub const fn to_wgpu_address_mode(address: AddressMode) -> wgpu::AddressMode {
    match address {
        AddressMode::Repeat => wgpu::AddressMode::Repeat,
        AddressMode::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
        AddressMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
    }
}

/// 将可选的比较状态转换为 wgpu 比较状态。
pub const fn to_wgpu_compare(compare: Option<CompareFunction>) -> Option<wgpu::CompareFunction> {
    match compare {
        Some(CompareFunction::Less) => Some(wgpu::CompareFunction::Less),
        Some(CompareFunction::LessEqual) => Some(wgpu::CompareFunction::LessEqual),
        Some(CompareFunction::Greater) => Some(wgpu::CompareFunction::Greater),
        Some(CompareFunction::GreaterEqual) => Some(wgpu::CompareFunction::GreaterEqual),
        Some(CompareFunction::Equal) => Some(wgpu::CompareFunction::Equal),
        Some(CompareFunction::NotEqual) => Some(wgpu::CompareFunction::NotEqual),
        Some(CompareFunction::Always) => Some(wgpu::CompareFunction::Always),
        Some(CompareFunction::Never) => Some(wgpu::CompareFunction::Never),
        None => None,
    }
}
