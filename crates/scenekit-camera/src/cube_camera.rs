use scenekit_math::{Mat4, Vec3};

use crate::sanitize_near_far;

/// 立方体贴图面顺序。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CubeFace {
    /// 正 X 面。
    PositiveX,
    /// 负 X 面。
    NegativeX,
    /// 正 Y 面。
    PositiveY,
    /// 负 Y 面。
    NegativeY,
    /// 正 Z 面。
    PositiveZ,
    /// 负 Z 面。
    NegativeZ,
}

/// 生成六个 90 度视图投影矩阵用于立方体贴图的相机。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CubeCamera {
    /// 捕获位置。
    pub position: Vec3,
    /// 近裁剪面距离。
    pub near: f32,
    /// 远裁剪面距离。
    pub far: f32,
}

impl CubeFace {
    /// 按存储顺序返回所有立方体面。
    #[inline]
    pub const fn all() -> [Self; 6] {
        [
            Self::PositiveX,
            Self::NegativeX,
            Self::PositiveY,
            Self::NegativeY,
            Self::PositiveZ,
            Self::NegativeZ,
        ]
    }

    /// 返回面的方向和上方向向量。
    #[inline]
    pub const fn basis(self) -> (Vec3, Vec3) {
        match self {
            Self::PositiveX => (Vec3::X, Vec3::new(0.0, -1.0, 0.0)),
            Self::NegativeX => (Vec3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0)),
            Self::PositiveY => (Vec3::Y, Vec3::Z),
            Self::NegativeY => (Vec3::new(0.0, -1.0, 0.0), Vec3::NEG_Z),
            Self::PositiveZ => (Vec3::Z, Vec3::new(0.0, -1.0, 0.0)),
            Self::NegativeZ => (Vec3::NEG_Z, Vec3::new(0.0, -1.0, 0.0)),
        }
    }
}

impl CubeCamera {
    /// 创建一个立方体贴图相机。
    #[inline]
    pub fn new(position: Vec3, near: f32, far: f32) -> Self {
        let (near, far) = sanitize_near_far(near, far);
        Self {
            position,
            near,
            far,
        }
    }

    /// 返回所有面共享的投影矩阵。
    #[inline]
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective(core::f32::consts::FRAC_PI_2, 1.0, self.near, self.far)
    }

    /// 返回指定面的视图矩阵。
    #[inline]
    pub fn view_matrix(&self, face: CubeFace) -> Mat4 {
        let (direction, up) = face.basis();
        Mat4::look_at(self.position, self.position + direction, up)
    }

    /// 返回指定面的投影矩阵乘以视图矩阵的结果。
    #[inline]
    pub fn view_projection(&self, face: CubeFace) -> Mat4 {
        self.projection_matrix() * self.view_matrix(face)
    }

    /// 返回所有六个视图投影矩阵。
    #[inline]
    pub fn view_projections(&self) -> [Mat4; 6] {
        let faces = CubeFace::all();
        [
            self.view_projection(faces[0]),
            self.view_projection(faces[1]),
            self.view_projection(faces[2]),
            self.view_projection(faces[3]),
            self.view_projection(faces[4]),
            self.view_projection(faces[5]),
        ]
    }
}

impl Default for CubeCamera {
    #[inline]
    fn default() -> Self {
        Self::new(Vec3::ZERO, 0.1, 1000.0)
    }
}
