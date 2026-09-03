# 使用 Animato 动画

## 目的

通过可选的 Animato 桥接驱动场景、相机、材质和骨骼值。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

启用 `animato`。Scenix v1.5.0 目标为 Animato `1.7.0`，包括过程式缓动/弹簧桥接和基于剪辑的动画运行时。

## 关键规则

- 驱动程序按确定性动画器列表进行。
- 场景动画器目标变换和可见性。
- 材质和相机动画器使用存储 trait。


## 示例

```rust
use scenekit::{NodeAnimationTarget, NodeAnimator, ScenixAnimationDriver, Vec3, Vec3Track};

let mut driver = ScenixAnimationDriver::new();
driver.add_node(NodeAnimator::new(
    scenekit::NodeId::new(1),
    NodeAnimationTarget::Translation(Vec3Track::tween(
        Vec3::ZERO,
        Vec3::new(1.0, 0.0, 0.0),
        0.5,
    )),
));
```

## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)