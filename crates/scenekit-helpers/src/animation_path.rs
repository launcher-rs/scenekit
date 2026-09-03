//! 可视化采样动画路径的线段几何体辅助工具。
//!
//! 从采样位置构建折线，使动画轨迹可以像 Three.js 的
//! `GridHelper`/`CameraHelper` 调试叠加层一样绘制。

use alloc::vec::Vec;

use scenekit_core::{Color, ValidationError};
use scenekit_math::Vec3;

use crate::LineGeometry;

/// 用于可视化采样动画轨迹的折线辅助工具。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimationPathHelper {
    /// 有序的折线点。
    pub points: Vec<Vec3>,
    /// 线段颜色。
    pub color: Color,
}

impl AnimationPathHelper {
    /// 从采样点创建辅助工具。
    #[inline]
    pub const fn new(points: Vec<Vec3>, color: Color) -> Self {
        Self { points, color }
    }

    /// 在 `[0, duration]` 范围内以 `steps` 个线段采样闭包 `f: f32 -> Vec3`，
    /// 并构建路径辅助工具。
    pub fn sample(
        steps: usize,
        duration: f32,
        color: Color,
        mut f: impl FnMut(f32) -> Vec3,
    ) -> Self {
        let mut points = Vec::with_capacity(steps + 1);
        for i in 0..=steps {
            let t = if steps == 0 {
                0.0
            } else {
                duration * (i as f32) / (steps as f32)
            };
            points.push(f(t));
        }
        Self::new(points, color)
    }

    /// 验证路径至少有两个点。
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.points.len() < 2 {
            return Err(ValidationError::InvalidState);
        }
        Ok(())
    }

    /// 将路径转换为 `LineGeometry`（线段条带）。
    pub fn to_geometry(&self) -> LineGeometry {
        let mut geometry = LineGeometry::new();
        for window in self.points.windows(2) {
            geometry.push_segment(window[0], window[1], self.color);
        }
        geometry
    }
}
