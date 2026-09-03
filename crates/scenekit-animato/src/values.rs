use animato::Interpolate;
use scenekit_core::Color;
use scenekit_math::{Quat, Vec3};

/// [`Vec3`] 的 Animato 兼容包装器。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimVec3(pub Vec3);

impl From<Vec3> for AnimVec3 {
    #[inline]
    fn from(value: Vec3) -> Self {
        Self(value)
    }
}

impl From<AnimVec3> for Vec3 {
    #[inline]
    fn from(value: AnimVec3) -> Self {
        value.0
    }
}

impl Interpolate for AnimVec3 {
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self(self.0.lerp(other.0, t))
    }
}

/// [`Quat`] 的 Animato 兼容包装器。
///
/// 插值使用四元数球面线性插值（slerp），使旋转轨道保持归一化。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimQuat(pub Quat);

impl From<Quat> for AnimQuat {
    #[inline]
    fn from(value: Quat) -> Self {
        Self(value)
    }
}

impl From<AnimQuat> for Quat {
    #[inline]
    fn from(value: AnimQuat) -> Self {
        value.0
    }
}

impl Interpolate for AnimQuat {
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self(self.0.slerp(other.0, t))
    }
}

/// scenekit [`Color`] 的 Animato 兼容包装器。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimColor(pub Color);

impl From<Color> for AnimColor {
    #[inline]
    fn from(value: Color) -> Self {
        Self(value)
    }
}

impl From<AnimColor> for Color {
    #[inline]
    fn from(value: AnimColor) -> Self {
        value.0
    }
}

impl Interpolate for AnimColor {
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self(self.0.lerp(other.0, t))
    }
}
