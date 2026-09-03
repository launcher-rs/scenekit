# `scenekit-light`

## 角色

CPU 灯光描述、阴影设置和光照探针。

## 依赖权重

轻量级 `no_std`；默认外观功能。

## 安装

```toml
[dependencies]
scenekit-light = "1"
```

## 关键公共 API

AmbientLight、DirectionalLight、PointLight、SpotLight、HemisphereLight、LightProbe、ShadowSettings

## 常见用法

```rust
use scenekit_light::AmbientLight;
let light = AmbientLight::default();
# let _ = light;
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)