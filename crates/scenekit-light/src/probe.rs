use scenekit_core::ValidationError;
use scenekit_math::Vec3;

/// 使用三阶 RGB 系数的球谐环境光。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LightProbe {
    /// 九个 SH 系数，每个存储 RGB 辐射度。
    pub sh_coefficients: [Vec3; 9],
    /// 渲染器应用于系数的标量强度。
    pub intensity: f32,
}

impl LightProbe {
    /// 从显式 SH 系数创建光探针。
    #[inline]
    pub const fn from_coefficients(sh_coefficients: [Vec3; 9], intensity: f32) -> Self {
        Self {
            sh_coefficients,
            intensity,
        }
    }

    /// 将原始立方体面采样投影为 SH 系数。
    ///
    /// 面顺序为 `+X, -X, +Y, -Y, +Z, -Z`。采样值为每个面按行优先排列的
    /// 线性 RGB 辐射度值。
    pub fn from_cube_faces(
        faces: [&[Vec3]; 6],
        face_size: u32,
        intensity: f32,
    ) -> Result<Self, ValidationError> {
        if face_size == 0 {
            return Err(ValidationError::InvalidState);
        }
        let expected = face_size as usize * face_size as usize;
        for face in faces {
            if face.len() != expected {
                return Err(ValidationError::InvalidState);
            }
        }

        let mut projection = ShProjection::new();
        for (face_index, face) in faces.iter().enumerate() {
            for y in 0..face_size {
                for x in 0..face_size {
                    let u = ((x as f32 + 0.5) / face_size as f32) * 2.0 - 1.0;
                    let v = ((y as f32 + 0.5) / face_size as f32) * 2.0 - 1.0;
                    let raw = cube_direction(face_index, u, v);
                    let inv_len = 1.0 / raw.length();
                    let direction = raw * inv_len;
                    let weight = inv_len * inv_len * inv_len;
                    projection.add(direction, face[(y * face_size + x) as usize], weight);
                }
            }
        }

        projection.finish(intensity)
    }

    /// 将原始等距柱状采样投影为 SH 系数。
    ///
    /// 采样值为按行优先排列的线性 RGB 辐射度值。
    pub fn from_equirectangular_samples(
        samples: &[Vec3],
        width: u32,
        height: u32,
        intensity: f32,
    ) -> Result<Self, ValidationError> {
        if width == 0 || height == 0 || samples.len() != width as usize * height as usize {
            return Err(ValidationError::InvalidState);
        }

        let mut projection = ShProjection::new();
        for y in 0..height {
            let v = (y as f32 + 0.5) / height as f32;
            let phi = v * core::f32::consts::PI;
            let (sin_phi, cos_phi) = sin_cos(phi);
            for x in 0..width {
                let u = (x as f32 + 0.5) / width as f32;
                let theta = u * core::f32::consts::TAU;
                let (sin_theta, cos_theta) = sin_cos(theta);
                let direction = Vec3::new(sin_phi * cos_theta, cos_phi, sin_phi * sin_theta);
                let sample = samples[(y * width + x) as usize];
                projection.add(direction, sample, sin_phi.max(0.0));
            }
        }

        projection.finish(intensity)
    }
}

impl Default for LightProbe {
    #[inline]
    fn default() -> Self {
        Self::from_coefficients([Vec3::ZERO; 9], 1.0)
    }
}

/// SH 投影累积器。
struct ShProjection {
    /// 累积的 SH 系数。
    coefficients: [Vec3; 9],
    /// 累积的总权重。
    total_weight: f32,
}

impl ShProjection {
    #[inline]
    const fn new() -> Self {
        Self {
            coefficients: [Vec3::ZERO; 9],
            total_weight: 0.0,
        }
    }

