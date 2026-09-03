use core::ops::{Mul, MulAssign, Neg};

use crate::{EPSILON, Mat4, Vec3, Vec4, acos, clamp, cos, sin, sqrt};

/// 表示三维旋转的单位四元数。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quat {
    /// 虚部的 X 分量。
    pub x: f32,
    /// 虚部的 Y 分量。
    pub y: f32,
    /// 虚部的 Z 分量。
    pub z: f32,
    /// 实部分量。
    pub w: f32,
}

impl Quat {
    /// 单位旋转四元数。
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// 从分量创建四元数。
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// 从轴和弧度角创建四元数。
    pub fn from_axis_angle(axis: Vec3, angle_rad: f32) -> Self {
        let axis = axis.normalize();
        if axis.length_squared() <= EPSILON {
            return Self::IDENTITY;
        }
        let half = angle_rad * 0.5;
        let s = sin(half);
        Self::new(axis.x * s, axis.y * s, axis.z * s, cos(half)).normalize()
    }

    /// 从弧度表示的 XYZ 欧拉角创建四元数。
    #[inline]
    pub fn from_euler_xyz(x: f32, y: f32, z: f32) -> Self {
        crate::Euler::new(x, y, z, crate::RotationOrder::XYZ).to_quat()
    }

    /// 创建从一个方向到另一个方向的最短旋转。
    pub fn from_rotation_arc(from: Vec3, to: Vec3) -> Self {
        let from = from.normalize();
        let to = to.normalize();
        if from.length_squared() <= EPSILON || to.length_squared() <= EPSILON {
            return Self::IDENTITY;
        }

        let dot = from.dot(to);
        if dot > 1.0 - EPSILON {
            return Self::IDENTITY;
        }
        if dot < -1.0 + EPSILON {
            let axis = if from.x.abs() < 0.9 {
                from.cross(Vec3::X).normalize()
            } else {
                from.cross(Vec3::Y).normalize()
            };
            return Self::from_axis_angle(axis, core::f32::consts::PI);
        }

        let cross = from.cross(to);
        Self::new(cross.x, cross.y, cross.z, 1.0 + dot).normalize()
    }

    /// 从矩阵的旋转部分提取四元数。
    pub fn from_mat4(matrix: Mat4) -> Self {
        let m00 = matrix.get(0, 0);
        let m11 = matrix.get(1, 1);
        let m22 = matrix.get(2, 2);
        let trace = m00 + m11 + m22;

        if trace > 0.0 {
            let s = sqrt(trace + 1.0) * 2.0;
            Self::new(
                (matrix.get(2, 1) - matrix.get(1, 2)) / s,
                (matrix.get(0, 2) - matrix.get(2, 0)) / s,
                (matrix.get(1, 0) - matrix.get(0, 1)) / s,
                0.25 * s,
            )
        } else if m00 > m11 && m00 > m22 {
            let s = sqrt(1.0 + m00 - m11 - m22) * 2.0;
            Self::new(
                0.25 * s,
                (matrix.get(0, 1) + matrix.get(1, 0)) / s,
                (matrix.get(0, 2) + matrix.get(2, 0)) / s,
                (matrix.get(2, 1) - matrix.get(1, 2)) / s,
            )
        } else if m11 > m22 {
            let s = sqrt(1.0 + m11 - m00 - m22) * 2.0;
            Self::new(
                (matrix.get(0, 1) + matrix.get(1, 0)) / s,
                0.25 * s,
                (matrix.get(1, 2) + matrix.get(2, 1)) / s,
                (matrix.get(0, 2) - matrix.get(2, 0)) / s,
            )
        } else {
            let s = sqrt(1.0 + m22 - m00 - m11) * 2.0;
            Self::new(
                (matrix.get(0, 2) + matrix.get(2, 0)) / s,
                (matrix.get(1, 2) + matrix.get(2, 1)) / s,
                0.25 * s,
                (matrix.get(1, 0) - matrix.get(0, 1)) / s,
            )
        }
        .normalize()
    }

    /// 返回点积。
    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    /// 返回长度的平方。
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// 返回长度。
    #[inline]
    pub fn length(self) -> f32 {
        sqrt(self.length_squared())
    }

