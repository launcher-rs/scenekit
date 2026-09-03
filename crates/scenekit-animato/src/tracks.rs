use animato::{Easing, Spring, SpringConfig, Tween, Update};
use scenekit_core::Color;
use scenekit_math::{Quat, Vec3};

use crate::{AnimColor, AnimQuat, AnimVec3};

/// 由 Animato 补间或弹簧原语支持的标量动画轨道。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScalarTrack {
    /// 从起始值到结束值的有限补间。
    Tween(Tween<f32>),
    /// 向目标值运动的物理弹簧。
    Spring(Spring),
}

impl ScalarTrack {
    /// 创建线性标量补间。
    #[inline]
    pub fn tween(start: f32, end: f32, duration: f32) -> Self {
        Self::tween_with_easing(start, end, duration, Easing::Linear)
    }

    /// 创建带有缓动曲线的标量补间。
    #[inline]
    pub fn tween_with_easing(start: f32, end: f32, duration: f32, easing: Easing) -> Self {
        Self::Tween(
            Tween::new(start, end)
                .duration(duration)
                .easing(easing)
                .build(),
        )
    }

    /// 创建从 `start` 初始化并向 `target` 运动的标量弹簧。
    #[inline]
    pub fn spring(start: f32, target: f32, config: SpringConfig) -> Self {
        Self::Spring(spring1(start, target, config))
    }

    /// 推进轨道并返回是否仍在运行。
    #[inline]
    pub fn update(&mut self, dt: f32) -> bool {
        match self {
            Self::Tween(track) => track.update(dt),
            Self::Spring(track) => track.update(dt),
        }
    }

    /// 返回当前标量值。
    #[inline]
    pub fn value(&self) -> f32 {
        match self {
            Self::Tween(track) => track.value(),
            Self::Spring(track) => track.position(),
        }
    }

    /// 返回轨道是否已完成或已稳定。
    #[inline]
    pub fn is_complete(&self) -> bool {
        match self {
            Self::Tween(track) => track.is_complete(),
            Self::Spring(track) => track.is_settled(),
        }
    }
}

/// 三维向量动画轨道。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Vec3Track {
    /// 使用 Animato 插值的有限补间。
    Tween(Tween<AnimVec3>),
    /// 逐分量的 Animato 弹簧。
    Spring {
        /// X 轴弹簧。
        x: Spring,
        /// Y 轴弹簧。
        y: Spring,
        /// Z 轴弹簧。
        z: Spring,
    },
}

impl Vec3Track {
    /// 创建线性向量补间。
    #[inline]
    pub fn tween(start: Vec3, end: Vec3, duration: f32) -> Self {
        Self::tween_with_easing(start, end, duration, Easing::Linear)
    }

    /// 创建带有缓动曲线的向量补间。
    #[inline]
    pub fn tween_with_easing(start: Vec3, end: Vec3, duration: f32, easing: Easing) -> Self {
        Self::Tween(
            Tween::new(AnimVec3(start), AnimVec3(end))
                .duration(duration)
                .easing(easing)
                .build(),
        )
    }

    /// 创建从 `start` 初始化并向 `target` 运动的逐分量弹簧。
    #[inline]
    pub fn spring(start: Vec3, target: Vec3, config: SpringConfig) -> Self {
        Self::Spring {
            x: spring1(start.x, target.x, config.clone()),
            y: spring1(start.y, target.y, config.clone()),
            z: spring1(start.z, target.z, config),
        }
    }

    /// 推进轨道并返回是否仍在运行。
    #[inline]
    pub fn update(&mut self, dt: f32) -> bool {
        match self {
            Self::Tween(track) => track.update(dt),
            Self::Spring { x, y, z } => x.update(dt) | y.update(dt) | z.update(dt),
        }
    }

    /// 返回当前向量值。
    #[inline]
    pub fn value(&self) -> Vec3 {
        match self {
            Self::Tween(track) => track.value().0,
            Self::Spring { x, y, z } => Vec3::new(x.position(), y.position(), z.position()),
        }
    }

    /// 返回轨道是否已完成或已稳定。
    #[inline]
    pub fn is_complete(&self) -> bool {
        match self {
            Self::Tween(track) => track.is_complete(),
            Self::Spring { x, y, z } => x.is_settled() && y.is_settled() && z.is_settled(),
        }
    }
}

/// 四元数旋转动画轨道。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum QuatTrack {
    /// 使用四元数球面线性插值的有限补间。
    Tween(Tween<AnimQuat>),
}

impl QuatTrack {
    /// 创建四元数补间。
    #[inline]
    pub fn tween(start: Quat, end: Quat, duration: f32) -> Self {
        Self::tween_with_easing(start, end, duration, Easing::Linear)
    }

