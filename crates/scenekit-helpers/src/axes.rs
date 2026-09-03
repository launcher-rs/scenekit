use scenekit_core::Color;
use scenekit_math::Vec3;

use crate::{EPSILON, LineGeometry};

/// RGB XYZ 坐标轴辅助工具。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AxesHelper {
    /// 每个正向轴的长度。
    pub size: f32,
}

impl AxesHelper {
    /// 创建坐标轴辅助工具。
    #[inline]
    pub const fn new(size: f32) -> Self {
        Self { size }
    }

    /// 生成线段列表几何体。
    pub fn to_geometry(&self) -> LineGeometry {
        let size = self.size.abs().max(EPSILON);
        let mut geometry = LineGeometry::new();
        geometry.push_segment(Vec3::ZERO, Vec3::X * size, Color::RED);
        geometry.push_segment(Vec3::ZERO, Vec3::Y * size, Color::GREEN);
        geometry.push_segment(Vec3::ZERO, Vec3::Z * size, Color::BLUE);
        geometry
    }
}

impl Default for AxesHelper {
    #[inline]
    fn default() -> Self {
        Self::new(1.0)
    }
}
