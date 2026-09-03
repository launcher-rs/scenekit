# `scenekit-animato`

## 角色

用于场景、相机、材质和骨骼动画的可选 Animato 桥接。

## 依赖权重

可选 `std` 路径；在外观上启用 `animato`。

## 安装

```toml
[dependencies]
scenekit-animato = "1"
```

## 关键公共 API

AnimVec3、AnimQuat、Vec3Track、QuatTrack、NodeAnimator、CameraAnimator、MaterialAnimator、SkeletonPose、ScenixAnimationDriver

## 常见用法

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

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)