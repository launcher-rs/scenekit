use scenekit_camera::{OrthographicCamera, PerspectiveCamera};
use scenekit_core::Color;
use scenekit_math::{Mat4, Vec3};

use crate::LineGeometry;

/// 相机视锥体线框辅助工具。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CameraHelper {
    /// 相机的视图投影矩阵。
    pub view_projection: Mat4,
    /// 线段颜色。
    pub color: Color,
}

impl CameraHelper {
    /// 从视图投影矩阵创建辅助工具。
    #[inline]
    pub const fn new(view_projection: Mat4, color: Color) -> Self {
        Self {
            view_projection,
            color,
        }
    }

    /// 从透视相机创建辅助工具。
    #[inline]
    pub fn from_perspective(camera: &PerspectiveCamera, color: Color) -> Self {
        Self::new(camera.view_projection(), color)
    }

    /// 从正交相机创建辅助工具。
    #[inline]
    pub fn from_orthographic(camera: &OrthographicCamera, color: Color) -> Self {
        Self::new(camera.view_projection(), color)
    }

    /// 生成视锥体边缘几何体。
    pub fn to_geometry(&self) -> LineGeometry {
        let inverse = self.view_projection.inverse().unwrap_or(Mat4::IDENTITY);
        let ndc = [
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(-1.0, 1.0, 0.0),
            Vec3::new(-1.0, -1.0, 1.0),
            Vec3::new(1.0, -1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(-1.0, 1.0, 1.0),
        ];
        let mut corners = [Vec3::ZERO; 8];
        for (out, corner) in corners.iter_mut().zip(ndc) {
            *out = inverse.mul_vec3(corner);
        }

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
        for (a, b) in edges {
            geometry.push_segment(corners[a], corners[b], self.color);
        }
        geometry
    }
}
