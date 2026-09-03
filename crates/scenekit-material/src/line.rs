use scenekit_core::Color;

use crate::{
    AlphaMode, FEATURE_DASHED, FEATURE_WORLD_SPACE, Material, PipelineAlphaMode, PipelineKey,
    ShaderKind,
};

/// 具有可选虚线图案的线段材质。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LineMaterial {
    /// 线性 RGBA 线段颜色。
    pub color: Color,
    /// 逻辑像素线宽，除非 `world_units` 为 true。
    pub width: f32,
    /// 虚线长度。`0.0` 禁用虚线渲染。
    pub dash_size: f32,
    /// 虚线之间的间隔长度。
    pub gap_size: f32,
    /// 将宽度解释为世界单位而非逻辑像素。
    pub world_units: bool,
    /// alpha 行为。
    pub alpha_mode: AlphaMode,
}

impl LineMaterial {
    /// 创建默认的一像素白色线段材质。
    #[inline]
    pub const fn new() -> Self {
        Self {
            color: Color::WHITE,
            width: 1.0,
            dash_size: 0.0,
            gap_size: 0.0,
            world_units: false,
            alpha_mode: AlphaMode::Opaque,
        }
    }

    /// 返回设置虚线图案的此材质。
    #[inline]
    pub fn dashed(mut self, dash_size: f32, gap_size: f32) -> Self {
        self.dash_size = dash_size.max(0.0);
        self.gap_size = gap_size.max(0.0);
        self
    }
}

impl Default for LineMaterial {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Material for LineMaterial {
    #[inline]
    fn pipeline_key(&self) -> PipelineKey {
        let mut bits = 0;
        if self.dash_size > 0.0 && self.gap_size > 0.0 {
            bits |= FEATURE_DASHED;
        }
        if self.world_units {
            bits |= FEATURE_WORLD_SPACE;
        }
        let alpha = if self.color.a < 1.0 {
            PipelineAlphaMode::Blend
        } else {
            self.alpha_mode.pipeline_alpha()
        };
        PipelineKey::new(ShaderKind::Line, alpha, bits)
    }

    #[inline]
    fn is_transparent(&self) -> bool {
        self.alpha_mode.is_transparent() || self.color.a < 1.0
    }

    #[inline]
    fn double_sided(&self) -> bool {
        true
    }

    #[inline]
    fn alpha_cutoff(&self) -> Option<f32> {
        self.alpha_mode.cutoff()
    }
}
