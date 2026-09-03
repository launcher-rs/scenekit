use scenekit_core::{Bounded, MaterialId, Renderable};
use scenekit_math::{Aabb, Vec3};

use crate::Geometry;

/// 由几何体和材质标识符组成的可渲染网格。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mesh {
    /// CPU 端几何体数据。
    pub geometry: Geometry,
    /// 材质资源标识符。
    pub material_id: MaterialId,
    /// 稳定的渲染顺序。值越小越先渲染。
    pub render_order: u32,
}

impl Mesh {
    /// 从几何体和材质 ID 创建网格。
    #[inline]
    pub const fn new(geometry: Geometry, material_id: MaterialId) -> Self {
        Self {
            geometry,
            material_id,
            render_order: 0,
        }
    }

    /// 返回设置了渲染顺序的网格。
    #[inline]
    pub const fn render_order(mut self, render_order: u32) -> Self {
        self.render_order = render_order;
        self
    }
}

impl Renderable for Mesh {
    #[inline]
    fn render_order(&self) -> u32 {
        self.render_order
    }
}

impl Bounded for Mesh {
    #[inline]
    fn aabb(&self) -> Aabb {
        self.geometry.aabb()
    }

    #[inline]
    fn bounding_sphere(&self) -> (Vec3, f32) {
        self.geometry.bounding_sphere()
    }
}
