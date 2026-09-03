use scenekit_core::Color;

use crate::{ShadowConfig, positive};

/// 全向点光源。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointLight {
    /// 线性 RGB 光照颜色。
    pub color: Color,
    /// 标量强度。
    pub intensity: f32,
    /// 最大影响距离。`0.0` 表示无界。
    pub range: f32,
    /// 距离衰减指数。基于物理的平方反比衰减为 `2.0`。
    pub decay: f32,
    /// 可选的阴影配置。
    pub shadow: Option<ShadowConfig>,
}

impl PointLight {
    /// 创建点光源。
    #[inline]
    pub fn new(color: Color, intensity: f32, range: f32) -> Self {
        Self {
            color,
            intensity,
            range: range.max(0.0),
            decay: 2.0,
            shadow: None,
        }
    }

    /// 返回设置距离衰减指数的此光源。
    #[inline]
    pub fn decay(mut self, decay: f32) -> Self {
        self.decay = positive(decay, 2.0);
        self
    }

    /// 返回设置阴影配置的此光源。
    #[inline]
    pub const fn shadow(mut self, shadow: ShadowConfig) -> Self {
        self.shadow = Some(shadow);
        self
    }
}

impl Default for PointLight {
    #[inline]
    fn default() -> Self {
        Self::new(Color::WHITE, 1.0, 0.0)
    }
}
