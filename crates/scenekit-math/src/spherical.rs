use crate::{EPSILON, Vec3, acos, atan2, clamp, cos, sin};

/// 球坐标，以 Y 轴为极轴。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Spherical {
    /// 到原点的半径。
    pub radius: f32,
    /// 从正 Y 轴起算的极角。
    pub phi: f32,
    /// XZ 平面内的方位角。
    pub theta: f32,
}

impl Spherical {
    /// 创建球坐标。
    #[inline]
    pub const fn new(radius: f32, phi: f32, theta: f32) -> Self {
        Self { radius, phi, theta }
    }

    /// 从向量转换。
    pub fn from_vec3(value: Vec3) -> Self {
        let radius = value.length();
        if radius <= EPSILON {
            return Self::default();
        }
        Self::new(
            radius,
            acos(clamp(value.y / radius, -1.0, 1.0)),
            atan2(value.x, value.z),
        )
    }

    /// 转换为向量。
    pub fn to_vec3(self) -> Vec3 {
        let sin_phi = sin(self.phi);
        Vec3::new(
            self.radius * sin_phi * sin(self.theta),
            self.radius * cos(self.phi),
            self.radius * sin_phi * cos(self.theta),
        )
    }

    /// 钳制极角。
    #[inline]
    pub fn clamp_phi(mut self, min: f32, max: f32) -> Self {
        self.phi = clamp(self.phi, min, max);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_close;

    #[test]
    fn spherical_round_trips_vec3() {
        let value = Vec3::new(2.0, 3.0, 4.0);
        let out = Spherical::from_vec3(value).to_vec3();
        assert_close(out.x, value.x);
        assert_close(out.y, value.y);
        assert_close(out.z, value.z);
    }
}
