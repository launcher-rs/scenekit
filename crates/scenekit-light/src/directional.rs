use scenekit_core::Color;
use scenekit_math::Vec3;

use crate::ShadowConfig;

/// 具有可选阴影配置的方向光。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirectionalLight {
    /// 光照照射的方向。
    pub direction: Vec3,
    /// 线性 RGB 光照颜色。
    pub color: Color,
    /// 标量强度。
    pub intensity: f32,
    /// 可选的阴影配置。
    pub shadow: Option<ShadowConfig>,
}

impl DirectionalLight {
    /// 创建方向光。零方向会回退到负 Z 方向。
    #[inline]
    pub fn new(direction: Vec3, color: Color, intensity: f32) -> Self {
        let direction = direction.normalize();
        Self {
            direction: if direction == Vec3::ZERO {
                Vec3::NEG_Z
            } else {
                direction
            },
            color,
            intensity,
            shadow: None,
        }
    }

    /// 返回设置阴影配置的此光源。
    #[inline]
    pub const fn shadow(mut self, shadow: ShadowConfig) -> Self {
        self.shadow = Some(shadow);
        self
    }
}

impl Default for DirectionalLight {
    #[inline]
    fn default() -> Self {
        Self::new(Vec3::NEG_Z, Color::WHITE, 1.0)
    }
}
