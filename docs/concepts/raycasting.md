# 光线投射

## 目的

使用 BVH 和精确三角形测试拾取可见场景节点。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

默认外观功能包括 `raycaster`。

## 关键规则

- 在场景或几何体更改后构建 BVH。
- 图层遮罩和可见性控制候选节点。
- 在验证结果时在测试中使用暴力辅助工具。


## 示例

```rust
use scenekit::{PerspectiveCamera, Raycaster, Vec2, Vec3};

let camera = PerspectiveCamera::new(60.0, 1.0, 0.1, 100.0)
    .position(Vec3::new(0.0, 0.0, 4.0))
    .target(Vec3::ZERO);
let ray = Raycaster::from_camera_ndc(&camera, Vec2::ZERO);
# let _ = ray;
```

## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)