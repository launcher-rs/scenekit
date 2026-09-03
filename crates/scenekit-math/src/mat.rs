use core::ops::{Index, IndexMut, Mul};

use crate::{EPSILON, Quat, Transform, Vec3, Vec4, tan};

/// 3x3 列主序矩阵。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mat3 {
    /// 矩阵列。
    pub cols: [Vec3; 3],
}

/// 4x4 列主序矩阵。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mat4 {
    /// 矩阵列。
    pub cols: [Vec4; 4],
}

impl Mat3 {
    /// 单位矩阵。
    pub const IDENTITY: Self = Self::from_cols(Vec3::X, Vec3::Y, Vec3::Z);

    /// 从列向量创建矩阵。
    #[inline]
    pub const fn from_cols(x: Vec3, y: Vec3, z: Vec3) -> Self {
        Self { cols: [x, y, z] }
    }

    /// 从 `Mat4` 中提取左上角 3x3 矩阵。
    #[inline]
    pub fn from_mat4(matrix: Mat4) -> Self {
        Self::from_cols(
            matrix.cols[0].truncate(),
            matrix.cols[1].truncate(),
            matrix.cols[2].truncate(),
        )
    }

    /// 返回指定行和列的元素。
    #[inline]
    pub fn get(self, row: usize, col: usize) -> f32 {
        self.cols[col][row]
    }

    /// 返回行列式。
    #[inline]
    pub fn determinant(self) -> f32 {
        let a = self.get(0, 0);
        let b = self.get(0, 1);
        let c = self.get(0, 2);
        let d = self.get(1, 0);
        let e = self.get(1, 1);
        let f = self.get(1, 2);
        let g = self.get(2, 0);
        let h = self.get(2, 1);
        let i = self.get(2, 2);

        a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    }

    /// 返回逆矩阵，若矩阵不可逆则返回 `None`。
    #[allow(clippy::needless_range_loop)]
    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() <= EPSILON {
            return None;
        }
        let inv_det = 1.0 / det;
        let m = |r, c| self.get(r, c);
        Some(Self::from_cols(
            Vec3::new(
                (m(1, 1) * m(2, 2) - m(1, 2) * m(2, 1)) * inv_det,
                (m(1, 2) * m(2, 0) - m(1, 0) * m(2, 2)) * inv_det,
                (m(1, 0) * m(2, 1) - m(1, 1) * m(2, 0)) * inv_det,
            ),
            Vec3::new(
                (m(0, 2) * m(2, 1) - m(0, 1) * m(2, 2)) * inv_det,
                (m(0, 0) * m(2, 2) - m(0, 2) * m(2, 0)) * inv_det,
                (m(0, 1) * m(2, 0) - m(0, 0) * m(2, 1)) * inv_det,
            ),
            Vec3::new(
                (m(0, 1) * m(1, 2) - m(0, 2) * m(1, 1)) * inv_det,
                (m(0, 2) * m(1, 0) - m(0, 0) * m(1, 2)) * inv_det,
                (m(0, 0) * m(1, 1) - m(0, 1) * m(1, 0)) * inv_det,
            ),
        ))
    }

    /// 返回转置矩阵。
    #[inline]
    pub fn transpose(self) -> Self {
        Self::from_cols(
            Vec3::new(self.get(0, 0), self.get(0, 1), self.get(0, 2)),
            Vec3::new(self.get(1, 0), self.get(1, 1), self.get(1, 2)),
            Vec3::new(self.get(2, 0), self.get(2, 1), self.get(2, 2)),
        )
    }

    /// 与另一个矩阵相乘。
    #[inline]
    pub fn mul_mat3(self, rhs: Self) -> Self {
        Self::from_cols(
            self.mul_vec3(rhs.cols[0]),
            self.mul_vec3(rhs.cols[1]),
            self.mul_vec3(rhs.cols[2]),
        )
    }

    /// 与向量相乘。
    #[inline]
    pub fn mul_vec3(self, rhs: Vec3) -> Vec3 {
        self.cols[0] * rhs.x + self.cols[1] * rhs.y + self.cols[2] * rhs.z
    }

    /// 返回列主序数组形式的矩阵。
    #[inline]
    pub fn to_cols_array(self) -> [f32; 9] {
        [
            self.cols[0].x,
            self.cols[0].y,
            self.cols[0].z,
            self.cols[1].x,
            self.cols[1].y,
            self.cols[1].z,
            self.cols[2].x,
            self.cols[2].y,
            self.cols[2].z,
        ]
    }
}

