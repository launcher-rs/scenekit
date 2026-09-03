use scenekit_math::{Mat4, Ray3, Vec2, Vec3};

use crate::{Frustum, clamp, sanitize_near_far};

/// 带有右手 WebGPU 深度投影的透视相机。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PerspectiveCamera {
    /// 垂直视野角（弧度）。
    pub fov_y: f32,
    /// 宽高比，宽度除以高度。
    pub aspect: f32,
    /// 近裁剪面距离。
    pub near: f32,
    /// 远裁剪面距离。
    pub far: f32,
    /// 相机在世界空间中的位置。
    pub position: Vec3,
    /// 世界空间中的观察目标。
    pub target: Vec3,
    /// 上方向。
    pub up: Vec3,
}

impl PerspectiveCamera {
    /// 从度数表示的垂直视野角创建相机。
    pub fn new(fov_y_deg: f32, aspect: f32, near: f32, far: f32) -> Self {
        let (near, far) = sanitize_near_far(near, far);
        let min_fov = core::f32::consts::PI / 180.0;
        let max_fov = 179.0 * core::f32::consts::PI / 180.0;
        Self {
            fov_y: clamp(fov_y_deg * core::f32::consts::PI / 180.0, min_fov, max_fov),
            aspect: if aspect.abs() > crate::EPSILON {
                aspect.abs()
            } else {
                1.0
            },
            near,
            far,
            position: Vec3::new(0.0, 0.0, 1.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
        }
    }

    /// 返回设置了世界空间位置的相机。
    #[inline]
    pub const fn position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    /// 返回设置了观察目标的相机。
    #[inline]
    pub const fn target(mut self, target: Vec3) -> Self {
        self.target = target;
        self
    }

    /// 返回设置了上方向向量的相机。
    #[inline]
    pub const fn up(mut self, up: Vec3) -> Self {
        self.up = up;
        self
    }

    /// 返回投影矩阵。
    #[inline]
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective(self.fov_y, self.aspect, self.near, self.far)
    }

    /// 返回视图矩阵。
    #[inline]
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at(self.position, self.target, self.up)
    }

    /// 返回投影矩阵乘以视图矩阵的结果。
    #[inline]
    pub fn view_projection(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// 提取视锥体。
    #[inline]
    pub fn frustum(&self) -> Frustum {
        Frustum::from_view_projection(self.view_projection())
    }

    /// 从 `[-1, 1]` 范围的归一化设备坐标构建射线。
    pub fn screen_to_ray(&self, ndc: Vec2) -> Ray3 {
        let inverse = self.view_projection().inverse().unwrap_or(Mat4::IDENTITY);
        let near = inverse.mul_vec3(Vec3::new(ndc.x, ndc.y, 0.0));
        let far = inverse.mul_vec3(Vec3::new(ndc.x, ndc.y, 1.0));
        Ray3::new(near, far - near)
    }
}

impl Default for PerspectiveCamera {
    #[inline]
    fn default() -> Self {
        Self::new(60.0, 1.0, 0.1, 1000.0)
    }
}
