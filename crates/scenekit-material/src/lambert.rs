use alloc::string::String;

use scenekit_core::{Color, TextureId};
use scenekit_math::Vec3;

use crate::traits::{double_sided_bit, option_texture_bit};
use crate::{AlphaMode, FEATURE_ALBEDO_TEXTURE, Material, PipelineKey, ShaderKind};

/// 用于快速光照表面的仅漫反射材质。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambertMaterial {
    /// 人类可读的材质名称。
    pub name: String,
    /// 线性 RGBA 漫反射颜色。
    pub color: Color,
    /// 可选的漫反射颜色纹理。
    pub color_texture: Option<TextureId>,
    /// 线性空间中的自发光 RGB 颜色。
    pub emissive: Vec3,
    /// alpha 行为。
    pub alpha_mode: AlphaMode,
    /// 材质是否为双面渲染。
    pub double_sided: bool,
}

impl LambertMaterial {
    /// 创建默认的不透明白色 Lambert 材质。
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 返回设置漫反射颜色的此材质。
    #[inline]
    pub const fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Default for LambertMaterial {
    #[inline]
    fn default() -> Self {
        Self {
            name: String::new(),
            color: Color::WHITE,
            color_texture: None,
            emissive: Vec3::ZERO,
            alpha_mode: AlphaMode::Opaque,
            double_sided: false,
        }
    }
}

impl Material for LambertMaterial {
    #[inline]
    fn pipeline_key(&self) -> PipelineKey {
        PipelineKey::new(
            ShaderKind::Lambert,
            self.alpha_mode.pipeline_alpha(),
            double_sided_bit(self.double_sided)
                | option_texture_bit(&self.color_texture, FEATURE_ALBEDO_TEXTURE),
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
