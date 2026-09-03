# `scenekit-raycaster`

## 角色

BVH 加速拾取、精确网格交叉、选择体积、拖动平面和可逆变换交互。

## 依赖权重

轻量级 `no_std`；默认外观功能。

## 安装

```toml
[dependencies]
scenekit-raycaster = "1"
```

## 关键公共 API

`Raycaster`、`Intersection`、`Bvh`、`GeometryProvider`、`SelectionRect`、`SelectionFrustum`、`SelectionContainment`、`DragPlane`、`DragController` 和 `TransformController`。

## 常见用法

```rust
use scenekit::{PerspectiveCamera, Raycaster, Vec2, Vec3};

let camera = PerspectiveCamera::new(60.0, 1.0, 0.1, 100.0)
    .position(Vec3::new(0.0, 0.0, 4.0))
    .target(Vec3::ZERO);
let ray = Raycaster::from_camera_ndc(&camera, Vec2::ZERO);
# let _ = ray;
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [选择和拖动示例](../examples/selection-and-drag.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)