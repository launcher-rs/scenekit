use crate::{EPSILON, Vec3, atan2, cos, sin, sqrt};

/// 绕 Y 轴的柱坐标。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cylindrical {
    /// XZ 平面内的半径。
    pub radius: f32,
    /// XZ 平面内的角度。
    pub theta: f32,
    /// Y 轴上的高度。
    pub y: f32,
}

impl Cylindrical {
    /// 创建柱坐标。
    #[inline]
    pub const fn new(radius: f32, theta: f32, y: f32) -> Self {
        Self { radius, theta, y }
    }

    /// 从向量转换。
    pub fn from_vec3(value: Vec3) -> Self {
        let radius = sqrt(value.x * value.x + value.z * value.z);
        let theta = if radius <= EPSILON {
            0.0
        } else {
            atan2(value.x, value.z)
        };
        Self::new(radius, theta, value.y)
    }

    /// 转换为向量。
    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(
            self.radius * sin(self.theta),
            self.y,
            self.radius * cos(self.theta),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_close;

    #[test]
    fn cylindrical_round_trips_vec3() {
        let value = Vec3::new(2.0, 3.0, 4.0);
        let out = Cylindrical::from_vec3(value).to_vec3();
        assert_close(out.x, value.x);
        assert_close(out.y, value.y);
        assert_close(out.z, value.z);
    }
}
