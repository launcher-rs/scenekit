use scenekit_math::Vec2;

use crate::TouchState;

/// 当前帧累积的手势增量。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GestureState {
    /// 接触点质心的移动量（逻辑像素）。
    pub pan_delta: Vec2,
    /// 相对捏合变化量；`0` 为静止，`0.1` 表示大百分之十。
    pub pinch_delta: f32,
    /// 带符号的双指旋转（弧度）。
    pub rotation_delta: f32,
    /// 识别器使用的接触点数。
    pub contact_count: u8,
}

impl GestureState {
    /// 清除瞬态手势增量。
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// 有状态、无分配的触摸手势识别器。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GestureRecognizer {
    previous_centroid: Option<Vec2>,
    previous_vector: Option<Vec2>,
}

impl GestureRecognizer {
    /// 创建重置后的识别器。
    pub const fn new() -> Self {
        Self {
            previous_centroid: None,
            previous_vector: None,
        }
    }

    /// 从当前活跃触摸更新手势。
    pub fn update(&mut self, touches: &TouchState) -> GestureState {
        let mut iter = touches.iter();
        let Some(first) = iter.next() else {
            self.reset();
            return GestureState::default();
        };
        let second = iter.next();
        let (centroid, vector, count) = if let Some(second) = second {
            (
                (first.position + second.position) * 0.5,
                Some(second.position - first.position),
                2,
            )
        } else {
            (first.position, None, 1)
        };

        let pan_delta = self
            .previous_centroid
            .map_or(Vec2::ZERO, |previous| centroid - previous);
        let mut pinch_delta = 0.0;
        let mut rotation_delta = 0.0;
        if let (Some(previous), Some(current)) = (self.previous_vector, vector) {
            let previous_length = previous.length();
            if previous_length > 1.0e-6 {
                pinch_delta = current.length() / previous_length - 1.0;
            }
            if previous.length_squared() > 1.0e-12 && current.length_squared() > 1.0e-12 {
                let angle = previous.angle_between(current);
                let cross = previous.x * current.y - previous.y * current.x;
                rotation_delta = angle * cross.signum();
            }
        }

        self.previous_centroid = Some(centroid);
        self.previous_vector = vector;
        GestureState {
            pan_delta,
            pinch_delta,
            rotation_delta,
            contact_count: count,
        }
    }

    /// 忘记之前的触摸配置。
    pub fn reset(&mut self) {
        self.previous_centroid = None;
        self.previous_vector = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TouchId, TouchPhase};

    #[test]
    fn recognizes_pan_pinch_and_rotation() {
        let mut touches = TouchState::new();
        let mut recognizer = GestureRecognizer::new();
        touches.on_event(TouchId(1), TouchPhase::Started, Vec2::new(-1.0, 0.0), 1.0);
        touches.on_event(TouchId(2), TouchPhase::Started, Vec2::new(1.0, 0.0), 1.0);
        assert_eq!(recognizer.update(&touches).contact_count, 2);
        touches.on_event(TouchId(1), TouchPhase::Moved, Vec2::new(0.0, -2.0), 1.0);
        touches.on_event(TouchId(2), TouchPhase::Moved, Vec2::new(0.0, 2.0), 1.0);
        let gesture = recognizer.update(&touches);
        assert!(gesture.pinch_delta > 0.9);
        assert!(gesture.rotation_delta > 1.5);
    }
}
