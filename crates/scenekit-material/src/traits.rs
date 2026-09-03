/// 材质的 alpha 行为。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlphaMode {
    /// 完全不透明的材质。
    Opaque,
    /// 使用存储的裁剪阈值进行 alpha 测试的材质。
    Mask(f32),
    /// alpha 混合的材质。
    Blend,
}

impl AlphaMode {
    /// 返回管线级别的 alpha 模式。
    #[inline]
    pub const fn pipeline_alpha(self) -> PipelineAlphaMode {
        match self {
            Self::Opaque => PipelineAlphaMode::Opaque,
            Self::Mask(_) => PipelineAlphaMode::Mask,
            Self::Blend => PipelineAlphaMode::Blend,
        }
    }

    /// 返回此模式是否需要透明排序和混合。
    #[inline]
    pub const fn is_transparent(self) -> bool {
        matches!(self, Self::Blend)
    }

    /// 返回经过遮罩材质的 alpha 测试裁剪值。
    #[inline]
    pub const fn cutoff(self) -> Option<f32> {
        match self {
            Self::Mask(cutoff) => Some(cutoff),
            Self::Opaque | Self::Blend => None,
        }
    }
}

impl Default for AlphaMode {
    #[inline]
    fn default() -> Self {
        Self::Opaque
    }
}

/// 管线级别的 alpha 模式。由于裁剪值不参与渲染管线选择，
/// 因此可以实现 Hash。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PipelineAlphaMode {
    /// 完全不透明管线。
    #[default]
    Opaque,
    /// alpha 测试管线。
    Mask,
    /// alpha 混合管线。
    Blend,
}

/// 材质使用的内置着色器族。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ShaderKind {
    /// 基于物理的金属度粗糙度着色器。
    #[default]
    Pbr,
    /// 高级物理表面着色器。
    Physical,
    /// 常量色无光照着色器。
    Unlit,
    /// 漫反射 Lambert 着色器。
    Lambert,
    /// 赛璐璐/卡通着色器。
    Toon,
    /// 表面法线调试着色器。
    Normal,
    /// 线框着色器。
    Wireframe,
    /// 仅深度着色器。
    Depth,
    /// 线段着色器。
    Line,
    /// 点精灵着色器。
    Points,
    /// 用户提供的 WGSL 着色器源码，通过稳定哈希标识。
    Custom(u64),
}

/// 紧凑的渲染器管线选择器。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PipelineKey {
    /// 着色器族。
    pub shader: ShaderKind,
    /// 管线级别的 alpha 模式。
    pub alpha: PipelineAlphaMode,
    /// 材质特性标志位。
    pub feature_bits: u64,
}

impl PipelineKey {
    /// 创建管线选择器。
    #[inline]
    pub const fn new(shader: ShaderKind, alpha: PipelineAlphaMode, feature_bits: u64) -> Self {
        Self {
            shader,
            alpha,
            feature_bits,
        }
    }

    /// 返回启用一个特性标志的选择器。
    #[inline]
    pub const fn with_feature(mut self, feature: u64) -> Self {
        self.feature_bits |= feature;
        self
    }

    /// 返回指定特性标志是否已启用。
    #[inline]
    pub const fn has_feature(self, feature: u64) -> bool {
        self.feature_bits & feature != 0
    }
}

/// 材质为双面渲染。
pub const FEATURE_DOUBLE_SIDED: u64 = 1 << 0;
/// 已绑定基础颜色纹理。
pub const FEATURE_ALBEDO_TEXTURE: u64 = 1 << 1;
/// 已绑定金属度粗糙度纹理。
pub const FEATURE_METALLIC_ROUGHNESS_TEXTURE: u64 = 1 << 2;
/// 已绑定法线贴图。
pub const FEATURE_NORMAL_TEXTURE: u64 = 1 << 3;
/// 已绑定遮挡纹理。
pub const FEATURE_OCCLUSION_TEXTURE: u64 = 1 << 4;
/// 已绑定自发光纹理。
pub const FEATURE_EMISSIVE_TEXTURE: u64 = 1 << 5;
/// 已绑定渐变/色阶纹理。
pub const FEATURE_GRADIENT_TEXTURE: u64 = 1 << 6;
/// 清漆层已激活。
pub const FEATURE_CLEARCOAT: u64 = 1 << 7;
/// 光泽层已激活。
pub const FEATURE_SHEEN: u64 = 1 << 8;
/// 透射路径已激活。
pub const FEATURE_TRANSMISSION: u64 = 1 << 9;
/// 虹彩路径已激活。
pub const FEATURE_IRIDESCENCE: u64 = 1 << 10;
/// 平面法线着色已激活。
pub const FEATURE_FLAT_SHADING: u64 = 1 << 11;
/// 法线在世界空间中计算。
pub const FEATURE_WORLD_SPACE: u64 = 1 << 12;
/// 线框渲染路径已激活。
pub const FEATURE_WIREFRAME: u64 = 1 << 13;
/// 虚线路径已激活。
pub const FEATURE_DASHED: u64 = 1 << 14;
/// 点大小衰减已激活。
pub const FEATURE_SIZE_ATTENUATION: u64 = 1 << 15;
/// 自定义着色器具有纹理绑定。
pub const FEATURE_CUSTOM_TEXTURES: u64 = 1 << 16;
/// 材质期望顶点颜色。
pub const FEATURE_VERTEX_COLORS: u64 = 1 << 17;
/// 卡通描边路径已激活。
pub const FEATURE_OUTLINE: u64 = 1 << 18;

/// 无 GPU 依赖的 CPU 端材质描述。
pub trait Material: Send + Sync + 'static {
    /// 返回此材质状态的渲染器管线选择器。
    fn pipeline_key(&self) -> PipelineKey;

    /// 返回材质是否应在透明路径中渲染。
    fn is_transparent(&self) -> bool;

    /// 返回是否应禁用背面剔除。
    fn double_sided(&self) -> bool;

    /// 返回经过遮罩材质的 alpha 测试裁剪值。
    fn alpha_cutoff(&self) -> Option<f32>;
}

#[inline]
pub(crate) const fn double_sided_bit(double_sided: bool) -> u64 {
    if double_sided {
        FEATURE_DOUBLE_SIDED
    } else {
        0
    }
}

#[inline]
pub(crate) const fn option_texture_bit<T>(texture: &Option<T>, feature: u64) -> u64 {
    if texture.is_some() { feature } else { 0 }
}

/// 用于着色器源码标识的稳定 FNV-1a 哈希。
#[inline]
pub(crate) fn stable_shader_id(vertex_wgsl: &str, fragment_wgsl: &str) -> u64 {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    let mut hash = OFFSET;
    for byte in vertex_wgsl.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(PRIME);
    }
    hash ^= 0xff;
    hash = hash.wrapping_mul(PRIME);
    for byte in fragment_wgsl.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}
