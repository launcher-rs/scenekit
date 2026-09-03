use scenekit_math::Vec2;

/// 无需分配的最大同时触摸接触点数。
pub const MAX_TOUCH_POINTS: usize = 10;

/// 平台提供的触摸接触标识符。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TouchId(pub u64);

/// 触摸事件的阶段。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TouchPhase {
    /// 接触开始。
    Started,
    /// 接触移动。
    Moved,
    /// 接触正常结束。
    Ended,
    /// 接触被平台取消。
    Cancelled,
}

/// 一个活跃的触摸接触点（逻辑像素）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TouchPoint {
    /// 平台接触标识符。
    pub id: TouchId,
    /// 当前逻辑位置。
    pub position: Vec2,
    /// 当前帧内累积的移动量。
    pub delta: Vec2,
    /// 可用时的归一化压力值（`0..=1`）。
    pub pressure: f32,
}

/// 适用于 `no_std` 输入循环的固定容量触摸状态。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TouchState {
    points: [Option<TouchPoint>; MAX_TOUCH_POINTS],
    len: u8,
}

impl TouchState {
    /// 创建空的触摸状态。
    pub const fn new() -> Self {
        Self {
            points: [None; MAX_TOUCH_POINTS],
            len: 0,
        }
    }

    /// 活跃接触点数量。
    #[inline]
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    /// 返回是否没有活跃的接触点。
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 返回指定 ID 的接触点。
    pub fn get(&self, id: TouchId) -> Option<&TouchPoint> {
        self.points.iter().flatten().find(|point| point.id == id)
    }

    /// 按稳定槽位顺序返回活跃接触点。
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &TouchPoint> {
        self.points.iter().flatten()
    }

    /// 处理平台触摸事件。仅在固定槽位已满且无法存储新接触点时返回 `false`。
    pub fn on_event(
        &mut self,
        id: TouchId,
        phase: TouchPhase,
        position: Vec2,
        pressure: f32,
    ) -> bool {
        let pressure = pressure.clamp(0.0, 1.0);
        let existing = self
            .points
            .iter()
            .position(|point| point.is_some_and(|point| point.id == id));

        match phase {
            TouchPhase::Started => {
                if let Some(index) = existing {
                    self.points[index] = Some(TouchPoint {
                        id,
                        position,
                        delta: Vec2::ZERO,
                        pressure,
                    });
                    return true;
                }
                let Some(index) = self.points.iter().position(Option::is_none) else {
                    return false;
                };
                self.points[index] = Some(TouchPoint {
                    id,
                    position,
                    delta: Vec2::ZERO,
                    pressure,
                });
                self.len += 1;
                true
            }
            TouchPhase::Moved => {
                let Some(index) = existing else {
                    return false;
                };
                let point = self.points[index].as_mut().expect("occupied touch slot");
                point.delta += position - point.position;
                point.position = position;
                point.pressure = pressure;
                true
            }
            TouchPhase::Ended | TouchPhase::Cancelled => {
                let Some(index) = existing else {
                    return false;
                };
                self.points[index] = None;
                self.len -= 1;
                true
            }
        }
    }

    /// 清除每帧移动量，同时保留活跃接触点。
    pub fn end_frame(&mut self) {
        for point in self.points.iter_mut().flatten() {
            point.delta = Vec2::ZERO;
        }
    }

    /// 取消所有活跃接触点。
    pub fn cancel_all(&mut self) {
        self.points = [None; MAX_TOUCH_POINTS];
        self.len = 0;
    }
}

impl Default for TouchState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touch_lifecycle_accumulates_delta_and_reuses_slots() {
        let mut state = TouchState::new();
        assert!(state.on_event(TouchId(7), TouchPhase::Started, Vec2::new(1.0, 2.0), 2.0));
        assert!(state.on_event(TouchId(7), TouchPhase::Moved, Vec2::new(4.0, 6.0), 0.5));
        assert_eq!(state.get(TouchId(7)).unwrap().delta, Vec2::new(3.0, 4.0));
        assert_eq!(state.get(TouchId(7)).unwrap().pressure, 0.5);
        state.end_frame();
        assert_eq!(state.get(TouchId(7)).unwrap().delta, Vec2::ZERO);
        assert!(state.on_event(TouchId(7), TouchPhase::Ended, Vec2::ZERO, 0.0));
        assert!(state.is_empty());
    }

    #[test]
    fn capacity_is_bounded_and_cancel_clears_everything() {
        let mut state = TouchState::new();
        for id in 0..MAX_TOUCH_POINTS as u64 {
            assert!(state.on_event(TouchId(id), TouchPhase::Started, Vec2::ZERO, 0.0));
        }
        assert!(!state.on_event(TouchId(99), TouchPhase::Started, Vec2::ZERO, 0.0));
        state.cancel_all();
        assert!(state.is_empty());
    }
}
