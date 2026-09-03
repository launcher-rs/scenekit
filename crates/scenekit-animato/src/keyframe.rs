//! 基于片段的动画的时间采样关键帧轨道。
//!
//! 这些轨道补充了 [`crate::tracks`] 中的过程式补间/弹簧轨道：
//! 过程式轨道运行一次固定的补间/弹簧；关键帧轨道在有序的关键帧数组上
//! 采样任意片段本地时间，匹配 glTF/FBX 导入语义。
//! [`crate::mixer`] 中的混合器通过 [`crate::clip::ClipTrack`] 消费这些轨道。

use alloc::vec::Vec;
use core::f32;

use scenekit_core::Color;
use scenekit_math::{Quat, Vec3};

/// 关键帧插值模式。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KeyframeInterpolation {
    /// 相邻关键帧之间的线性插值。
    Linear,
    /// 保持前一个关键帧的值直到下一个时间边界。
    Step,
    /// 三次 Hermite 样条插值（切线打包为入/出斜率）。
    CubicSpline,
}

/// 定位 `time` 所在的括号关键帧对，并返回 `(left_index, alpha)`，
/// 其中 `alpha` 是 `times[left]` 和 `times[left + 1]` 之间的归一化位置
/// `[0, 1]`。
///
/// 时间会被钳位到第一个/最后一个关键帧。对于 `Step` 插值，
/// 调用者直接使用 `left_index`；`alpha` 被忽略。
fn bracket(times: &[f32], time: f32) -> (usize, f32) {
    if times.is_empty() {
        return (0, 0.0);
    }
    if time <= times[0] {
        return (0, 0.0);
    }
    if let Some(&last) = times.last()
        && time >= last
    {
        return (times.len() - 1, 0.0);
    }
    // 二分查找括号对。
    let mut lo = 0usize;
    let mut hi = times.len() - 1;
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if times[mid] <= time {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let span = (times[hi] - times[lo]).max(f32::EPSILON);
    let alpha = ((time - times[lo]) / span).clamp(0.0, 1.0);
    (lo, alpha)
}

/// 验证打包标量关键帧轨道：非空、单调非递减时间，
/// 且 `values.len() == times.len() * per_key`。
fn validate_scalar(times: &[f32], values: &[f32], per_key: usize) -> bool {
    if times.is_empty() || values.len() != times.len() * per_key {
        return false;
    }
    let mut prev = f32::NEG_INFINITY;
    for &t in times {
        if !t.is_finite() || t < prev {
            return false;
        }
        prev = t;
    }
    true
}

/// 标量关键帧轨道。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyframeScalar {
    /// 以秒为单位的关键帧时间，单调非递减。
    pub times: Vec<f32>,
    /// 打包值（`Linear/Step` 时 `len == times.len()`，
    /// `CubicSpline` 时 `3 * times.len()`：入切线、值、出切线）。
    pub values: Vec<f32>,
    /// 插值模式。
    pub interpolation: KeyframeInterpolation,
}

impl KeyframeScalar {
    /// 创建经过验证的标量关键帧轨道。
    pub fn new(times: Vec<f32>, values: Vec<f32>, interpolation: KeyframeInterpolation) -> Self {
        let per_key = if interpolation == KeyframeInterpolation::CubicSpline {
            3
        } else {
            1
        };
        assert!(
            validate_scalar(&times, &values, per_key),
            "invalid scalar keyframe track"
        );
        Self {
            times,
            values,
            interpolation,
        }
    }

    /// 片段持续时间（最后一个关键帧时间）。
    #[inline]
    pub fn duration(&self) -> f32 {
        self.times.last().copied().unwrap_or(0.0)
    }

    /// 在 `time` 处采样轨道。
    pub fn sample(&self, time: f32) -> f32 {
        let (i, a) = bracket(&self.times, time);
        match self.interpolation {
            KeyframeInterpolation::Step => self.values[i],
            KeyframeInterpolation::Linear => {
                if i + 1 >= self.times.len() {
                    self.values[i]
                } else {
                    self.values[i] + (self.values[i + 1] - self.values[i]) * a
                }
            }
            KeyframeInterpolation::CubicSpline => {
                // 打包布局：每个关键帧 [入切线, 值, 出切线]。
                if i + 1 >= self.times.len() {
                    self.values[i * 3 + 1]
                } else {
                    let t = a;
                    let p0 = self.values[i * 3 + 1];
                    let m0 = self.values[i * 3 + 2];
                    let p1 = self.values[(i + 1) * 3 + 1];
                    let m1 = self.values[(i + 1) * 3];
                    let t2 = t * t;
                    let t3 = t2 * t;
                    let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
                    let h10 = t3 - 2.0 * t2 + t;
                    let h01 = -2.0 * t3 + 3.0 * t2;
                    let h11 = t3 - t2;
                    h00 * p0 + h10 * m0 + h01 * p1 + h11 * m1
                }
            }
        }
    }
}

/// 三维向量关键帧轨道。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyframeVec3 {
    /// 以秒为单位的关键帧时间。
    pub times: Vec<f32>,
    /// 每个关键帧一个值。
    pub values: Vec<Vec3>,
    /// 插值模式（CubicSpline 在 v1.4 中回退到线性 lerp）。
    pub interpolation: KeyframeInterpolation,
}

impl KeyframeVec3 {
    /// 创建经过验证的向量关键帧轨道。
    pub fn new(times: Vec<f32>, values: Vec<Vec3>, interpolation: KeyframeInterpolation) -> Self {
        assert_eq!(times.len(), values.len(), "keyframe count mismatch");
        Self {
            times,
            values,
            interpolation,
        }
    }

    /// 片段持续时间（最后一个关键帧时间）。
    #[inline]
    pub fn duration(&self) -> f32 {
        self.times.last().copied().unwrap_or(0.0)
    }