impl Mat4 {
    /// 单位矩阵。
    pub const IDENTITY: Self = Self::from_cols(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W);

    /// 从列向量创建矩阵。
    #[inline]
    pub const fn from_cols(x: Vec4, y: Vec4, z: Vec4, w: Vec4) -> Self {
        Self { cols: [x, y, z, w] }
    }

    /// 从列主序数组创建矩阵。
    #[inline]
    pub const fn from_cols_array(values: [f32; 16]) -> Self {
        Self::from_cols(
            Vec4::new(values[0], values[1], values[2], values[3]),
            Vec4::new(values[4], values[5], values[6], values[7]),
            Vec4::new(values[8], values[9], values[10], values[11]),
            Vec4::new(values[12], values[13], values[14], values[15]),
        )
    }

    /// 返回指定行和列的元素。
    #[inline]
    pub fn get(self, row: usize, col: usize) -> f32 {
        self.cols[col][row]
    }

    /// 返回右手坐标系透视投影矩阵，使用 WebGPU 深度范围。
    pub fn perspective(fov_y_rad: f32, aspect: f32, near: f32, far: f32) -> Self {
        if aspect.abs() <= EPSILON || near <= 0.0 || far <= near {
            return Self::IDENTITY;
        }

        let f = 1.0 / tan(fov_y_rad * 0.5);
        Self::from_cols(
            Vec4::new(f / aspect, 0.0, 0.0, 0.0),
            Vec4::new(0.0, f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, far / (near - far), -1.0),
            Vec4::new(0.0, 0.0, (near * far) / (near - far), 0.0),
        )
    }

    /// 返回右手坐标系正交投影矩阵，使用 WebGPU 深度范围。
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let width = right - left;
        let height = top - bottom;
        let depth = near - far;
        if width.abs() <= EPSILON || height.abs() <= EPSILON || depth.abs() <= EPSILON {
            return Self::IDENTITY;
        }

