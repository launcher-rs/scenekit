# 使用轨道相机

## 目标

将轨道相机控件附加到指针输入，用于产品查看器和编辑器。

## 相关功能标志

默认 `camera` 和 `input` 功能。

## 步骤

1. 添加所需的 Cargo 功能。
2. 在调用者拥有的存储中保存 CPU 场景数据。
3. 变换或层次结构编辑后调用 `update_world_transforms()`。
4. 仅在启用这些系统时向可选系统注册资源。

## 示例

```rust
use scenekit::{OrbitController, PointerState, Vec3};

let mut controller = OrbitController::new(Vec3::ZERO, 4.0);
let pointer = PointerState::default();
controller.update(&pointer, 1.0 / 60.0);
```

## 验证

运行 `cargo run -p scenekit --example orbit_camera`。

## 相关文档

- [快速开始](../quick-start.md)
- [功能标志](../concepts/feature-flags.md)
