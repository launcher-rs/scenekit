use scenekit_core::Color;

use crate::positive;

/// 矩形面光源描述。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreaLight {
    /// 发射器宽度（世界单位）。
    pub width: f32,
    /// 发射器高度（世界单位）。
    pub height: f32,
    /// 线性 RGB 光照颜色。
    pub color: Color,
    /// 标量强度。
    pub intensity: f32,
}

impl AreaLight {
    /// 创建面光源。
    #[inline]
    pub fn new(width: f32, height: f32, color: Color, intensity: f32) -> Self {
        Self {
            width: positive(width, 1.0),
            height: positive(height, 1.0),
            color,
            intensity,
        }
    }
}

impl Default for AreaLight {
    #[inline]
    fn default() -> Self {
        Self::new(1.0, 1.0, Color::WHITE, 1.0)
    }
}
