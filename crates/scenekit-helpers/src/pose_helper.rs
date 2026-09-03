//! `SkeletonPose` 的骨骼轴向 Gizmo 辅助工具。
//!
//! 在每个骨骼原点处绘制一个小的坐标轴三元组，用于姿态调试。

use alloc::vec::Vec;

use scenekit_core::{Color, ValidationError};
use scenekit_math::Vec3;

use crate::LineGeometry;

/// 在每个骨骼原点处绘制一个小的坐标轴三元组，用于姿态调试。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PoseHelper {
    /// 每个骨骼的三元组：`(原点, x终点, y终点, z终点)`。
    pub triads: Vec<(Vec3, Vec3, Vec3, Vec3)>,
    /// 坐标轴颜色 `(x, y, z)`。
    pub colors: [Color; 3],
}

impl PoseHelper {
    /// 从骨骼原点和每个坐标轴的 `size` 长度构建三元组。
    pub fn from_origins(origins: &[Vec3], size: f32) -> Self {
        let triads = origins
            .iter()
            .map(|&o| {
                (
                    o,
                    o + Vec3::new(size, 0.0, 0.0),
                    o + Vec3::new(0.0, size, 0.0),
                    o + Vec3::new(0.0, 0.0, size),
                )
            })
            .collect();
        Self {
            triads,
            colors: [
                Color::rgb(1.0, 0.0, 0.0),
                Color::rgb(0.0, 1.0, 0.0),
                Color::rgb(0.0, 0.0, 1.0),
            ],
        }
    }

    /// 设置自定义坐标轴颜色。
    #[inline]
    pub const fn with_colors(mut self, colors: [Color; 3]) -> Self {
        self.colors = colors;
        self
    }

    /// 验证至少存在一个三元组。
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.triads.is_empty() {
            return Err(ValidationError::InvalidState);
        }
        Ok(())
    }

    /// 将三元组转换为多色的 `[LineGeometry; 3]`（x/y/z）。
    pub fn to_geometries(&self) -> [LineGeometry; 3] {
        let mut gx = LineGeometry::new();
        let mut gy = LineGeometry::new();
        let mut gz = LineGeometry::new();
        for &(o, x, y, z) in &self.triads {
            gx.push_segment(o, x, self.colors[0]);
            gy.push_segment(o, y, self.colors[1]);
            gz.push_segment(o, z, self.colors[2]);
        }
        [gx, gy, gz]
    }
}
