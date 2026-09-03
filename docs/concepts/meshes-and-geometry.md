# 网格和几何体

## 目的

创建 CPU 顶点/索引数据和基本几何体。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

默认外观功能包括 `mesh`。

## 关键规则

- 几何体在注册到渲染器之前是 CPU 数据。
- 使用基本生成器进行示例和测试。
- 对于重复几何体使用实例化或批处理。


## 示例

```rust
use scenekit::box_geometry;

let cube = box_geometry(1.0, 1.0, 1.0, 1, 1, 1);
assert!(!cube.positions.is_empty());
```

## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)
