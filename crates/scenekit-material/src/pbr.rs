use alloc::string::String;

use scenekit_core::{Color, TextureId};
use scenekit_math::Vec3;

use crate::traits::{
    AlphaMode, FEATURE_ALBEDO_TEXTURE, FEATURE_EMISSIVE_TEXTURE,
    FEATURE_METALLIC_ROUGHNESS_TEXTURE, FEATURE_NORMAL_TEXTURE, FEATURE_OCCLUSION_TEXTURE,
    Material, PipelineKey, ShaderKind, double_sided_bit, option_texture_bit,
};

/// 金属度粗糙度基于物理的材质。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PbrMaterial {
    /// 人类可读的材质名称。
    pub name: String,
    /// 线性 RGBA 基础颜色。
    pub albedo: Color,
    /// 可选的基础颜色纹理。
    pub albedo_texture: Option<TextureId>,
    /// 金属度因子，0 为介电质，1 为金属。
    pub metallic: f32,
    /// 粗糙度因子，0 为镜面反射，1 为哑光。
    pub roughness: f32,
    /// 可选的打包金属度粗糙度纹理。
    pub metallic_roughness_texture: Option<TextureId>,
    /// 可选的切线空间法线贴图。
    pub normal_texture: Option<TextureId>,
    /// 可选的环境遮挡纹理。
    pub occlusion_texture: Option<TextureId>,
    /// 线性空间中的自发光 RGB 颜色。
    pub emissive: Vec3,
    /// 可选的自发光纹理。
    pub emissive_texture: Option<TextureId>,
    /// alpha 行为。
    pub alpha_mode: AlphaMode,
    /// 材质是否为双面渲染。
    pub double_sided: bool,
}

impl PbrMaterial {
    /// 创建默认的不透明白色介电材质。
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 返回设置名称的此材质。
    #[inline]
    pub fn named(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 返回设置基础颜色的此材质。
    #[inline]
    pub const fn albedo(mut self, albedo: Color) -> Self {
        self.albedo = albedo;
        self
    }

    /// 返回设置金属度和粗糙度因子的此材质。
    #[inline]
    pub fn metallic_roughness(mut self, metallic: f32, roughness: f32) -> Self {
        self.metallic = metallic.clamp(0.0, 1.0);
        self.roughness = roughness.clamp(0.0, 1.0);
        self
    }

    /// 返回设置 alpha 行为的此材质。
    #[inline]
    pub const fn alpha_mode(mut self, alpha_mode: AlphaMode) -> Self {
        self.alpha_mode = alpha_mode;
        self
    }

    /// 返回启用或禁用双面渲染的此材质。
    #[inline]
    pub const fn double_sided(mut self, double_sided: bool) -> Self {
        self.double_sided = double_sided;
        self
    }

    pub(crate) fn feature_bits(&self) -> u64 {
        double_sided_bit(self.double_sided)
            | option_texture_bit(&self.albedo_texture, FEATURE_ALBEDO_TEXTURE)
            | option_texture_bit(
                &self.metallic_roughness_texture,
                FEATURE_METALLIC_ROUGHNESS_TEXTURE,
            )
            | option_texture_bit(&self.normal_texture, FEATURE_NORMAL_TEXTURE)
            | option_texture_bit(&self.occlusion_texture, FEATURE_OCCLUSION_TEXTURE)
            | option_texture_bit(&self.emissive_texture, FEATURE_EMISSIVE_TEXTURE)
    }
}

impl Default for PbrMaterial {
    #[inline]
    fn default() -> Self {
        Self {
            name: String::new(),
            albedo: Color::WHITE,
            albedo_texture: None,
            metallic: 0.0,
            roughness: 1.0,
            metallic_roughness_texture: None,
            normal_texture: None,
            occlusion_texture: None,
            emissive: Vec3::ZERO,
            emissive_texture: None,
            alpha_mode: AlphaMode::Opaque,
            double_sided: false,
        }
    }
}

impl Material for PbrMaterial {
    #[inline]
    fn pipeline_key(&self) -> PipelineKey {
        PipelineKey::new(
            ShaderKind::Pbr,
            self.alpha_mode.pipeline_alpha(),
            self.feature_bits(),
        )
    }

    #[inline]
    fn is_transparent(&self) -> bool {
        self.alpha_mode.is_transparent()
    }

    #[inline]
    fn double_sided(&self) -> bool {
        self.double_sided
    }

    #[inline]
    fn alpha_cutoff(&self) -> Option<f32> {
        self.alpha_mode.cutoff()
    }
}
