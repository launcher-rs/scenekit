//! 混合器中一个片段实例的回放状态。

extern crate alloc;

use alloc::vec::Vec;

use crate::loop_mode::LoopMode;

/// 动作的采样值如何与同一绑定上的其他动作组合。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BlendMode {
    /// 加权平均混合（覆盖）。默认值。
    Normal,
    /// 叠加混合：`result += (sample - reference) * weight`。
    Additive,
}

impl Default for BlendMode {
    #[inline]
    fn default() -> Self {
        Self::Normal
    }
}

/// 动作生命周期状态。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActionState {
    /// 已创建但尚未开始。
    Stopped,
    /// 正在每帧推进。
    Playing,
    /// 已暂停；保持当前局部时间。
    Paused,
    /// 已完成（Once 耗尽或达到最大迭代次数）。
    Finished,
}

impl Default for ActionState {
    #[inline]
    fn default() -> Self {
        Self::Stopped
    }
}

/// 由 [`crate::mixer::AnimationMixer::add_action`] 返回的稳定句柄。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ActionHandle(pub usize);

/// 片段的一个播放实例。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimationAction {
    /// 混合器片段表中的索引。
    pub clip_index: usize,
    /// 当前局部时间，单位为秒。
    pub time: f32,
    /// 每个动作的时间倍率。
    pub time_scale: f32,
    /// `[0, 1]` 范围内的当前混合权重。
    pub weight: f32,
    /// 正在淡入淡出的目标权重（交叉淡入淡出目标）。
    pub(crate) target_weight: f32,
    /// 每秒权重增量（交叉淡入淡出速率）。
    pub(crate) weight_rate: f32,
    /// 循环行为。
    pub loop_mode: LoopMode,
    /// 混合行为。
    pub blend_mode: BlendMode,
    /// 生命周期状态。
    pub state: ActionState,
    /// 已完成的循环迭代次数。
    pub iteration: u32,
    /// 当前播放方向（在乒乓模式中翻转）。
    pub(crate) forward: bool,
    /// 当前遍历中尚未触发的标记索引。
    pending_markers: Vec<usize>,
    /// 片段本地起始偏移（子片段回放窗口起点）。
    pub start: f32,
    /// 片段本地结束偏移（`None` = 片段持续时间）。
    pub end: Option<f32>,
}

impl AnimationAction {
    /// 创建引用 `clip_index` 的已停止动作。
    pub fn new(clip_index: usize) -> Self {
        Self {
            clip_index,
            time: 0.0,
            time_scale: 1.0,
            weight: 1.0,
            target_weight: 1.0,
            weight_rate: 0.0,
            loop_mode: LoopMode::Once,
            blend_mode: BlendMode::Normal,
            state: ActionState::Stopped,
            iteration: 0,
            forward: true,
            pending_markers: Vec::new(),
            start: 0.0,
            end: None,
        }
    }

    /// 从 `time` 开始播放。
    #[inline]
    pub fn play(&mut self, time: f32) {
        self.time = time;
        self.state = ActionState::Playing;
        self.iteration = 0;
        self.forward = true;
    }

    /// 暂停播放（保持局部时间）。
    #[inline]
    pub fn pause(&mut self) {
        if self.state == ActionState::Playing {
            self.state = ActionState::Paused;
        }
    }

    /// 恢复播放。
    #[inline]
    pub fn resume(&mut self) {
        if self.state == ActionState::Paused {
            self.state = ActionState::Playing;
        }
    }

    /// 停止并重置动作。
    #[inline]
    pub fn stop(&mut self) {
        self.state = ActionState::Stopped;
        self.time = 0.0;
        self.iteration = 0;
        self.forward = true;
    }

    /// 设置每个动作的时间倍率。
    #[inline]
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale;
    }

    /// 设置循环模式。
    #[inline]
    pub fn set_loop_mode(&mut self, mode: LoopMode) {
        self.loop_mode = mode;
    }

    /// 设置混合模式。
    #[inline]
    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        self.blend_mode = mode;
    }

    /// 设置片段本地回放窗口 `[start, end]`。
    #[inline]
    pub fn set_window(&mut self, start: f32, end: Option<f32>) {
        self.start = start.max(0.0);
        self.end = end;
    }

    /// 直接设置当前混合权重（钳位到 `[0, 1]`）。
    #[inline]
    pub fn set_weight(&mut self, weight: f32) {
        self.weight = weight.clamp(0.0, 1.0);
        self.target_weight = self.weight;
        self.weight_rate = 0.0;
    }

    /// 在 `duration` 秒内将权重淡入淡出到 `target`（交叉淡入淡出）。
    pub fn fade_to(&mut self, target: f32, duration: f32) {
        self.target_weight = target.clamp(0.0, 1.0);
        if duration > 0.0 {
            self.weight_rate = (self.target_weight - self.weight) / duration;
        } else {
            self.weight = self.target_weight;
            self.weight_rate = 0.0;
        }
    }

    /// 返回动作是否正在播放。
    #[inline]
    pub const fn is_playing(&self) -> bool {
        matches!(self.state, ActionState::Playing)
    }

    /// 返回动作是否已完成。
    #[inline]
    pub const fn is_finished(&self) -> bool {
        matches!(self.state, ActionState::Finished)
    }

    /// 返回当前播放方向。
    #[inline]
    pub const fn forward(&self) -> bool {
        self.forward
    }

    /// 推进权重淡入淡出并返回新权重。
    #[inline]
    pub(crate) fn advance_weight(&mut self, dt: f32) -> f32 {
        if self.weight_rate != 0.0 {
            let next = self.weight + self.weight_rate * dt;
            if (self.weight_rate > 0.0 && next >= self.target_weight)
                || (self.weight_rate < 0.0 && next <= self.target_weight)
            {
                self.weight = self.target_weight;
                self.weight_rate = 0.0;
            } else {
                self.weight = next;
            }
        }
        self.weight
    }

    /// 将待处理标记列表重置为 `[time, end]` 范围内的所有标记。
    pub(crate) fn reset_markers(&mut self, marker_count: usize) {
        self.pending_markers.clear();
        for i in 0..marker_count {
            self.pending_markers.push(i);
        }
    }

    /// 排空时间 `<= time` 的标记，按顺序返回其索引。
    pub(crate) fn drain_markers_until(&mut self, time: f32, marker_times: &[f32]) -> Vec<usize> {
        let mut fired = Vec::new();
        let mut i = 0;
        while i < self.pending_markers.len() {
            let midx = self.pending_markers[i];
            if marker_times.get(midx).is_some_and(|&mt| mt <= time) {
                fired.push(self.pending_markers.swap_remove(i));
            } else {
                i += 1;
            }
        }
        fired
    }
}
