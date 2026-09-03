# 动画相机路径

## 使用场景

当你需要此行为在应用程序中并想要支持它的最小 Scenix 子系统集时。

## 方法

使用 `CameraAnimator` 或来自 Animato 桥接的相机动画器存储 trait 进行确定性相机移动。

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

围绕上述状态更改或命令添加专注测试。对于浏览器或 GPU 路径，保持测试门控以便正常 CPU CI 保持快速。