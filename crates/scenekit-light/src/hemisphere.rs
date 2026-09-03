use scenekit_core::Color;

/// 天空/地面渐变环境光。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HemisphereLight {
    /// 线性 RGB 天空颜色。
    pub sky_color: Color,
    /// 线性 RGB 地面颜色。
    pub ground_color: Color,
    /// 标量强度。
    pub intensity: f32,
}

impl HemisphereLight {
    /// 创建半球光。
    #[inline]
    pub const fn new(sky_color: Color, ground_color: Color, intensity: f32) -> Self {
        Self {
            sky_color,
            ground_color,
            intensity,
        }
    }
}

impl Default for HemisphereLight {
    #[inline]
    fn default() -> Self {
        Self::new(Color::WHITE, Color::BLACK, 1.0)
    }
}
