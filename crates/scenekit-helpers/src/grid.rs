use scenekit_core::Color;
use scenekit_math::Vec3;

use crate::{EPSILON, LineGeometry};

/// XZ 平面网格辅助工具。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GridHelper {
    /// 网格总范围。
    pub size: f32,
    /// 每个坐标轴的细分数。
    pub divisions: u32,
    /// 中心轴颜色。
    pub color1: Color,
    /// 常规网格线颜色。
    pub color2: Color,
}

impl GridHelper {
    /// 创建网格辅助工具。
    #[inline]
    pub const fn new(size: f32, divisions: u32) -> Self {
        Self {
            size,
            divisions,
            color1: Color::from_rgba(0.45, 0.45, 0.45, 1.0),
            color2: Color::from_rgba(0.2, 0.2, 0.2, 1.0),
        }
    }

    /// 返回带有自定义颜色的辅助工具。
    #[inline]
    pub const fn colors(mut self, center: Color, grid: Color) -> Self {
        self.color1 = center;
        self.color2 = grid;
        self
    }

    /// 生成线段列表几何体。
    pub fn to_geometry(&self) -> LineGeometry {
        let size = self.size.abs().max(EPSILON);
        let divisions = self.divisions.max(1);
        let half = size * 0.5;
        let step = size / divisions as f32;
        let mut geometry = LineGeometry::new();
        geometry.positions.reserve(((divisions + 1) * 4) as usize);
        geometry.colors.reserve(((divisions + 1) * 4) as usize);

        for i in 0..=divisions {
            let k = -half + i as f32 * step;
            let color = if k.abs() <= EPSILON {
                self.color1
            } else {
                self.color2
            };
            geometry.push_segment(Vec3::new(-half, 0.0, k), Vec3::new(half, 0.0, k), color);
            geometry.push_segment(Vec3::new(k, 0.0, -half), Vec3::new(k, 0.0, half), color);
        }

        geometry
    }
}

impl Default for GridHelper {
    #[inline]
    fn default() -> Self {
        Self::new(10.0, 10)
    }
}
