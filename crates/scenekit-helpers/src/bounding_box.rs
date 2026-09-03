use scenekit_core::Color;
use scenekit_math::{Aabb, Vec3};

use crate::LineGeometry;

/// 线框包围盒（AABB）辅助工具。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoundingBoxHelper {
    /// 要可视化的包围盒。
    pub aabb: Aabb,
    /// 线段颜色。
    pub color: Color,
}

impl BoundingBoxHelper {
    /// 从包围盒创建辅助工具。
    #[inline]
    pub const fn new(aabb: Aabb, color: Color) -> Self {
        Self { aabb, color }
    }

    /// 生成线段列表几何体。
    pub fn to_geometry(&self) -> LineGeometry {
        let min = self.aabb.min;
        let max = self.aabb.max;
        let corners = [
            Vec3::new(min.x, min.y, min.z),
            Vec3::new(max.x, min.y, min.z),
            Vec3::new(max.x, max.y, min.z),
            Vec3::new(min.x, max.y, min.z),
            Vec3::new(min.x, min.y, max.z),
            Vec3::new(max.x, min.y, max.z),
            Vec3::new(max.x, max.y, max.z),
            Vec3::new(min.x, max.y, max.z),
        ];
        let edges = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ];
        let mut geometry = LineGeometry::new();
        geometry.positions.reserve(edges.len() * 2);
        geometry.colors.reserve(edges.len() * 2);
        for (a, b) in edges {
            geometry.push_segment(corners[a], corners[b], self.color);
        }
        geometry
    }
}
