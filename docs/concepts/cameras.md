# 相机

## 目的

选择透视、正交、立方体、轨道和飞行相机工具。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

默认外观功能包括 `camera`。

## 关键规则

- 透视相机常用于交互式场景。
- 正交相机适用于编辑器和技术视图。
- 视锥体支持可见性测试和辅助工具。


## 示例

```rust
use scenekit::{OrbitController, PerspectiveCamera, Vec3};

let camera = PerspectiveCamera::new(60.0, 1.0, 0.1, 100.0)
    .position(Vec3::new(0.0, 2.0, 5.0))
    .target(Vec3::ZERO);
let controller = OrbitController::new(Vec3::ZERO, 5.0);
# let _ = (camera, controller);
```

## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)