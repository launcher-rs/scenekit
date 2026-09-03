use scenekit_core::Color;

use crate::{ShadowConfig, clamp01};

/// 锥形聚光灯。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpotLight {
    /// 线性 RGB 光照颜色。
    pub color: Color,
    /// 标量强度。
    pub intensity: f32,
    /// 最大影响距离。`0.0` 表示无界。
    pub range: f32,
    /// 外锥半角（弧度）。
    pub angle: f32,
    /// 柔边比例，范围 `0.0..=1.0`。
    pub penumbra: f32,
    /// 可选的阴影配置。
    pub shadow: Option<ShadowConfig>,
}

impl SpotLight {
    /// 创建聚光灯。
    #[inline]
    pub fn new(color: Color, intensity: f32, range: f32, angle: f32) -> Self {
        Self {
            color,
            intensity,
            range: range.max(0.0),
            angle: angle.max(0.0),
            penumbra: 0.0,
            shadow: None,
        }
    }

    /// 返回设置柔边比例的此光源。
    #[inline]
    pub fn penumbra(mut self, penumbra: f32) -> Self {
        self.penumbra = clamp01(penumbra);
        self
    }

    /// 返回设置阴影配置的此光源。
    #[inline]
    pub const fn shadow(mut self, shadow: ShadowConfig) -> Self {
        self.shadow = Some(shadow);
        self
    }
}

impl Default for SpotLight {
    #[inline]
    fn default() -> Self {
        Self::new(Color::WHITE, 1.0, 0.0, core::f32::consts::FRAC_PI_4)
    }
}