        Self::from_cols(
            Vec4::new(2.0 / width, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 2.0 / height, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0 / depth, 0.0),
            Vec4::new(
                -(right + left) / width,
                -(top + bottom) / height,
                near / depth,
                1.0,
            ),
        )
    }

    /// 返回右手坐标系观察矩阵，从 `eye` 看向 `target`。
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let forward = (target - eye).normalize();
        if forward.length_squared() <= EPSILON {
            return Self::from_translation(-eye);
        }

        let right = forward.cross(up).normalize();
        let up = right.cross(forward).normalize();

        Self::from_cols(
            Vec4::new(right.x, up.x, -forward.x, 0.0),
            Vec4::new(right.y, up.y, -forward.y, 0.0),
            Vec4::new(right.z, up.z, -forward.z, 0.0),
            Vec4::new(-right.dot(eye), -up.dot(eye), forward.dot(eye), 1.0),
        )
    }

    /// 创建平移矩阵。
    #[inline]
    pub fn from_translation(value: Vec3) -> Self {
        Self::from_cols(
            Vec4::X,
            Vec4::Y,
            Vec4::Z,
            Vec4::new(value.x, value.y, value.z, 1.0),
        )
    }

    /// 从四元数创建旋转矩阵。
    #[inline]
    pub fn from_rotation(rotation: Quat) -> Self {
        rotation.to_mat4()
    }

    /// 创建缩放矩阵。
    #[inline]
    pub fn from_scale(value: Vec3) -> Self {
        Self::from_cols(
            Vec4::new(value.x, 0.0, 0.0, 0.0),
            Vec4::new(0.0, value.y, 0.0, 0.0),
            Vec4::new(0.0, 0.0, value.z, 0.0),
            Vec4::W,
        )
    }

    /// 从平移、旋转和缩放创建矩阵。
    #[inline]
    pub fn from_trs(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self::from_translation(translation)
            .mul_mat4(Self::from_rotation(rotation))
            .mul_mat4(Self::from_scale(scale))
    }

    /// 与另一个矩阵相乘。
    #[inline]
    pub fn mul_mat4(self, rhs: Self) -> Self {
        Self::from_cols(
            self.mul_vec4(rhs.cols[0]),
            self.mul_vec4(rhs.cols[1]),
            self.mul_vec4(rhs.cols[2]),
            self.mul_vec4(rhs.cols[3]),
        )
    }

    /// 与向量相乘。
    #[inline]
    pub fn mul_vec4(self, rhs: Vec4) -> Vec4 {
        self.cols[0] * rhs.x + self.cols[1] * rhs.y + self.cols[2] * rhs.z + self.cols[3] * rhs.w
    }

    /// 变换点并在可能时进行齐次除法。
    #[inline]
    pub fn mul_vec3(self, rhs: Vec3) -> Vec3 {
        let out = self.mul_vec4(Vec4::new(rhs.x, rhs.y, rhs.z, 1.0));
        if out.w.abs() <= EPSILON {
            out.truncate()
        } else {
            out.truncate() / out.w
        }
    }

    /// 返回逆矩阵，若矩阵不可逆则返回 `None`。
    #[allow(clippy::needless_range_loop)]
    pub fn inverse(self) -> Option<Self> {
        let mut aug = [[0.0_f32; 8]; 4];
        for row in 0..4 {
            for col in 0..4 {
                aug[row][col] = self.get(row, col);
            }
            aug[row][row + 4] = 1.0;
        }

        for col in 0..4 {
            let mut pivot = col;
            let mut pivot_abs = aug[pivot][col].abs();
            for (row, values) in aug.iter().enumerate().skip(col + 1) {
                let value_abs = values[col].abs();
                if value_abs > pivot_abs {
                    pivot = row;
                    pivot_abs = value_abs;
                }
            }
            if pivot_abs <= EPSILON {
                return None;
            }
            if pivot != col {
                aug.swap(pivot, col);
            }

            let inv_pivot = 1.0 / aug[col][col];
            for value in &mut aug[col] {
                *value *= inv_pivot;
            }

            for row in 0..4 {
                if row == col {
                    continue;
                }
                let factor = aug[row][col];
                if factor.abs() <= EPSILON {
                    continue;
                }
                for i in 0..8 {
                    aug[row][i] -= factor * aug[col][i];
                }
            }
        }

        Some(Self::from_cols(
            Vec4::new(aug[0][4], aug[1][4], aug[2][4], aug[3][4]),
            Vec4::new(aug[0][5], aug[1][5], aug[2][5], aug[3][5]),
            Vec4::new(aug[0][6], aug[1][6], aug[2][6], aug[3][6]),
            Vec4::new(aug[0][7], aug[1][7], aug[2][7], aug[3][7]),
        ))
    }

    /// 返回转置矩阵。
    #[inline]
    pub fn transpose(self) -> Self {
        Self::from_cols(
            Vec4::new(
                self.get(0, 0),
                self.get(0, 1),
                self.get(0, 2),
                self.get(0, 3),
            ),
            Vec4::new(
                self.get(1, 0),
                self.get(1, 1),
                self.get(1, 2),
                self.get(1, 3),
            ),
            Vec4::new(
                self.get(2, 0),
                self.get(2, 1),
                self.get(2, 2),
                self.get(2, 3),
            ),
            Vec4::new(
                self.get(3, 0),
                self.get(3, 1),
                self.get(3, 2),
                self.get(3, 3),
            ),
        )
    }

    /// 将 TRS 矩阵分解为平移、旋转和缩放。
    pub fn decompose(self) -> Option<Transform> {
        let translation = self.cols[3].truncate();
        let scale = Vec3::new(
            self.cols[0].truncate().length(),
            self.cols[1].truncate().length(),
            self.cols[2].truncate().length(),
        );
        if scale.x <= EPSILON || scale.y <= EPSILON || scale.z <= EPSILON {
            return None;
        }

        let inv_scale = Vec3::new(1.0 / scale.x, 1.0 / scale.y, 1.0 / scale.z);
        let rotation_matrix = Self::from_cols(
            Vec4::new(
                self.cols[0].x * inv_scale.x,
                self.cols[0].y * inv_scale.x,
                self.cols[0].z * inv_scale.x,
                0.0,
            ),
            Vec4::new(
                self.cols[1].x * inv_scale.y,
                self.cols[1].y * inv_scale.y,
                self.cols[1].z * inv_scale.y,
                0.0,
            ),
            Vec4::new(
                self.cols[2].x * inv_scale.z,
                self.cols[2].y * inv_scale.z,
                self.cols[2].z * inv_scale.z,
                0.0,
            ),
            Vec4::W,
        );

        Some(Transform::new(
            translation,
            Quat::from_mat4(rotation_matrix),
            scale,
        ))
    }

    /// 返回列主序数组形式的矩阵。
    #[inline]
    pub fn to_cols_array(self) -> [f32; 16] {
        [
            self.cols[0].x,
            self.cols[0].y,
            self.cols[0].z,
            self.cols[0].w,
            self.cols[1].x,
            self.cols[1].y,
            self.cols[1].z,
            self.cols[1].w,
            self.cols[2].x,
            self.cols[2].y,
            self.cols[2].z,
            self.cols[2].w,
            self.cols[3].x,
            self.cols[3].y,
            self.cols[3].z,
            self.cols[3].w,
        ]
    }
}

