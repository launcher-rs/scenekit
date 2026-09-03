# 变换

## 目的

处理局部变换、世界变换和父子传播。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

默认外观功能包括 `scene` 和 `math`。

## 关键规则

- 局部变换描述节点相对于其父节点的变换。
- 世界变换由场景图缓存。
- 脏根在传播前去重。


## 示例

```rust
use scenekit::{SceneGraph, Transform, Vec3};

let mut scene = SceneGraph::new();
let transform = Transform::from_translation(Vec3::new(1.0, 0.0, 0.0));
# let _ = (&mut scene, transform);
```

## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)
