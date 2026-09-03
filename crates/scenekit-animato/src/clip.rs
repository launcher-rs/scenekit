//! 片段数据模型：一组命名的关键帧通道 + 标记。

use alloc::string::String;
use alloc::vec::Vec;

use crate::binding::PropertyBinding;
use crate::keyframe::{KeyframeBool, KeyframeColor, KeyframeQuat, KeyframeScalar, KeyframeVec3};

/// 片段通道携带的一种关键帧轨道变体。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClipTrack {
    /// 标量关键帧轨道（变形权重、强度、不透明度等）。
    Scalar(KeyframeScalar),
    /// 三维向量关键帧轨道（平移、缩放、自发光等）。
    Vec3(KeyframeVec3),
    /// 四元数关键帧轨道（旋转）。
    Quat(KeyframeQuat),
    /// 颜色关键帧轨道（反照率、灯光颜色等）。
    Color(KeyframeColor),
    /// 布尔关键帧轨道（可见性）。
    Bool(KeyframeBool),
}

impl ClipTrack {
    /// 返回轨道持续时间（最后一个关键帧时间）。
    #[inline]
    pub fn duration(&self) -> f32 {
        match self {
            Self::Scalar(t) => t.duration(),
            Self::Vec3(t) => t.duration(),
            Self::Quat(t) => t.duration(),
            Self::Color(t) => t.duration(),
            Self::Bool(t) => t.duration(),
        }
    }
}

/// 单个动画通道：一个绑定 + 一个关键帧轨道。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClipChannel {
    /// 采样值写入的位置。
    pub binding: PropertyBinding,
    /// 在片段本地时间处采样的关键帧轨道。
    pub track: ClipTrack,
}

/// 片段内的命名时间标记（类似 Three.js `AnimationClip`）。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimationMarker {
    /// 标记标签。
    pub name: String,
    /// 片段本地时间，单位为秒。
    pub time: f32,
}

impl AnimationMarker {
    /// 创建标记。
    #[inline]
    pub fn new(name: impl Into<String>, time: f32) -> Self {
        Self {
            name: name.into(),
            time: time.max(0.0),
        }
    }
}

/// 可播放的动画片段。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimationClip {
    /// 人类可读的片段名称。
    pub name: String,
    /// 片段持续时间，单位为秒（`max(channel.track.duration())`）。
    pub duration: f32,
    /// 按确定性顺序排列的通道。
    pub channels: Vec<ClipChannel>,
    /// 命名时间标记。
    pub markers: Vec<AnimationMarker>,
}

impl AnimationClip {
    /// 创建空的命名片段。
    pub fn empty(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            duration: 0.0,
            channels: Vec::new(),
            markers: Vec::new(),
        }
    }

    /// 构建器：添加通道并在需要时延长片段持续时间。
    pub fn with_channel(mut self, channel: ClipChannel) -> Self {
        self.duration = self.duration.max(channel.track.duration());
        self.channels.push(channel);
        self
    }

    /// 构建器：添加标记。
    pub fn with_marker(mut self, marker: AnimationMarker) -> Self {
        self.duration = self.duration.max(marker.time);
        self.markers.push(marker);
        self
    }

    /// 从通道 + 标记重新计算持续时间。
    pub fn recompute_duration(&mut self) {
        let mut d = 0.0_f32;
        for ch in &self.channels {
            d = d.max(ch.track.duration());
        }
        for m in &self.markers {
            d = d.max(m.time);
        }
        self.duration = d;
    }
}
