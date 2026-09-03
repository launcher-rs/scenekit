use alloc::string::String;

use scenekit_core::{Color, TextureId};

use crate::traits::{double_sided_bit, option_texture_bit};
use crate::{
    AlphaMode, FEATURE_ALBEDO_TEXTURE, FEATURE_GRADIENT_TEXTURE, FEATURE_OUTLINE, Material,
    PipelineKey, ShaderKind,
};

/// 具有离散光照带的赛璐璐着色材质。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ToonMaterial {
    /// 人类可读的材质名称。
    pub name: String,
    /// 线性 RGBA 基础颜色。
    pub color: Color,
    /// 可选的基础颜色纹理。
    pub color_texture: Option<TextureId>,
    /// 可选的一维渐变/色阶纹理。
    pub gradient_map: Option<TextureId>,
    /// 无渐变贴图时的回退离散色带数量。
    pub steps: u32,
    /// 描边宽度。`0.0` 禁用描边路径。
    pub outline_width: f32,
    /// 线性 RGBA 描边颜色。
    pub outline_color: Color,
    /// alpha 行为。
    pub alpha_mode: AlphaMode,
    /// 材质是否为双面渲染。
    pub double_sided: bool,
}

impl ToonMaterial {
    /// 创建默认的卡通材质。
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 返回设置回退色带数量的此材质。
    #[inline]
    pub const fn steps(mut self, steps: u32) -> Self {
        self.steps = steps;
        self
    }

    /// 返回设置描边参数的此材质。
    #[inline]
    pub fn outline(mut self, width: f32, color: Color) -> Self {
        self.outline_width = width.max(0.0);
        self.outline_color = color;
        self
    }
}

impl Default for ToonMaterial {
    #[inline]
    fn default() -> Self {
        Self {
            name: String::new(),
            color: Color::WHITE,
            color_texture: None,
            gradient_map: None,
            steps: 4,
            outline_width: 0.0,
            outline_color: Color::BLACK,
            alpha_mode: AlphaMode::Opaque,
            double_sided: false,
        }
    }
}

impl Material for ToonMaterial {
    #[inline]
    fn pipeline_key(&self) -> PipelineKey {
        let mut bits = double_sided_bit(self.double_sided)
            | option_texture_bit(&self.color_texture, FEATURE_ALBEDO_TEXTURE)
            | option_texture_bit(&self.gradient_map, FEATURE_GRADIENT_TEXTURE);
        if self.outline_width > 0.0 {
            bits |= FEATURE_OUTLINE;
        }
        PipelineKey::new(ShaderKind::Toon, self.alpha_mode.pipeline_alpha(), bits)
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
