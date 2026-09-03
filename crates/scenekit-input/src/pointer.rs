use scenekit_math::Vec2;

/// 指针按键标识符。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum PointerButton {
    /// 主按键。
    Left = 0,
    /// 辅助按键。
    Right = 1,
    /// 中间按键。
    Middle = 2,
    /// 附加后退按键。
    Back = 3,
    /// 附加前进按键。
    Forward = 4,
}

impl PointerButton {
    #[inline]
    const fn mask(self) -> u8 {
        1 << (self as u8)
    }
}

/// 当前指针位置、移动和按键状态。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointerState {
    /// 当前指针位置（像素）。
    pub position: Vec2,
    /// 自上次更新以来的移动量。
    pub delta: Vec2,
    /// 按键掩码。
    pub buttons: u8,
}

impl PointerState {
    /// 创建空的指针状态。
    #[inline]
    pub const fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            delta: Vec2::ZERO,
            buttons: 0,
        }
    }

    /// 更新当前位置并计算增量。
    #[inline]
    pub fn set_position(&mut self, position: Vec2) {
        self.delta = position - self.position;
        self.position = position;
    }

    /// 清除累积的增量。
    #[inline]
    pub fn clear_delta(&mut self) {
        self.delta = Vec2::ZERO;
    }

    /// 将某个按键标记为按下。
    #[inline]
    pub fn on_button_down(&mut self, button: PointerButton) {
        self.buttons |= button.mask();
    }

    /// 将某个按键标记为释放。
    #[inline]
    pub fn on_button_up(&mut self, button: PointerButton) {
        self.buttons &= !button.mask();
    }

    /// 返回某个按键当前是否被按下。
    #[inline]
    pub const fn is_pressed(self, button: PointerButton) -> bool {
        (self.buttons & button.mask()) != 0
    }

    /// 返回是否有任何按键被按下。
    #[inline]
    pub const fn any_pressed(self) -> bool {
        self.buttons != 0
    }
}

impl Default for PointerState {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
