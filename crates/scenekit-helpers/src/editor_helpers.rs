use alloc::vec::Vec;

use scenekit_core::Color;
use scenekit_math::{Aabb, Vec3};

use crate::{BoundingBoxHelper, GridHelper, LineGeometry};

/// 一个或多个世界空间包围盒的选择轮廓辅助工具。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectionHelper {
    /// 选中的包围盒。
    pub bounds: Vec<Aabb>,
    /// 轮廓颜色。
    pub color: Color,
}

impl SelectionHelper {
    /// 将选择轮廓写入可复用的存储。
    pub fn write_geometry(&self, geometry: &mut LineGeometry) {
        geometry.clear();
        geometry.reserve(self.bounds.len() * 24, 0);
        for bounds in &self.bounds {
            geometry.merge(&BoundingBoxHelper::new(*bounds, self.color).to_geometry());
        }
    }

    /// 生成独立的选择几何体。
    pub fn to_geometry(&self) -> LineGeometry {
        let mut geometry = LineGeometry::new();
        self.write_geometry(&mut geometry);
        geometry
    }
}

/// 角点强调的编辑器包围盒辅助工具。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoundsGizmoHelper {
    /// 要可视化的包围盒。
    pub bounds: Aabb,
    /// 每条边的角点线段比例。
    pub corner_fraction: f32,
    /// 线段颜色。
    pub color: Color,
}

impl BoundsGizmoHelper {
    /// 将角点括号写入可复用的存储。
    pub fn write_geometry(&self, geometry: &mut LineGeometry) {
        geometry.clear();
        let full = BoundingBoxHelper::new(self.bounds, self.color).to_geometry();
        let fraction = self.corner_fraction.clamp(0.01, 0.5);
        geometry.reserve(full.positions.len() * 2, 0);
        for segment in full.positions.as_chunks::<2>().0 {
            let a = segment[0];
            let b = segment[1];
            let delta = b - a;
            geometry.push_segment(a, a + delta * fraction, self.color);
            geometry.push_segment(b, b - delta * fraction, self.color);
        }
    }

    /// 生成独立的几何体。
    pub fn to_geometry(&self) -> LineGeometry {
        let mut geometry = LineGeometry::new();
        self.write_geometry(&mut geometry);
        geometry
    }
}

/// 平移到编辑器工作平面原点的网格辅助工具。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SnapGridHelper {
    /// 网格中心。
    pub origin: Vec3,
    /// 网格大小。
    pub size: f32,
    /// 细分数。
    pub divisions: u32,
    /// 主坐标轴颜色。
    pub major_color: Color,
    /// 常规线颜色。
    pub minor_color: Color,
}

impl SnapGridHelper {
    /// 将网格线写入可复用的存储。
    pub fn write_geometry(&self, geometry: &mut LineGeometry) {
        let source = GridHelper::new(self.size, self.divisions)
            .colors(self.major_color, self.minor_color)
            .to_geometry();
        geometry.clear();
        geometry.reserve(source.positions.len(), source.indices.len());
        geometry
            .positions
            .extend(source.positions.iter().map(|point| *point + self.origin));
        geometry.colors.extend_from_slice(&source.colors);
        geometry.indices.extend_from_slice(&source.indices);
    }

    /// 生成独立的网格几何体。
    pub fn to_geometry(&self) -> LineGeometry {
        let mut geometry = LineGeometry::new();
        self.write_geometry(&mut geometry);
        geometry
    }
}
