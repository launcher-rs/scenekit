use scenekit_input::{GamepadAxis, GamepadButton, KeyCode, PointerButton, TouchPhase};

/// 将 DOM `KeyboardEvent.code` 字符串映射为 scenekit 键码。
pub fn key_code_from_dom(code: &str) -> Option<KeyCode> {
    Some(match code {
        "KeyW" => KeyCode::KeyW,
        "KeyA" => KeyCode::KeyA,
        "KeyS" => KeyCode::KeyS,
        "KeyD" => KeyCode::KeyD,
        "KeyQ" => KeyCode::KeyQ,
        "KeyE" => KeyCode::KeyE,
        "Space" => KeyCode::Space,
        "ShiftLeft" => KeyCode::ShiftLeft,
        "ShiftRight" => KeyCode::ShiftRight,
        "ControlLeft" => KeyCode::ControlLeft,
        "ControlRight" => KeyCode::ControlRight,
        "AltLeft" => KeyCode::AltLeft,
        "AltRight" => KeyCode::AltRight,
        "MetaLeft" => KeyCode::MetaLeft,
        "MetaRight" => KeyCode::MetaRight,
        "ArrowUp" => KeyCode::ArrowUp,
        "ArrowDown" => KeyCode::ArrowDown,
        "ArrowLeft" => KeyCode::ArrowLeft,
        "ArrowRight" => KeyCode::ArrowRight,
        "Escape" => KeyCode::Escape,
        "Enter" => KeyCode::Enter,
        "Tab" => KeyCode::Tab,
        _ => return None,
    })
}

/// 将 DOM 指针按钮整数映射为 scenekit 指针按钮。
pub const fn pointer_button_from_dom(button: i16) -> Option<PointerButton> {
    match button {
        0 => Some(PointerButton::Left),
        1 => Some(PointerButton::Middle),
        2 => Some(PointerButton::Right),
        3 => Some(PointerButton::Back),
        4 => Some(PointerButton::Forward),
        _ => None,
    }
}

/// 将紧凑浏览器触摸阶段代码（`0..=3`）映射为 scenekit。
pub const fn touch_phase_from_dom(phase: u8) -> Option<TouchPhase> {
    match phase {
        0 => Some(TouchPhase::Started),
        1 => Some(TouchPhase::Moved),
        2 => Some(TouchPhase::Ended),
        3 => Some(TouchPhase::Cancelled),
        _ => None,
    }
}

/// 将标准游戏手柄轴索引映射为 scenekit。
pub const fn gamepad_axis_from_standard(axis: u8) -> Option<GamepadAxis> {
    match axis {
        0 => Some(GamepadAxis::LeftX),
        1 => Some(GamepadAxis::LeftY),
        2 => Some(GamepadAxis::RightX),
        3 => Some(GamepadAxis::RightY),
        _ => None,
    }
}

/// 将标准游戏手柄按钮索引映射为 scenekit。
pub const fn gamepad_button_from_standard(button: u8) -> Option<GamepadButton> {
    Some(match button {
        0 => GamepadButton::South,
        1 => GamepadButton::East,
        2 => GamepadButton::West,
        3 => GamepadButton::North,
        4 => GamepadButton::LeftBumper,
        5 => GamepadButton::RightBumper,
        6 => GamepadButton::LeftTrigger,
        7 => GamepadButton::RightTrigger,
        8 => GamepadButton::Select,
        9 => GamepadButton::Start,
        10 => GamepadButton::LeftStick,
        11 => GamepadButton::RightStick,
        12 => GamepadButton::DPadUp,
        13 => GamepadButton::DPadDown,
        14 => GamepadButton::DPadLeft,
        15 => GamepadButton::DPadRight,
        16 => GamepadButton::Home,
        _ => return None,
    })
}
