//! 片段循环模式。

/// 动作在片段结束时的循环方式。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoopMode {
    /// 播放一次后停止；动作在最后一帧之后变为 `Finished`。
    Once,
    /// 永远重复，或最多重复 `max` 次迭代（`0` 表示无限制）。
    Repeat { max: u32 },
    /// 正向/反向交替播放，最多 `max` 个半迭代（`0` = 无限制）。
    PingPong { max: u32 },
}

impl LoopMode {
    /// 默认 `Repeat`（无限制）。
    pub const REPEAT: Self = Self::Repeat { max: 0 };
    /// 默认 `PingPong`（无限制）。
    pub const PING_PONG: Self = Self::PingPong { max: 0 };
}

impl Default for LoopMode {
    #[inline]
    fn default() -> Self {
        Self::Once
    }
}

/// 推进动作时钟的结果。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoopAdvance {
    /// `[0, duration]` 范围内的新局部时间。
    pub time: f32,
    /// 播放方向是否已翻转（乒乓转向）。
    pub flipped: bool,
    /// 本次 tick 是否发生了循环回绕。
    pub wrapped: bool,
    /// 新的迭代计数器。
    pub iteration: u32,
    /// 动作是否已完成（Once 耗尽或达到最大迭代次数）。
    pub finished: bool,
}

impl LoopMode {
    /// 在 `duration` 范围内以 `delta` 秒推进时钟。
    ///
    /// 确定性：一次大 delta 中的多次回绕会被完全解析，
    /// 使结果 `iteration` 计数器精确。
    pub fn advance(
        self,
        time: f32,
        delta: f32,
        duration: f32,
        iteration: u32,
        forward: bool,
    ) -> LoopAdvance {
        if duration <= 0.0 {
            return LoopAdvance {
                time: 0.0,
                flipped: false,
                wrapped: false,
                iteration,
                finished: true,
            };
        }
        let mut t = time + if forward { delta } else { -delta };
        let mut it = iteration;
        let mut fwd = forward;
        let mut wrapped = false;
        let mut finished = false;
        let mut guard = 0u32;

        // 解析回绕；限制迭代次数以避免病态无限循环。
        loop {
            guard += 1;
            if guard > 1_000_000 {
                finished = true;
                t = t.clamp(0.0, duration);
                break;
            }
            if t > duration {
                match self {
                    LoopMode::Once => {
                        t = duration;
                        finished = true;
                        break;
                    }
                    LoopMode::Repeat { max } => {
                        t -= duration;
                        it += 1;
                        wrapped = true;
                        if max > 0 && it >= max {
                            t = duration;
                            finished = true;
                            break;
                        }
                    }
                    LoopMode::PingPong { max } => {
                        t = duration - (t - duration);
                        fwd = !fwd;
                        wrapped = true;
                        it += 1;
                        if max > 0 && it >= max {
                            t = 0.0;
                            finished = true;
                            break;
                        }
                    }
                }
            } else if t < 0.0 {
                // 仅在乒乓反向阶段可达。
                t = -t;
                fwd = !fwd;
                wrapped = true;
                match self {
                    LoopMode::PingPong { max } => {
                        it += 1;
                        if max > 0 && it >= max {
                            t = 0.0;
                            finished = true;
                            break;
                        }
                    }
                    _ => {
                        // 不应发生在 Once/Repeat 反向阶段，但进行钳位。
                        t = t.clamp(0.0, duration);
                        break;
                    }
                }
            } else {
                break;
            }
        }

        LoopAdvance {
            time: t,
            flipped: fwd != forward,
            wrapped,
            iteration: it,
            finished,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn once_finishes_at_end() {
        let r = LoopMode::Once.advance(0.8, 0.4, 1.0, 0, true);
        assert!(r.finished);
        assert_eq!(r.time, 1.0);
    }

    #[test]
    fn repeat_wraps_and_counts() {
        let r = LoopMode::REPEAT.advance(0.8, 0.4, 1.0, 0, true);
        assert!(r.wrapped);
        assert_eq!(r.iteration, 1);
        assert!((r.time - 0.2).abs() < 1e-4);
        assert!(!r.finished);
    }

    #[test]
    fn ping_pong_flips_direction() {
        let r = LoopMode::PING_PONG.advance(0.8, 0.4, 1.0, 0, true);
        assert!(r.flipped);
        assert!((r.time - 0.8).abs() < 1e-4);
    }
}
