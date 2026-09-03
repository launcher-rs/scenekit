# 辅助工具

## 目的

生成调试线几何体，用于网格、轴、边界、相机、灯光、箭头和骨骼。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

默认外观功能包括 `helpers`。

## 关键规则

- 辅助工具返回 `LineGeometry`。
- 它们不需要渲染器。
- 将它们用于编辑器叠加层和诊断。


## 示例

```rust
use scenekit::GridHelper;

let grid = GridHelper::new(10, 1.0).geometry();
# let _ = grid;
```

## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)
