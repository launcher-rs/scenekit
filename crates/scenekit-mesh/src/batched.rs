use alloc::vec::Vec;

use scenekit_core::{Bounded, MaterialId, ValidationError};
use scenekit_math::{Aabb, Vec3};

use crate::Geometry;

/// 批量网格中单个几何体的绘制范围元数据。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BatchedGeometryRange {
    /// 合并几何体中的首个顶点。
    pub vertex_start: u32,
    /// 此范围内的顶点数量。
    pub vertex_count: u32,
    /// 合并索引缓冲区中的首个索引。
    pub index_start: u32,
    /// 此范围内的索引数量。
    pub index_count: u32,
    /// 此范围使用的材质。
    pub material_id: MaterialId,
}

/// 多个几何体合并为一个 CPU 端几何体，带有绘制范围。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BatchedMesh {
    /// 合并后的 CPU 端几何体。
    pub geometry: Geometry,
    /// 每个源几何体的范围。
    pub ranges: Vec<BatchedGeometryRange>,
}

impl BatchedMesh {
    /// 创建空的批量网格。
    #[inline]
    pub const fn new() -> Self {
        Self {
            geometry: Geometry::new(),
            ranges: Vec::new(),
        }
    }

    /// 将几何体添加到批次中并返回范围索引。
    pub fn add_geometry(
        &mut self,
        geometry: &Geometry,
        material_id: MaterialId,
    ) -> Result<usize, ValidationError> {
        geometry.validate()?;
        let vertex_start = self.geometry.positions.len() as u32;
        let index_start = self.geometry.indices.len() as u32;
        let index_count = if geometry.indices.is_empty() {
            geometry.positions.len()
        } else {
            geometry.indices.len()
        } as u32;
        let range = BatchedGeometryRange {
            vertex_start,
            vertex_count: geometry.positions.len() as u32,
            index_start,
            index_count,
            material_id,
        };
        self.geometry.merge(geometry);
        self.ranges.push(range);
        Ok(self.ranges.len() - 1)
    }

    /// 返回所有几何体范围。
    #[inline]
    pub fn ranges(&self) -> &[BatchedGeometryRange] {
        &self.ranges
    }
}

impl Bounded for BatchedMesh {
    #[inline]
    fn aabb(&self) -> Aabb {
        self.geometry.aabb()
    }

    #[inline]
    fn bounding_sphere(&self) -> (Vec3, f32) {
        self.geometry.bounding_sphere()
    }
}
