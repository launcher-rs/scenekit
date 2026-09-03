use scenekit_core::Color;

use crate::traits::double_sided_bit;
use crate::{FEATURE_WIREFRAME, Material, PipelineAlphaMode, PipelineKey, ShaderKind};

/// 线框叠加材质。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WireframeMaterial {
    /// 线性 RGBA 线框颜色。
    pub color: Color,
    /// 线框不透明度，范围 `0.0..=1.0`。
    pub opacity: f32,
    /// 逻辑像素线宽。
    pub line_width: f32,
    /// 材质是否为双面渲染。
    pub double_sided: bool,
}

impl WireframeMaterial {
    /// 创建默认的黑色线框材质。
    #[inline]
    pub const fn new() -> Self {
        Self {
            color: Color::BLACK,
            opacity: 1.0,
            line_width: 1.0,
            double_sided: true,
        }
    }
}

impl Default for WireframeMaterial {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Material for WireframeMaterial {
    #[inline]
    fn pipeline_key(&self) -> PipelineKey {
        let alpha = if self.opacity < 1.0 || self.color.a < 1.0 {
            PipelineAlphaMode::Blend
        } else {
            PipelineAlphaMode::Opaque
        };
        PipelineKey::new(
            ShaderKind::Wireframe,
            alpha,
            double_sided_bit(self.double_sided) | FEATURE_WIREFRAME,
        )
    }

    #[inline]
    fn is_transparent(&self) -> bool {
        self.opacity < 1.0 || self.color.a < 1.0
    }

    #[inline]
    fn double_sided(&self) -> bool {
        self.double_sided
    }

    #[inline]
    fn alpha_cutoff(&self) -> Option<f32> {
        None
    }
}
