use alloc::string::String;

use scenekit_core::{Color, TextureId};

use crate::traits::{double_sided_bit, option_texture_bit};
use crate::{AlphaMode, FEATURE_ALBEDO_TEXTURE, Material, PipelineKey, ShaderKind};

/// 忽略场景光照的恒定色材质。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnlitMaterial {
    /// 人类可读的材质名称。
    pub name: String,
    /// 线性 RGBA 颜色。
    pub color: Color,
    /// 可选的颜色纹理。
    pub color_texture: Option<TextureId>,
    /// alpha 行为。
    pub alpha_mode: AlphaMode,
    /// 材质是否为双面渲染。
    pub double_sided: bool,
}

impl UnlitMaterial {
    /// 创建默认的不透明白色无光照材质。
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 返回设置颜色的此材质。
    #[inline]
    pub const fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// 返回设置 alpha 行为的此材质。
    #[inline]
    pub const fn alpha_mode(mut self, alpha_mode: AlphaMode) -> Self {
        self.alpha_mode = alpha_mode;
        self
    }
}

impl Default for UnlitMaterial {
    #[inline]
    fn default() -> Self {
        Self {
            name: String::new(),
            color: Color::WHITE,
            color_texture: None,
            alpha_mode: AlphaMode::Opaque,
            double_sided: false,
        }
    }
}

impl Material for UnlitMaterial {
    #[inline]
    fn pipeline_key(&self) -> PipelineKey {
        PipelineKey::new(
            ShaderKind::Unlit,
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
