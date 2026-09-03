# `scenekit-mesh`

## 角色

CPU 几何体缓冲区、基本生成器、实例化、批处理和变形目标。

## 依赖权重

轻量级 `no_std`；默认外观功能。

## 安装

```toml
[dependencies]
scenekit-mesh = "1"
```

## 关键公共 API

Geometry、Mesh、BufferLayout、InstancedMesh、BatchedMesh、MorphTarget、基本生成器

## 常见用法

```rust
use scenekit_mesh::box_geometry;
let cube = box_geometry(1.0, 1.0, 1.0, 1, 1, 1);
# let _ = cube;
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)