# 后处理

## 目的

应用全屏泛光、SSAO、色调映射、抗锯齿、雾、轮廓、景深和运动模糊效果。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

启用 `renderer` 和 `post`。

## 关键规则

- 后处理效果是 GPU 通道。
- 渲染器集成保持公共渲染签名稳定。
- 仅使用适合目标平台预算的效果。


## 示例

```rust
use scenekit::{PostStack, ToneMapper};

let stack = PostStack::new().with_tonemap(ToneMapper::Aces);
# let _ = stack;
```

## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)