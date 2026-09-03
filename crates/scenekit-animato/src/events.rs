//! 混合器发出的动画事件。

use alloc::string::String;
use alloc::vec::Vec;

/// 推进混合器时产生的离散事件。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AnimationEvent {
    /// 动作完成了一次循环回绕。
    Loop {
        /// 动作的槽位索引。
        action: usize,
        /// 新的迭代计数器。
        iteration: u32,
    },
    /// 动作越过了一个命名标记。
    Marker {
        /// 动作的槽位索引。
        action: usize,
        /// 标记名称。
        name: String,
    },
    /// 动作已完成（Once 耗尽或达到最大迭代次数）。
    Finished {
        /// 动作的槽位索引。
        action: usize,
    },
}

/// 一次 [`crate::mixer::AnimationMixer::tick`] 的聚合结果。
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MixerTickResult {
    /// 本次 tick 触发的事件，按确定性顺序排列。
    pub events: Vec<AnimationEvent>,
    /// 本次 tick 中活跃（正在播放）的动作数。
    pub active_actions: usize,
    /// 本次 tick 中过渡到 `Finished` 状态的动作数。
    pub finished_actions: usize,
}
