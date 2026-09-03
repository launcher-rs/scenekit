use alloc::string::{String, ToString};
use alloc::vec::Vec;

use scenekit_core::ValidationError;
use scenekit_math::Vec3;

/// 用于混合形状风格网格变形的顶点增量数据。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MorphTarget {
    /// 人类可读的目标名称。
    pub name: String,
    /// 每顶点位置增量。
    pub positions_delta: Vec<Vec3>,
    /// 每顶点法线增量。
    pub normals_delta: Vec<Vec3>,
    /// 目标栈中的混合权重。
    pub weight: f32,
}

impl MorphTarget {
    /// 创建空的变形目标。
    #[inline]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            positions_delta: Vec::new(),
            normals_delta: Vec::new(),
            weight: 0.0,
        }
    }

    /// 返回设置了位置增量的变形目标。
    #[inline]
    pub fn positions_delta(mut self, positions_delta: Vec<Vec3>) -> Self {
        self.positions_delta = positions_delta;
        self
    }

    /// 返回设置了法线增量的变形目标。
    #[inline]
    pub fn normals_delta(mut self, normals_delta: Vec<Vec3>) -> Self {
        self.normals_delta = normals_delta;
        self
    }

    /// 返回设置了混合权重的变形目标。
    #[inline]
    pub const fn weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }

    /// 验证非空增量数组是否与顶点数量匹配。
    pub fn validate(&self, vertex_count: usize) -> Result<(), ValidationError> {
        if !self.positions_delta.is_empty() && self.positions_delta.len() != vertex_count {
            return Err(ValidationError::InvalidState);
        }
        if !self.normals_delta.is_empty() && self.normals_delta.len() != vertex_count {
            return Err(ValidationError::InvalidState);
        }
        Ok(())
    }
}

impl Default for MorphTarget {
    #[inline]
    fn default() -> Self {
        Self::new("morph".to_string())
    }
}
