# BVH 光线投射

## 目标

仅在相关的场景或几何体更改后重建 BVH，并在帧之间重用光线投射器状态。

## 先测量

使用专注的命令，一次比较一个更改。当只有一个二进制文件或示例需要重型功能时，避免全局启用它们。

## 命令或模式

```rust
use scenekit::{PerspectiveCamera, Raycaster, Vec2, Vec3};

let camera = PerspectiveCamera::new(60.0, 1.0, 0.1, 100.0)
    .position(Vec3::new(0.0, 0.0, 4.0))
    .target(Vec3::ZERO);
let ray = Raycaster::from_camera_ndc(&camera, Vec2::ZERO);
# let _ = ray;
```

## 实际检查

- 保持仅 CPU crate 轻量级。
- 除非输入发生变化，否则避免每帧重建数据结构。
- 分析时分离加载、注册、更新和渲染成本。