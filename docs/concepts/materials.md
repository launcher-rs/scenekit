# 材质

## 目的

使用无 GPU 的材质描述和渲染器材质注册。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

默认外观功能包括 `material`；渲染需要 `renderer`。

## 关键规则

- PBR 是标准材质路径。
- 物理、卡通、法线、线框、无光照和 Lambert 材质涵盖 v1 示例。
- v1.2 渲染器注册为活动 GPU 路径上传材质统一变量和绑定纹理。
- 高级物理着色是实时近似，而不是离线电影渲染器。


## 示例

```rust
use scenekit::{Color, PbrMaterial};

let material = PbrMaterial::new().albedo(Color::rgb(0.2, 0.8, 0.7));
# let _ = material;
```

## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)