use scenekit_math::Vec2;

use crate::{
    GamepadAxis, GamepadButton, GamepadId, GamepadStates, GestureRecognizer, GestureState, KeyCode,
    KeyboardState, PointerButton, PointerState, TouchId, TouchPhase, TouchState, ViewportMetrics,
};

/// 指针锁定状态及本帧累积的相对移动量。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointerLockState {
    /// 平台当前是否拥有指针锁定。
    pub locked: bool,
    /// 锁定时报告的相对移动量。
    pub delta: Vec2,
}

/// 完整的平台无关输入快照。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputState {
    /// 键盘按住状态。
    pub keyboard: KeyboardState,
    /// 指针位置、移动和按住的按键。
    pub pointer: PointerState,
    /// 活跃的触摸接触点。
    pub touches: TouchState,
    /// 当前帧识别的手势。
    pub gesture: GestureState,
    /// 固定手柄槽位。
    pub gamepads: GamepadStates,
    /// 指针锁定状态。
    pub pointer_lock: PointerLockState,
    /// 逻辑和物理视口测量值。
    pub viewport: ViewportMetrics,
    /// 累积的滚轮增量。
    pub scroll_delta: f32,
    gesture_recognizer: GestureRecognizer,
    key_pressed: u128,
    key_released: u128,
    pointer_pressed: u8,
    pointer_released: u8,
}

impl InputState {
    /// 为给定视口创建空的输入快照。
    pub fn new(viewport: ViewportMetrics) -> Self {
        Self {
            keyboard: KeyboardState::new(),
            pointer: PointerState::new(),
            touches: TouchState::new(),
            gesture: GestureState::default(),
            gamepads: GamepadStates::new(),
            pointer_lock: PointerLockState::default(),
            viewport,
            scroll_delta: 0.0,
            gesture_recognizer: GestureRecognizer::new(),
            key_pressed: 0,
            key_released: 0,
            pointer_pressed: 0,
            pointer_released: 0,
        }
    }

    /// 处理键盘按下事件，每帧记录一次状态转换。
    pub fn on_key_down(&mut self, key: KeyCode) {
        if !self.keyboard.is_pressed(key) {
            self.key_pressed |= 1_u128 << key as u8;
        }
        self.keyboard.on_key_down(key);
    }

    /// 处理键盘释放事件。
    pub fn on_key_up(&mut self, key: KeyCode) {
        if self.keyboard.is_pressed(key) {
            self.key_released |= 1_u128 << key as u8;
        }
        self.keyboard.on_key_up(key);
    }

    /// 返回某个键在本帧是否按下。
    pub const fn was_key_pressed(&self, key: KeyCode) -> bool {
        self.key_pressed & (1_u128 << key as u8) != 0
    }

    /// 返回某个键在本帧是否释放。
    pub const fn was_key_released(&self, key: KeyCode) -> bool {
        self.key_released & (1_u128 << key as u8) != 0
    }

    /// 累积绝对逻辑指针移动事件。
    pub fn on_pointer_move(&mut self, position: Vec2) {
        self.pointer.delta += position - self.pointer.position;
        self.pointer.position = position;
    }

    /// 在指针锁定激活时累积相对移动量。
    pub fn on_pointer_motion(&mut self, delta: Vec2) {
        if self.pointer_lock.locked {
            self.pointer_lock.delta += delta;
        }
    }

    /// 处理指针按键按下事件。
    pub fn on_pointer_down(&mut self, button: PointerButton) {
        if !self.pointer.is_pressed(button) {
            self.pointer_pressed |= 1 << button as u8;
        }
        self.pointer.on_button_down(button);
    }

    /// 处理指针按键释放事件。
    pub fn on_pointer_up(&mut self, button: PointerButton) {
        if self.pointer.is_pressed(button) {
            self.pointer_released |= 1 << button as u8;
        }
        self.pointer.on_button_up(button);
    }

