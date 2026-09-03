/// 无需分配的最大手柄追踪数量。
pub const MAX_GAMEPADS: usize = 4;

/// 手柄槽位标识符。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GamepadId(pub u8);

/// 标准浏览器/原生手柄映射中的按键。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum GamepadButton {
    South,
    East,
    West,
    North,
    LeftBumper,
    RightBumper,
    LeftTrigger,
    RightTrigger,
    Select,
    Start,
    LeftStick,
    RightStick,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Home,
}

/// 标准浏览器/原生手柄映射中的轴。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum GamepadAxis {
    /// 左摇杆水平轴。
    LeftX,
    /// 左摇杆垂直轴。
    LeftY,
    /// 右摇杆水平轴。
    RightX,
    /// 右摇杆垂直轴。
    RightY,
}

/// 一个标准映射手柄的状态。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GamepadState {
    /// 槽位是否已连接。
    pub connected: bool,
    /// 轴死区（`0..1`）。
    pub dead_zone: f32,
    axes: [f32; 4],
    buttons: [f32; 17],
}

impl GamepadState {
    /// 创建断开连接的状态。
    pub const fn new() -> Self {
        Self {
            connected: false,
            dead_zone: 0.15,
            axes: [0.0; 4],
            buttons: [0.0; 17],
        }
    }

    /// 返回经过死区过滤的轴值（`-1..=1`）。
    pub fn axis(&self, axis: GamepadAxis) -> f32 {
        let value = self.axes[axis as usize];
        let dead_zone = self.dead_zone.clamp(0.0, 0.99);
        if value.abs() <= dead_zone {
            0.0
        } else {
            value.signum() * ((value.abs() - dead_zone) / (1.0 - dead_zone))
        }
    }

    /// 设置原始轴值，钳制到 `-1..=1`。
    pub fn set_axis(&mut self, axis: GamepadAxis, value: f32) {
        self.axes[axis as usize] = value.clamp(-1.0, 1.0);
    }

    /// 返回模拟按键压力值（`0..=1`）。
    pub fn button_value(&self, button: GamepadButton) -> f32 {
        self.buttons[button as usize]
    }

    /// 返回按键是否超过常规按下阈值。
    pub fn is_pressed(&self, button: GamepadButton) -> bool {
        self.button_value(button) >= 0.5
    }

    /// 设置模拟按键压力值，钳制到 `0..=1`。
    pub fn set_button(&mut self, button: GamepadButton, value: f32) {
        self.buttons[button as usize] = value.clamp(0.0, 1.0);
    }

    /// 清除输入值并标记槽位为断开连接。
    pub fn disconnect(&mut self) {
        *self = Self::new();
    }
}

impl Default for GamepadState {
    fn default() -> Self {
        Self::new()
    }
}

/// 固定容量的手柄状态集合。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GamepadStates {
    pads: [GamepadState; MAX_GAMEPADS],
}

impl GamepadStates {
    /// 创建四个断开连接的手柄槽位。
    pub const fn new() -> Self {
        Self {
            pads: [GamepadState::new(); MAX_GAMEPADS],
        }
    }

    /// 返回手柄槽位。
    pub fn get(&self, id: GamepadId) -> Option<&GamepadState> {
        self.pads.get(id.0 as usize)
    }

    /// 返回可变的手柄槽位。
    pub fn get_mut(&mut self, id: GamepadId) -> Option<&mut GamepadState> {
        self.pads.get_mut(id.0 as usize)
    }

    /// 按槽位顺序遍历已连接的手柄。
    pub fn connected(&self) -> impl Iterator<Item = (GamepadId, &GamepadState)> {
        self.pads
            .iter()
            .enumerate()
            .filter(|(_, pad)| pad.connected)
            .map(|(index, pad)| (GamepadId(index as u8), pad))
    }
}

impl Default for GamepadStates {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axis_dead_zone_and_clamping_are_deterministic() {
        let mut pad = GamepadState::new();
        pad.set_axis(GamepadAxis::LeftX, 2.0);
        assert_eq!(pad.axis(GamepadAxis::LeftX), 1.0);
        pad.set_axis(GamepadAxis::LeftX, 0.1);
        assert_eq!(pad.axis(GamepadAxis::LeftX), 0.0);
    }
}
