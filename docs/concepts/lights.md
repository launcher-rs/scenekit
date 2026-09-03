# 灯光

## 目的

添加环境光、方向光、点光源、聚光灯、区域光、半球光和探针光数据。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

默认外观功能包括 `light`；阴影需要 `renderer`。

## 关键规则

- 灯光是 CPU 描述。
- 渲染器注册上传灯光数据。
- 阴影设置与灯光配置一起。


## 示例

```rust
use scenekit::{DirectionalLight, Vec3};

let light = DirectionalLight::new(Vec3::new(-1.0, -2.0, -1.0));
# let _ = light;
```

## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)