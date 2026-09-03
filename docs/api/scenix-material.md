# `scenekit-material`

## 角色

无 GPU 的材质描述和管线键。

## 依赖权重

轻量级 `no_std`；默认外观功能。

## 安装

```toml
[dependencies]
scenekit-material = "1"
```

## 关键公共 API

PbrMaterial、PhysicalMaterial、UnlitMaterial、LambertMaterial、ToonMaterial、WireframeMaterial、NormalMaterial、PipelineKey

## 常见用法

```rust
use scenekit_material::PbrMaterial;
let material = PbrMaterial::new();
# let _ = material;
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)