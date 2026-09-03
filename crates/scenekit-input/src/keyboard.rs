/// 键盘修饰键状态。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Modifiers {
    /// Shift 键激活。
    pub shift: bool,
    /// Control 键激活。
    pub ctrl: bool,
    /// Alt 键激活。
    pub alt: bool,
    /// Meta、Command 或 Windows 键激活。
    pub meta: bool,
}

/// scenekit 控制器使用的可移植键码。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum KeyCode {
    /// W 键。
    KeyW = 0,
    /// A 键。
    KeyA = 1,
    /// S 键。
    KeyS = 2,
    /// D 键。
    KeyD = 3,
    /// Q 键。
    KeyQ = 4,
    /// E 键。
    KeyE = 5,
    /// 空格键。
    Space = 6,
    /// 左 Shift 键。
    ShiftLeft = 7,
    /// 右 Shift 键。
    ShiftRight = 8,
    /// 左 Control 键。
    ControlLeft = 9,
    /// 右 Control 键。
    ControlRight = 10,
    /// 左 Alt 键。
    AltLeft = 11,
    /// 右 Alt 键。
    AltRight = 12,
    /// 左 Meta 键。
    MetaLeft = 13,
    /// 右 Meta 键。
    MetaRight = 14,
    /// 上方向键。
    ArrowUp = 15,
    /// 下方向键。
    ArrowDown = 16,
    /// 左方向键。
    ArrowLeft = 17,
    /// 右方向键。
    ArrowRight = 18,
    /// Escape 键。
    Escape = 19,
    /// Enter 键。
    Enter = 20,
    /// Tab 键。
    Tab = 21,
}

impl KeyCode {
    #[inline]
    const fn bit(self) -> u128 {
        1_u128 << (self as u8)
    }
}

/// 适用于 `no_std` 的固定大小键盘状态。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyboardState {
    pressed: u128,
    modifiers: Modifiers,
}

impl KeyboardState {
    /// 创建空的键盘状态。
    #[inline]
    pub const fn new() -> Self {
        Self {
            pressed: 0,
            modifiers: Modifiers {
                shift: false,
                ctrl: false,
                alt: false,
                meta: false,
            },
        }
    }

    /// 返回某个键当前是否被按下。
    #[inline]
    pub const fn is_pressed(self, key: KeyCode) -> bool {
        (self.pressed & key.bit()) != 0
    }

    /// 返回当前修饰键状态。
    #[inline]
    pub const fn modifiers(self) -> Modifiers {
        self.modifiers
    }

    /// 将某个键标记为按下。
    #[inline]
    pub fn on_key_down(&mut self, key: KeyCode) {
        self.pressed |= key.bit();
        self.sync_modifier(key, true);
    }

    /// 将某个键标记为释放。
    #[inline]
    pub fn on_key_up(&mut self, key: KeyCode) {
        self.pressed &= !key.bit();
        self.sync_modifier(key, false);
    }

    /// 清除所有按下的键和修饰键状态。
    #[inline]
    pub fn clear(&mut self) {
        self.pressed = 0;
        self.modifiers = Modifiers::default();
    }

    #[inline]
    fn sync_modifier(&mut self, key: KeyCode, pressed: bool) {
        match key {
            KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                self.modifiers.shift = pressed
                    || self.is_pressed(KeyCode::ShiftLeft)
                    || self.is_pressed(KeyCode::ShiftRight);
            }
            KeyCode::ControlLeft | KeyCode::ControlRight => {
                self.modifiers.ctrl = pressed
                    || self.is_pressed(KeyCode::ControlLeft)
                    || self.is_pressed(KeyCode::ControlRight);
            }
            KeyCode::AltLeft | KeyCode::AltRight => {
                self.modifiers.alt = pressed
                    || self.is_pressed(KeyCode::AltLeft)
                    || self.is_pressed(KeyCode::AltRight);
            }
            KeyCode::MetaLeft | KeyCode::MetaRight => {
                self.modifiers.meta = pressed
                    || self.is_pressed(KeyCode::MetaLeft)
                    || self.is_pressed(KeyCode::MetaRight);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_press_release_state_tracking_works() {
        let mut keyboard = KeyboardState::new();
        keyboard.on_key_down(KeyCode::KeyW);
        assert!(keyboard.is_pressed(KeyCode::KeyW));
        keyboard.on_key_up(KeyCode::KeyW);
        assert!(!keyboard.is_pressed(KeyCode::KeyW));
    }

    #[test]
    fn modifiers_track_both_sides() {
        let mut keyboard = KeyboardState::new();
        keyboard.on_key_down(KeyCode::ShiftLeft);
        keyboard.on_key_down(KeyCode::ShiftRight);
        keyboard.on_key_up(KeyCode::ShiftLeft);
        assert!(keyboard.modifiers().shift);
        keyboard.on_key_up(KeyCode::ShiftRight);
        assert!(!keyboard.modifiers().shift);
    }
}
