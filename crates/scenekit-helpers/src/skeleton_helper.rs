use alloc::vec::Vec;

use scenekit_core::{Color, ValidationError};
use scenekit_math::Vec3;

use crate::LineGeometry;

/// 骨骼线段辅助工具。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkeletonHelper {
    /// 世界空间中的关节位置。
    pub joints: Vec<Vec3>,
    /// 每个关节的父级索引。根关节使用 `None`。
    pub parents: Vec<Option<usize>>,
    /// 线段颜色。
    pub color: Color,
}

impl SkeletonHelper {
    /// 从关节位置和父级索引创建骨骼辅助工具。
    #[inline]
    pub fn new(joints: Vec<Vec3>, parents: Vec<Option<usize>>, color: Color) -> Self {
        Self {
            joints,
            parents,
            color,
        }
    }

    /// 验证父级列表的长度和范围。
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.joints.len() != self.parents.len() {
            return Err(ValidationError::InvalidState);
        }
        for (index, parent) in self.parents.iter().enumerate() {
            if let Some(parent) = parent
                && (*parent >= self.joints.len() || *parent == index)
            {
                return Err(ValidationError::OutOfRange);
            }
        }
        Ok(())
    }

    /// 为每个子关节到其父关节生成一条线段。
    pub fn to_geometry(&self) -> LineGeometry {
        let mut geometry = LineGeometry::new();
        if self.validate().is_err() {
            return geometry;
        }
        for (index, parent) in self.parents.iter().enumerate() {
            if let Some(parent) = parent {
                geometry.push_segment(self.joints[*parent], self.joints[index], self.color);
            }
        }
        geometry
    }
}
