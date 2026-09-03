# 使用光线投射器拾取对象

## 目标

将相机坐标转换为光线并测试场景网格。

## 相关功能标志

默认 `raycaster`。

## 步骤

1. 添加所需的 Cargo 功能。
2. 在调用者拥有的存储中保存 CPU 场景数据。
3. 变换或层次结构编辑后调用 `update_world_transforms()`。
4. 仅在启用这些系统时向可选系统注册资源。

## 示例

```rust
use scenekit::{PerspectiveCamera, Raycaster, Vec2, Vec3};

let camera = PerspectiveCamera::new(60.0, 1.0, 0.1, 100.0)
    .position(Vec3::new(0.0, 0.0, 4.0))
    .target(Vec3::ZERO);
let ray = Raycaster::from_camera_ndc(&camera, Vec2::ZERO);
# let _ = ray;
```

## 验证

运行 `cargo run -p scenekit --example raycasting`。

## 相关文档

- [快速开始](../quick-start.md)
- [功能标志](../concepts/feature-flags.md)