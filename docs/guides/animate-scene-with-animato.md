# 使用 Animato 动画场景

## 目标

使用 Animato 支持的轨道更新场景节点、相机、材质和骨骼姿势。

## 相关功能标志

`animato`

## 步骤

1. 添加所需的 Cargo 功能。
2. 在调用者拥有的存储中保存 CPU 场景数据。
3. 变换或层次结构编辑后调用 `update_world_transforms()`。
4. 仅在启用这些系统时向可选系统注册资源。

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

## 验证

运行 `cargo run -p scenekit --example animato_integration --features animato`。

## 相关文档

- [快速开始](../quick-start.md)
- [功能标志](../concepts/feature-flags.md)