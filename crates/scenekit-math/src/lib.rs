#![cfg_attr(not(feature = "std"), no_std)]

//! scenekit 的自定义标量 `f32` 数学原语。
//!
//! 该 crate 刻意保持最少依赖，可在无 `std` 的环境下使用。
//! 在面向需要可移植三角函数的 `no_std` 平台时，启用 `libm` feature。

pub mod bounds;
pub mod cylindrical;
pub mod euler;
pub mod mat;
pub mod plane;
pub mod quat;
pub mod ray;
pub mod spherical;
pub mod transform;
pub mod vec;

pub use bounds::{Aabb, Sphere};
pub use cylindrical::Cylindrical;
pub use euler::{Euler, RotationOrder};
pub use mat::{Mat3, Mat4};
pub use plane::Plane;
pub use quat::Quat;
pub use ray::Ray3;
pub use spherical::Spherical;
pub use transform::Transform;
pub use vec::{Vec2, Vec3, Vec4};

pub(crate) const EPSILON: f32 = 1.0e-6;
#[cfg(all(not(feature = "libm"), not(feature = "std")))]
pub(crate) const PI: f32 = core::f32::consts::PI;
#[cfg(all(not(feature = "libm"), not(feature = "std")))]
pub(crate) const FRAC_PI_2: f32 = core::f32::consts::FRAC_PI_2;

#[inline]
pub(crate) fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

#[inline]
pub(crate) fn sqrt(value: f32) -> f32 {
    #[cfg(feature = "libm")]
    {
        libm::sqrtf(value)
    }
    #[cfg(all(not(feature = "libm"), feature = "std"))]
    {
        value.sqrt()
    }
    #[cfg(all(not(feature = "libm"), not(feature = "std")))]
    {
        if value <= 0.0 {
            return 0.0;
        }
        let mut x = if value >= 1.0 { value } else { 1.0 };
        let mut i = 0;
        while i < 8 {
            x = 0.5 * (x + value / x);
            i += 1;
        }
        x
    }
}

#[inline]
pub(crate) fn sin(value: f32) -> f32 {
    #[cfg(feature = "libm")]
    {
        libm::sinf(value)
    }
    #[cfg(all(not(feature = "libm"), feature = "std"))]
    {
        value.sin()
    }
    #[cfg(all(not(feature = "libm"), not(feature = "std")))]
    {
        let x = reduce_half_pi(value);
        let x2 = x * x;
        x * (1.0 - x2 / 6.0 + (x2 * x2) / 120.0 - (x2 * x2 * x2) / 5040.0
            + (x2 * x2 * x2 * x2) / 362_880.0)
    }
}

#[inline]
pub(crate) fn cos(value: f32) -> f32 {
    #[cfg(feature = "libm")]
    {
        libm::cosf(value)
    }
    #[cfg(all(not(feature = "libm"), feature = "std"))]
    {
        value.cos()
    }
    #[cfg(all(not(feature = "libm"), not(feature = "std")))]
    {
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
        sign * (1.0 - x2 / 2.0 + (x2 * x2) / 24.0 - (x2 * x2 * x2) / 720.0
            + (x2 * x2 * x2 * x2) / 40_320.0)
    }
}

#[inline]
pub(crate) fn tan(value: f32) -> f32 {
    #[cfg(feature = "libm")]
    {
        libm::tanf(value)
    }
    #[cfg(all(not(feature = "libm"), feature = "std"))]
    {
        value.tan()
    }
    #[cfg(all(not(feature = "libm"), not(feature = "std")))]
    {
        let c = cos(value);
        if c.abs() <= EPSILON {
            if value >= 0.0 {
                f32::INFINITY
            } else {
                -f32::INFINITY
            }
        } else {
            sin(value) / c
        }
    }
}

#[inline]
pub(crate) fn acos(value: f32) -> f32 {
    #[cfg(feature = "libm")]
    {
        libm::acosf(value)
    }
    #[cfg(all(not(feature = "libm"), feature = "std"))]
    {
        value.acos()
    }
    #[cfg(all(not(feature = "libm"), not(feature = "std")))]
    {
        FRAC_PI_2 - asin(value)
    }
}

#[inline]
pub(crate) fn asin(value: f32) -> f32 {
    #[cfg(feature = "libm")]
    {
        libm::asinf(value)
    }
    #[cfg(all(not(feature = "libm"), feature = "std"))]
    {
        value.asin()
    }
    #[cfg(all(not(feature = "libm"), not(feature = "std")))]
    {
        let x = clamp(value, -1.0, 1.0);
        atan2(x, sqrt((1.0 - x * x).max(0.0)))
    }
}

#[inline]
pub(crate) fn atan2(y: f32, x: f32) -> f32 {
    #[cfg(feature = "libm")]
    {
        libm::atan2f(y, x)
    }
    #[cfg(all(not(feature = "libm"), feature = "std"))]
    {
        y.atan2(x)
    }
    #[cfg(all(not(feature = "libm"), not(feature = "std")))]
    {
        if x > 0.0 {
            atan_approx(y / x)
        } else if x < 0.0 && y >= 0.0 {
            atan_approx(y / x) + PI
        } else if x < 0.0 && y < 0.0 {
            atan_approx(y / x) - PI
        } else if y > 0.0 {
            FRAC_PI_2
        } else if y < 0.0 {
            -FRAC_PI_2
        } else {
            0.0
        }
    }
}

#[cfg(all(not(feature = "libm"), not(feature = "std")))]
#[inline]
fn reduce_pi(mut value: f32) -> f32 {
    let two_pi = 2.0 * PI;
    while value > PI {
        value -= two_pi;
    }
    while value < -PI {
        value += two_pi;
    }
    value
}

#[cfg(all(not(feature = "libm"), not(feature = "std")))]
#[inline]
fn reduce_half_pi(value: f32) -> f32 {
    let value = reduce_pi(value);
    if value > FRAC_PI_2 {
        PI - value
    } else if value < -FRAC_PI_2 {
        -PI - value
    } else {
        value
    }
}

#[cfg(all(not(feature = "libm"), not(feature = "std")))]
#[inline]
fn atan_approx(value: f32) -> f32 {
    let sign = value.signum();
    let abs = value.abs();
    if abs > 1.0 {
        sign * FRAC_PI_2 - sign * atan_approx(1.0 / abs)
    } else if abs > 0.414_213_57 {
        sign * (core::f32::consts::FRAC_PI_4 + atan_approx((abs - 1.0) / (abs + 1.0)))
    } else {
        let x2 = value * value;
        value
            * (1.0 - x2 / 3.0 + (x2 * x2) / 5.0 - (x2 * x2 * x2) / 7.0 + (x2 * x2 * x2 * x2) / 9.0
                - (x2 * x2 * x2 * x2 * x2) / 11.0)
    }
}

#[cfg(test)]
pub(crate) fn assert_close(a: f32, b: f32) {
    assert!((a - b).abs() <= 1.0e-4, "{a} != {b}");
}
