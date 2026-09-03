# `scenekit-math`

## 角色

向量、矩阵、四元数、变换、射线、平面、边界和坐标辅助工具。

## 依赖权重

轻量级 `no_std`；没有 `std` 时使用 `libm`。

## 安装

```toml
[dependencies]
scenekit-math = "1"
```

## 关键公共 API

Vec2、Vec3、Vec4、Mat4、Quat、Transform、Ray3、Aabb、Sphere、Plane

## 常见用法

```rust
use scenekit_math::{Transform, Vec3};
let t = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
# let _ = t;
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)