    /// 在 `time` 处采样轨道。
    pub fn sample(&self, time: f32) -> Vec3 {
        let (i, a) = bracket(&self.times, time);
        match self.interpolation {
            KeyframeInterpolation::Step => self.values[i],
            _ => {
                if i + 1 >= self.times.len() {
                    self.values[i]
                } else {
                    self.values[i].lerp(self.values[i + 1], a)
                }
            }
        }
    }
}

/// 四元数关键帧轨道（线性模式使用球面线性插值，步进模式使用最近值）。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyframeQuat {
    /// 以秒为单位的关键帧时间。
    pub times: Vec<f32>,
    /// 每个关键帧一个四元数。
    pub values: Vec<Quat>,
    /// 插值模式（Step 保持值；Linear/CubicSpline 使用球面线性插值）。
    pub interpolation: KeyframeInterpolation,
}

impl KeyframeQuat {
    /// 创建经过验证的四元数关键帧轨道。
    pub fn new(times: Vec<f32>, values: Vec<Quat>, interpolation: KeyframeInterpolation) -> Self {
        assert_eq!(times.len(), values.len(), "keyframe count mismatch");
        Self {
            times,
            values,
            interpolation,
        }
    }

    /// 片段持续时间（最后一个关键帧时间）。
    #[inline]
    pub fn duration(&self) -> f32 {
        self.times.last().copied().unwrap_or(0.0)
    }

    /// 在 `time` 处采样轨道，始终取最短弧。
    pub fn sample(&self, time: f32) -> Quat {
        let (i, a) = bracket(&self.times, time);
        match self.interpolation {
            KeyframeInterpolation::Step => self.values[i],
            _ => {
                if i + 1 >= self.times.len() {
                    self.values[i].normalize()
                } else {
                    let v0 = self.values[i];
                    let mut v1 = self.values[i + 1];
                    if v0.dot(v1) < 0.0 {
                        v1 = -v1;
                    }
                    v0.slerp(v1, a).normalize()
                }
            }
        }
    }
}

/// 颜色关键帧轨道。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyframeColor {
    /// 以秒为单位的关键帧时间。
    pub times: Vec<f32>,
    /// 每个关键帧一个颜色。
    pub values: Vec<Color>,
    /// 插值模式（CubicSpline 在 v1.4 中回退到线性 lerp）。
    pub interpolation: KeyframeInterpolation,
}

impl KeyframeColor {
    /// 创建经过验证的颜色关键帧轨道。
    pub fn new(times: Vec<f32>, values: Vec<Color>, interpolation: KeyframeInterpolation) -> Self {
        assert_eq!(times.len(), values.len(), "keyframe count mismatch");
        Self {
            times,
            values,
            interpolation,
        }
    }

    /// 片段持续时间（最后一个关键帧时间）。
    #[inline]
    pub fn duration(&self) -> f32 {
        self.times.last().copied().unwrap_or(0.0)
    }

    /// 在 `time` 处采样轨道。
    pub fn sample(&self, time: f32) -> Color {
        let (i, a) = bracket(&self.times, time);
        match self.interpolation {
            KeyframeInterpolation::Step => self.values[i],
            _ => {
                if i + 1 >= self.times.len() {
                    self.values[i]
                } else {
                    self.values[i].lerp(self.values[i + 1], a)
                }
            }
        }
    }
}

/// 布尔关键帧轨道（仅支持 Step 插值）。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyframeBool {
    /// 以秒为单位的关键帧时间。
    pub times: Vec<f32>,
    /// 每个关键帧一个布尔值。
    pub values: Vec<bool>,
}

impl KeyframeBool {
    /// 创建经过验证的布尔关键帧轨道。
    pub fn new(times: Vec<f32>, values: Vec<bool>) -> Self {
        assert_eq!(times.len(), values.len(), "keyframe count mismatch");
        Self { times, values }
    }

    /// 片段持续时间（最后一个关键帧时间）。
    #[inline]
    pub fn duration(&self) -> f32 {
        self.times.last().copied().unwrap_or(0.0)
    }

    /// 在 `time` 处采样轨道（保持最近关键帧的值）。
    pub fn sample(&self, time: f32) -> bool {
        let (i, _) = bracket(&self.times, time);
        self.values[i]
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    #[test]
    fn scalar_linear_samples_midpoint() {
        let track = KeyframeScalar::new(
            vec![0.0, 1.0],
            vec![0.0, 10.0],
            KeyframeInterpolation::Linear,
        );
        assert_eq!(track.sample(0.0), 0.0);
        assert!((track.sample(0.5) - 5.0).abs() < 1e-4);
        assert_eq!(track.sample(1.0), 10.0);
    }

    #[test]
    fn step_holds_previous_value() {
        let track =
            KeyframeScalar::new(vec![0.0, 1.0], vec![0.0, 10.0], KeyframeInterpolation::Step);
        assert_eq!(track.sample(0.99), 0.0);
        assert_eq!(track.sample(1.0), 10.0);
    }

    #[test]
    fn quat_takes_shortest_arc() {
        let track = KeyframeQuat::new(
            vec![0.0, 1.0],
            vec![Quat::IDENTITY, Quat::from_axis_angle(Vec3::Y, 3.0)],
            KeyframeInterpolation::Linear,
        );
        let mid = track.sample(0.5);
        assert!((mid.length() - 1.0).abs() < 1e-4);
    }

    #[test]
    fn bool_holds_until_boundary() {
        let track = KeyframeBool::new(vec![0.0, 0.5, 1.0], vec![false, true, false]);
        assert!(!track.sample(0.0));
        assert!(track.sample(0.7));
        assert!(!track.sample(1.0));
    }
}
