# `scenekit-input`

## 角色

平台中立的键盘、指针、触摸、手势、游戏手柄、指针锁和视口状态。

## 依赖权重

轻量级 `no_std`；与相机动画器和 WASM 输入映射一起使用。

## 安装

```toml
[dependencies]
scenekit-input = "1"
```

## 关键公共 API

`InputState`、`KeyboardState`、`PointerState`、`TouchState`、`GestureState`、`GamepadStates`、`PointerLockState`、`ViewportMetrics`、`KeyCode` 和 `PointerButton`。

## 常见用法

```rust
use scenekit_input::{InputState, PointerButton};
use scenekit_math::Vec2;

let mut input = InputState::default();
input.on_pointer_down(PointerButton::Left);
input.on_pointer_move(Vec2::new(20.0, 8.0));
assert!(input.was_pointer_pressed(PointerButton::Left));
input.end_frame();
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [交互和编辑器原语](../concepts/interaction-and-editor.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)