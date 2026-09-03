use alloc::vec::Vec;

use scenekit_core::{MaterialId, MeshId, Renderable, ValidationError};
use scenekit_math::Mat4;

/// 相同网格和材质的多个实例的绘制元数据。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InstancedMesh {
    /// 网格资源标识符。
    pub mesh_id: MeshId,
    /// 材质资源标识符。
    pub material_id: MaterialId,
    /// 每实例世界变换。
    pub transforms: Vec<Mat4>,
    /// 稳定的渲染顺序。值越小越先渲染。
    pub render_order: u32,
}

impl InstancedMesh {
    /// 创建空的实例化网格。
    #[inline]
    pub const fn new(mesh_id: MeshId, material_id: MaterialId) -> Self {
        Self {
            mesh_id,
            material_id,
            transforms: Vec::new(),
            render_order: 0,
        }
    }

    /// 创建预留变换容量的实例化网格。
    #[inline]
    pub fn with_capacity(mesh_id: MeshId, material_id: MaterialId, capacity: usize) -> Self {
        Self {
            mesh_id,
            material_id,
            transforms: Vec::with_capacity(capacity),
            render_order: 0,
        }
    }

    /// 返回实例数量。
    #[inline]
    pub fn len(&self) -> usize {
        self.transforms.len()
    }

    /// 返回是否没有实例。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.transforms.is_empty()
    }

    /// 追加一个实例变换。
    #[inline]
    pub fn push_transform(&mut self, transform: Mat4) {
        self.transforms.push(transform);
    }

    /// 按索引设置实例变换。
    pub fn set_transform_at(
        &mut self,
        index: usize,
        transform: Mat4,
    ) -> Result<(), ValidationError> {
        let Some(slot) = self.transforms.get_mut(index) else {
            return Err(ValidationError::OutOfRange);
        };
        *slot = transform;
        Ok(())
    }

    /// 按索引返回实例变换。
    #[inline]
    pub fn transform_at(&self, index: usize) -> Option<Mat4> {
        self.transforms.get(index).copied()
    }

    /// 返回设置了渲染顺序的实例化网格。
    #[inline]
    pub const fn render_order(mut self, render_order: u32) -> Self {
        self.render_order = render_order;
        self
    }
}

impl Renderable for InstancedMesh {
    #[inline]
    fn render_order(&self) -> u32 {
        self.render_order
    }
}