    /// 向投影中添加一个方向的辐射度采样。
    #[inline]
    fn add(&mut self, direction: Vec3, radiance: Vec3, weight: f32) {
        if weight <= 0.0 {
            return;
        }
        let basis = sh_basis(direction);
        for (coefficient, basis_value) in self.coefficients.iter_mut().zip(basis) {
            *coefficient += radiance * (basis_value * weight);
        }
        self.total_weight += weight;
    }

    /// 完成投影并返回光探针。
    #[inline]
    fn finish(self, intensity: f32) -> Result<LightProbe, ValidationError> {
        if self.total_weight <= 0.0 {
            return Err(ValidationError::InvalidState);
        }
        let scale = core::f32::consts::TAU * 2.0 / self.total_weight;
        let mut coefficients = self.coefficients;
        for coefficient in &mut coefficients {
            *coefficient *= scale;
        }
        Ok(LightProbe::from_coefficients(coefficients, intensity))
    }
}

/// 根据立方体面索引和 UV 坐标计算方向向量。
#[inline]
fn cube_direction(face_index: usize, u: f32, v: f32) -> Vec3 {
    match face_index {
        0 => Vec3::new(1.0, -v, -u),
        1 => Vec3::new(-1.0, -v, u),
        2 => Vec3::new(u, 1.0, v),
        3 => Vec3::new(u, -1.0, -v),
        4 => Vec3::new(u, -v, 1.0),
        _ => Vec3::new(-u, -v, -1.0),
    }
}

/// 计算三阶球谐基函数值。
#[inline]
fn sh_basis(direction: Vec3) -> [f32; 9] {
    let x = direction.x;
    let y = direction.y;
    let z = direction.z;
    [
        0.282_095,
        0.488_603 * y,
        0.488_603 * z,
        0.488_603 * x,
        1.092_548 * x * y,
        1.092_548 * y * z,
        0.315_392 * (3.0 * z * z - 1.0),
        1.092_548 * x * z,
        0.546_274 * (x * x - y * y),
    ]
}

/// 计算正弦和余弦值，兼容 no_std 环境。
#[inline]
fn sin_cos(value: f32) -> (f32, f32) {
    #[cfg(feature = "std")]
    {
        value.sin_cos()
    }
    #[cfg(not(feature = "std"))]
    {
        (sin_approx(value), cos_approx(value))
    }
}

/// 将角度规约到 [-PI, PI] 范围。
#[cfg(not(feature = "std"))]
fn reduce_pi(mut value: f32) -> f32 {
    const PI: f32 = core::f32::consts::PI;
    const TAU: f32 = core::f32::consts::TAU;
    while value > PI {
        value -= TAU;
    }
    while value < -PI {
        value += TAU;
    }
    value
}

/// 无 std 环境下的正弦近似。
#[cfg(not(feature = "std"))]
fn sin_approx(value: f32) -> f32 {
    const FRAC_PI_2: f32 = core::f32::consts::FRAC_PI_2;
    const PI: f32 = core::f32::consts::PI;
    let mut x = reduce_pi(value);
    if x > FRAC_PI_2 {
        x = PI - x;
    } else if x < -FRAC_PI_2 {
        x = -PI - x;
    }
    let x2 = x * x;
    x * (1.0 - x2 / 6.0 + (x2 * x2) / 120.0 - (x2 * x2 * x2) / 5040.0)
}

/// 无 std 环境下的余弦近似。
#[cfg(not(feature = "std"))]
fn cos_approx(value: f32) -> f32 {
    const FRAC_PI_2: f32 = core::f32::consts::FRAC_PI_2;
    const PI: f32 = core::f32::consts::PI;
    let mut x = reduce_pi(value);
    let mut sign = 1.0;
    if x > FRAC_PI_2 {
        x = PI - x;
        sign = -1.0;
    } else if x < -FRAC_PI_2 {
        x = -PI - x;
        sign = -1.0;
    }
    let x2 = x * x;
    sign * (1.0 - x2 / 2.0 + (x2 * x2) / 24.0 - (x2 * x2 * x2) / 720.0)
}