    /// 创建带有缓动曲线的四元数补间。
    #[inline]
    pub fn tween_with_easing(start: Quat, end: Quat, duration: f32, easing: Easing) -> Self {
        Self::Tween(
            Tween::new(AnimQuat(start), AnimQuat(end))
                .duration(duration)
                .easing(easing)
                .build(),
        )
    }

    /// 推进轨道并返回是否仍在运行。
    #[inline]
    pub fn update(&mut self, dt: f32) -> bool {
        match self {
            Self::Tween(track) => track.update(dt),
        }
    }

    /// 返回当前归一化的四元数。
    #[inline]
    pub fn value(&self) -> Quat {
        match self {
            Self::Tween(track) => track.value().0.normalize(),
        }
    }

    /// 返回轨道是否已完成。
    #[inline]
    pub fn is_complete(&self) -> bool {
        match self {
            Self::Tween(track) => track.is_complete(),
        }
    }
}

/// 颜色动画轨道。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ColorTrack {
    /// 使用颜色通道插值的有限补间。
    Tween(Tween<AnimColor>),
    /// 逐分量的 Animato 弹簧。
    Spring {
        /// 红色通道弹簧。
        r: Spring,
        /// 绿色通道弹簧。
        g: Spring,
        /// 蓝色通道弹簧。
        b: Spring,
        /// Alpha 通道弹簧。
        a: Spring,
    },
}

impl ColorTrack {
    /// 创建线性颜色补间。
    #[inline]
    pub fn tween(start: Color, end: Color, duration: f32) -> Self {
        Self::tween_with_easing(start, end, duration, Easing::Linear)
    }

    /// 创建带有缓动曲线的颜色补间。
    #[inline]
    pub fn tween_with_easing(start: Color, end: Color, duration: f32, easing: Easing) -> Self {
        Self::Tween(
            Tween::new(AnimColor(start), AnimColor(end))
                .duration(duration)
                .easing(easing)
                .build(),
        )
    }

    /// 创建从 `start` 初始化并向 `target` 运动的逐分量弹簧。
    #[inline]
    pub fn spring(start: Color, target: Color, config: SpringConfig) -> Self {
        Self::Spring {
            r: spring1(start.r, target.r, config.clone()),
            g: spring1(start.g, target.g, config.clone()),
            b: spring1(start.b, target.b, config.clone()),
            a: spring1(start.a, target.a, config),
        }
    }

    /// 推进轨道并返回是否仍在运行。
    #[inline]
    pub fn update(&mut self, dt: f32) -> bool {
        match self {
            Self::Tween(track) => track.update(dt),
            Self::Spring { r, g, b, a } => {
                r.update(dt) | g.update(dt) | b.update(dt) | a.update(dt)
            }
        }
    }

    /// 返回当前颜色值。
    #[inline]
    pub fn value(&self) -> Color {
        match self {
            Self::Tween(track) => track.value().0,
            Self::Spring { r, g, b, a } => {
                Color::rgba(r.position(), g.position(), b.position(), a.position())
            }
        }
    }

    /// 返回轨道是否已完成或已稳定。
    #[inline]
    pub fn is_complete(&self) -> bool {
        match self {
            Self::Tween(track) => track.is_complete(),
            Self::Spring { r, g, b, a } => {
                r.is_settled() && g.is_settled() && b.is_settled() && a.is_settled()
            }
        }
    }
}

/// 延时布尔开关轨道。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoolTrack {
    start: bool,
    end: bool,
    duration: f32,
    elapsed: f32,
    value: bool,
}

impl BoolTrack {
    /// 创建在 `duration` 结束时切换的布尔轨道。
    #[inline]
    pub fn new(start: bool, end: bool, duration: f32) -> Self {
        Self {
            start,
            end,
            duration: duration.max(0.0),
            elapsed: 0.0,
            value: start,
        }
    }

    /// 创建立即生效的布尔值。
    #[inline]
    pub const fn immediate(value: bool) -> Self {
        Self {
            start: value,
            end: value,
            duration: 0.0,
            elapsed: 0.0,
            value,
        }
    }

    /// 推进轨道并返回是否仍在运行。
    #[inline]
    pub fn update(&mut self, dt: f32) -> bool {
        if self.is_complete() {
            self.value = self.end;
            return false;
        }
        self.elapsed = (self.elapsed + dt.max(0.0)).min(self.duration);
        if self.is_complete() {
            self.value = self.end;
            false
        } else {
            self.value = self.start;
            true
        }
    }

    /// 返回当前布尔值。
    #[inline]
    pub const fn value(&self) -> bool {
        self.value
    }

    /// 返回开关是否已完成。
    #[inline]
    pub fn is_complete(&self) -> bool {
        self.duration == 0.0 || self.elapsed >= self.duration
    }
}

fn spring1(start: f32, target: f32, config: SpringConfig) -> Spring {
    let mut spring = Spring::new(config);
    spring.snap_to(start);
    spring.set_target(target);
    spring
}