impl Default for Mat3 {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Default for Mat4 {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Index<usize> for Mat3 {
    type Output = Vec3;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.cols[index]
    }
}

impl IndexMut<usize> for Mat3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cols[index]
    }
}

impl Index<usize> for Mat4 {
    type Output = Vec4;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.cols[index]
    }
}

impl IndexMut<usize> for Mat4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cols[index]
    }
}

impl Mul for Mat3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_mat3(rhs)
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.mul_vec3(rhs)
    }
}

impl Mul for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_mat4(rhs)
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        self.mul_vec4(rhs)
    }
}

impl Mul<Vec3> for Mat4 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.mul_vec3(rhs)
    }
}

#[cfg(feature = "approx")]
macro_rules! impl_matrix_approx {
    ($type:ident, $cols:expr) => {
        impl approx::AbsDiffEq for $type {
            type Epsilon = f32;

            #[inline]
            fn default_epsilon() -> Self::Epsilon {
                f32::default_epsilon()
            }

            #[inline]
            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                self.cols
                    .iter()
                    .zip(other.cols.iter())
                    .all(|(a, b)| approx::AbsDiffEq::abs_diff_eq(a, b, epsilon))
            }
        }

        impl approx::RelativeEq for $type {
            #[inline]
            fn default_max_relative() -> Self::Epsilon {
                f32::default_max_relative()
            }

            #[inline]
            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                self.cols
                    .iter()
                    .zip(other.cols.iter())
                    .all(|(a, b)| approx::RelativeEq::relative_eq(a, b, epsilon, max_relative))
            }
        }

        impl approx::UlpsEq for $type {
            #[inline]
            fn default_max_ulps() -> u32 {
                f32::default_max_ulps()
            }

            #[inline]
            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                self.cols
                    .iter()
                    .zip(other.cols.iter())
                    .all(|(a, b)| approx::UlpsEq::ulps_eq(a, b, epsilon, max_ulps))
            }
        }
    };
}

#[cfg(feature = "approx")]
impl_matrix_approx!(Mat3, 3);
#[cfg(feature = "approx")]
impl_matrix_approx!(Mat4, 4);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_close;

    #[test]
    fn perspective_maps_near_and_far_to_webgpu_depth() {
        let projection = Mat4::perspective(core::f32::consts::FRAC_PI_2, 1.0, 0.1, 10.0);
        let near = projection.mul_vec4(Vec4::new(0.0, 0.0, -0.1, 1.0));
        let far = projection.mul_vec4(Vec4::new(0.0, 0.0, -10.0, 1.0));

        assert_close(near.z / near.w, 0.0);
        assert_close(far.z / far.w, 1.0);
    }

    #[test]
    fn orthographic_maps_center_and_depth() {
        let projection = Mat4::orthographic(-1.0, 1.0, -1.0, 1.0, 0.1, 10.0);
        let center = projection.mul_vec4(Vec4::new(0.0, 0.0, -0.1, 1.0));
        assert_close(center.x, 0.0);
        assert_close(center.y, 0.0);
        assert_close(center.z, 0.0);
    }

    #[test]
    fn inverse_multiplies_to_identity() {
        let matrix = Mat4::from_trs(
            Vec3::new(2.0, 3.0, 4.0),
            Quat::from_axis_angle(Vec3::Y, 0.7),
            Vec3::new(2.0, 3.0, 4.0),
        );
        let inverse = matrix.inverse().unwrap();
        let identity = matrix * inverse;
        let values = identity.to_cols_array();
        let expected = Mat4::IDENTITY.to_cols_array();
        for (a, b) in values.into_iter().zip(expected) {
            assert_close(a, b);
        }
    }

    #[test]
    fn transpose_and_column_major_array_work() {
        let matrix = Mat4::from_cols_array([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ]);
        assert_eq!(matrix.to_cols_array()[1], 2.0);
        assert_eq!(matrix.transpose().get(0, 1), 2.0);
    }
}
