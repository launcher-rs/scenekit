# `scenekit-camera`

## 角色

透视、正交、立方体相机、视锥体和控件。

## 依赖权重

轻量级 `no_std`；默认外观功能。

## 安装

```toml
[dependencies]
scenekit-camera = "1"
```

## 关键公共 API

`PerspectiveCamera`、`OrthographicCamera`、`CubeCamera`、`Frustum`、`OrbitController`、`FlyController`、`ArcballController`、`TrackballController`、`MapController`、`FirstPersonController` 和 `PointerLockController`。

## 常见用法

```rust
use scenekit_camera::{ArcballController, PerspectiveCamera};
use scenekit_input::InputState;
use scenekit_math::Vec3;

let input = InputState::default();
let mut controls = ArcballController::new(Vec3::ZERO, 5.0);
let mut camera = PerspectiveCamera::default();
controls.update_from_input(&input, 1.0 / 60.0);
controls.apply_to_perspective(&mut camera);
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [交互和编辑器原语](../concepts/interaction-and-editor.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)