    /// 两个四元数相乘。
    #[inline]
    pub fn mul_quat(self, rhs: Self) -> Self {
        Self::new(
            self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }

    /// 用此四元数旋转向量。
    #[inline]
    pub fn mul_vec3(self, rhs: Vec3) -> Vec3 {
        let q = self.normalize();
        let u = Vec3::new(q.x, q.y, q.z);
        let s = q.w;
        u * (2.0 * u.dot(rhs)) + rhs * (s * s - u.dot(u)) + u.cross(rhs) * (2.0 * s)
    }

    /// 返回共轭四元数。
    #[inline]
    pub fn conjugate(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }

    /// 返回逆四元数。
    #[inline]
    pub fn inverse(self) -> Self {
        let len_sq = self.length_squared();
        if len_sq <= EPSILON {
            Self::IDENTITY
        } else {
            self.conjugate() * (1.0 / len_sq)
        }
    }

    /// 返回归一化后的四元数，对于接近零的输入返回单位四元数。
    #[inline]
    pub fn normalize(self) -> Self {
        let length = self.length();
        if length <= EPSILON {
            Self::IDENTITY
        } else {
            self * (1.0 / length)
        }
    }

    /// 球面插值到 `rhs`。
    pub fn slerp(self, rhs: Self, t: f32) -> Self {
        let t = clamp(t, 0.0, 1.0);
        if t <= 0.0 {
            return self;
        }
        if t >= 1.0 {
            return rhs;
        }
        let mut end = rhs;
        let mut cos_half_theta = self.dot(end);

        if cos_half_theta < -EPSILON {
            end = -end;
            cos_half_theta = -cos_half_theta;
        }

        if cos_half_theta >= 1.0 - EPSILON {
            return Self::new(
                self.x + t * (end.x - self.x),
                self.y + t * (end.y - self.y),
                self.z + t * (end.z - self.z),
                self.w + t * (end.w - self.w),
            )
            .normalize();
        }

        let half_theta = acos(clamp(cos_half_theta, -1.0, 1.0));
        let sin_half_theta = sin(half_theta);
        if sin_half_theta.abs() <= EPSILON {
            return self;
        }

        let ratio_a = sin((1.0 - t) * half_theta) / sin_half_theta;
        let ratio_b = sin(t * half_theta) / sin_half_theta;
        Self::new(
            self.x * ratio_a + end.x * ratio_b,
            self.y * ratio_a + end.y * ratio_b,
            self.z * ratio_a + end.z * ratio_b,
            self.w * ratio_a + end.w * ratio_b,
        )
        .normalize()
    }

    /// 将此四元数转换为旋转矩阵。
    pub fn to_mat4(self) -> Mat4 {
        let q = self.normalize();
        let x2 = q.x + q.x;
        let y2 = q.y + q.y;
        let z2 = q.z + q.z;
        let xx = q.x * x2;
        let xy = q.x * y2;
        let xz = q.x * z2;
        let yy = q.y * y2;
        let yz = q.y * z2;
        let zz = q.z * z2;
        let wx = q.w * x2;
        let wy = q.w * y2;
        let wz = q.w * z2;

        Mat4::from_cols(
            Vec4::new(1.0 - (yy + zz), xy + wz, xz - wy, 0.0),
            Vec4::new(xy - wz, 1.0 - (xx + zz), yz + wx, 0.0),
            Vec4::new(xz + wy, yz - wx, 1.0 - (xx + yy), 0.0),
            Vec4::W,
        )
    }

    /// 提取弧度表示的 XYZ 欧拉角。
    #[inline]
    pub fn to_euler_xyz(self) -> Vec3 {
        let euler = crate::Euler::from_quat(self, crate::RotationOrder::XYZ);
        Vec3::new(euler.x, euler.y, euler.z)
    }

    /// 返回到另一个四元数的绝对角度距离。
    #[inline]
    pub fn angle_between(self, rhs: Self) -> f32 {
        2.0 * acos(clamp(
            self.normalize().dot(rhs.normalize()).abs(),
            -1.0,
            1.0,
        ))
    }

    /// 返回数组形式 `[x, y, z, w]` 的四元数。
    #[inline]
    pub const fn to_array(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl Default for Quat {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Mul for Quat {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_quat(rhs)
    }
}

impl MulAssign for Quat {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul_quat(rhs);
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.mul_vec3(rhs)
    }
}

impl Mul<f32> for Quat {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Neg for Quat {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

#[cfg(feature = "approx")]
impl approx::AbsDiffEq for Quat {
    type Epsilon = f32;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f32::abs_diff_eq(&self.x, &other.x, epsilon)
            && f32::abs_diff_eq(&self.y, &other.y, epsilon)
            && f32::abs_diff_eq(&self.z, &other.z, epsilon)
            && f32::abs_diff_eq(&self.w, &other.w, epsilon)
    }
}

#[cfg(feature = "approx")]
impl approx::RelativeEq for Quat {
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
        f32::relative_eq(&self.x, &other.x, epsilon, max_relative)
            && f32::relative_eq(&self.y, &other.y, epsilon, max_relative)
            && f32::relative_eq(&self.z, &other.z, epsilon, max_relative)
            && f32::relative_eq(&self.w, &other.w, epsilon, max_relative)
    }
}

#[cfg(feature = "approx")]
impl approx::UlpsEq for Quat {
    #[inline]
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        f32::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
            && f32::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
            && f32::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
            && f32::ulps_eq(&self.w, &other.w, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_close;

    #[test]
    fn slerp_handles_endpoints_and_midpoint() {
        let a = Quat::IDENTITY;
        let b = Quat::from_axis_angle(Vec3::Y, core::f32::consts::PI);
        assert_eq!(a.slerp(b, 0.0), a);
        assert_eq!(a.slerp(b, 1.0), b);

        let midpoint = a.slerp(b, 0.5);
        let rotated = midpoint.mul_vec3(Vec3::X);
        assert_close(rotated.x, 0.0);
        assert_close(rotated.z, -1.0);
    }

    #[test]
    fn inverse_undoes_rotation() {
        let q = Quat::from_axis_angle(Vec3::Y, 0.8);
        let v = Vec3::new(1.0, 2.0, 3.0);
        let rotated = q.mul_vec3(v);
        let restored = q.inverse().mul_vec3(rotated);
        assert_close(restored.x, v.x);
        assert_close(restored.y, v.y);
        assert_close(restored.z, v.z);
    }

    #[test]
    fn rotation_arc_rotates_between_vectors() {
        let q = Quat::from_rotation_arc(Vec3::X, Vec3::Y);
        let rotated = q.mul_vec3(Vec3::X);
        assert_close(rotated.x, 0.0);
        assert_close(rotated.y, 1.0);
    }
}
