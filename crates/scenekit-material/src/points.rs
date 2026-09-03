use scenekit_core::Color;

use crate::{
    AlphaMode, FEATURE_SIZE_ATTENUATION, Material, PipelineAlphaMode, PipelineKey, ShaderKind,
};

/// 用于点列表几何体的点材质。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointsMaterial {
    /// 线性 RGBA 点颜色。
    pub color: Color,
    /// 逻辑像素点大小。
    pub size: f32,
    /// 点大小是否随深度衰减。
    pub size_attenuation: bool,
    /// alpha 行为。
    pub alpha_mode: AlphaMode,
}

impl PointsMaterial {
    /// 创建默认的白色点材质。
    #[inline]
    pub const fn new() -> Self {
        Self {
            color: Color::WHITE,
            size: 1.0,
            size_attenuation: true,
            alpha_mode: AlphaMode::Opaque,
        }
    }
}

impl Default for PointsMaterial {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Material for PointsMaterial {
    #[inline]
    fn pipeline_key(&self) -> PipelineKey {
        let bits = if self.size_attenuation {
            FEATURE_SIZE_ATTENUATION
        } else {
            0
        };
        let alpha = if self.color.a < 1.0 {
            PipelineAlphaMode::Blend
        } else {
            self.alpha_mode.pipeline_alpha()
        };
        PipelineKey::new(ShaderKind::Points, alpha, bits)
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
