use scenekit_math::{Mat4, Ray3, Vec2, Vec3};

use crate::{Frustum, sanitize_near_far};

/// 带有右手 WebGPU 深度投影的正交相机。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrthographicCamera {
    /// 投影左边界。
    pub left: f32,
    /// 投影右边界。
    pub right: f32,
    /// 投影下边界。
    pub bottom: f32,
    /// 投影上边界。
    pub top: f32,
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

impl OrthographicCamera {
    /// 创建一个正交相机。
    pub fn new(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let (near, far) = sanitize_near_far(near, far);
        Self {
            left,
            right,
            bottom,
            top,
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
        Mat4::orthographic(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.near,
            self.far,
        )
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

impl Default for OrthographicCamera {
    #[inline]
    fn default() -> Self {
        Self::new(-1.0, 1.0, -1.0, 1.0, 0.1, 1000.0)
    }
}
