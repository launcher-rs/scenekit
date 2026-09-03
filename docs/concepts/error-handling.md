# 错误处理

## 目的

一致地处理验证、加载器、渲染器和 GPU 故障。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

核心错误不需要特殊功能；子系统错误随其 crate 出现。

## 关键规则

- 构造函数在可能的情况下验证维度、字节大小和 ID。
- 加载器错误区分不支持的资产和 IO/解码失败。
- 渲染器错误应呈现给 UI 或测试输出。


## 示例

```rust
use scenekit::ScenixError;

fn report(error: ScenixError) {
    eprintln!("scenekit error: {error}");
}
```

## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)