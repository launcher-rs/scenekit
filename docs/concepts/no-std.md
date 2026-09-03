# no_std

## 目的

在没有标准库依赖的情况下使用 CPU crate。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

在支持的 CPU crate 上禁用默认功能；需要时使用 `libm` 进行数学运算。

## 关键规则

- 渲染器、加载器、后处理和 WASM 不是 `no_std` 目标。
- 在无默认构建中保持 alloc 使用显式。
- 在 CI 中对 CPU crate 运行无默认检查。


## 示例

```toml
scenekit-math = { version = "1", default-features = false, features = ["libm"] }
scenekit-core = { version = "1", default-features = false }
```

## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)