use scenekit_core::Color;

/// 恒定的环境场景光。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AmbientLight {
    /// 线性 RGB 光照颜色。
    pub color: Color,
    /// 标量强度。
    pub intensity: f32,
}

impl AmbientLight {
    /// 创建环境光。
    #[inline]
    pub const fn new(color: Color, intensity: f32) -> Self {
        Self { color, intensity }
    }
}

impl Default for AmbientLight {
    #[inline]
    fn default() -> Self {
        Self::new(Color::WHITE, 1.0)
    }
}