    /// 返回某个指针按键在本帧是否按下。
    pub const fn was_pointer_pressed(&self, button: PointerButton) -> bool {
        self.pointer_pressed & (1 << button as u8) != 0
    }

    /// 返回某个指针按键在本帧是否释放。
    pub const fn was_pointer_released(&self, button: PointerButton) -> bool {
        self.pointer_released & (1 << button as u8) != 0
    }

    /// 累积滚轮事件。
    pub fn on_scroll(&mut self, delta: f32) {
        if delta.is_finite() {
            self.scroll_delta += delta;
        }
    }

    /// 处理触摸事件并刷新当前手势。
    pub fn on_touch(
        &mut self,
        id: TouchId,
        phase: TouchPhase,
        position: Vec2,
        pressure: f32,
    ) -> bool {
        let accepted = self.touches.on_event(id, phase, position, pressure);
        if accepted {
            self.gesture = self.gesture_recognizer.update(&self.touches);
        }
        accepted
    }

    /// 设置指针锁定状态并清除过时的相对移动量。
    pub fn set_pointer_locked(&mut self, locked: bool) {
        if self.pointer_lock.locked != locked {
            self.pointer_lock.delta = Vec2::ZERO;
        }
        self.pointer_lock.locked = locked;
    }

    /// 设置一个手柄的连接状态。
    pub fn set_gamepad_connected(&mut self, id: GamepadId, connected: bool) -> bool {
        let Some(pad) = self.gamepads.get_mut(id) else {
            return false;
        };
        if connected {
            pad.connected = true;
        } else {
            pad.disconnect();
        }
        true
    }

    /// 设置一个标准手柄轴。
    pub fn set_gamepad_axis(&mut self, id: GamepadId, axis: GamepadAxis, value: f32) -> bool {
        let Some(pad) = self.gamepads.get_mut(id) else {
            return false;
        };
        pad.set_axis(axis, value);
        true
    }

    /// 设置一个标准手柄按键。
    pub fn set_gamepad_button(&mut self, id: GamepadId, button: GamepadButton, value: f32) -> bool {
        let Some(pad) = self.gamepads.get_mut(id) else {
            return false;
        };
        pad.set_button(button, value);
        true
    }

    /// 清除瞬态增量和转换，同时保留按住状态。
    pub fn end_frame(&mut self) {
        self.pointer.clear_delta();
        self.touches.end_frame();
        self.gesture.clear();
        self.pointer_lock.delta = Vec2::ZERO;
        self.scroll_delta = 0.0;
        self.key_pressed = 0;
        self.key_released = 0;
        self.pointer_pressed = 0;
        self.pointer_released = 0;
    }

    /// 清除所有按住和瞬态输入，例如焦点丢失后。
    pub fn clear(&mut self) {
        let viewport = self.viewport;
        *self = Self::new(viewport);
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new(ViewportMetrics::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn end_frame_preserves_held_state_only() {
        let mut input = InputState::default();
        input.on_key_down(KeyCode::KeyW);
        input.on_pointer_down(PointerButton::Left);
        input.on_pointer_move(Vec2::new(4.0, 5.0));
        input.on_scroll(2.0);
        assert!(input.was_key_pressed(KeyCode::KeyW));
        input.end_frame();
        assert!(input.keyboard.is_pressed(KeyCode::KeyW));
        assert!(input.pointer.is_pressed(PointerButton::Left));
        assert_eq!(input.pointer.delta, Vec2::ZERO);
        assert_eq!(input.scroll_delta, 0.0);
        assert!(!input.was_key_pressed(KeyCode::KeyW));
    }

    #[test]
    fn pointer_lock_gates_relative_movement() {
        let mut input = InputState::default();
        input.on_pointer_motion(Vec2::ONE);
        assert_eq!(input.pointer_lock.delta, Vec2::ZERO);
        input.set_pointer_locked(true);
        input.on_pointer_motion(Vec2::ONE);
        assert_eq!(input.pointer_lock.delta, Vec2::ONE);
    }
}
