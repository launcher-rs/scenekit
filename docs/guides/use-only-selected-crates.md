# 仅使用选定的 Crate

## 目标

对于库和小工具，依赖专注的 crate 而不是外观。

## 相关功能标志

取决于选定的 crate 集。

## 步骤

1. 添加所需的 Cargo 功能。
2. 在调用者拥有的存储中保存 CPU 场景数据。
3. 变换或层次结构编辑后调用 `update_world_transforms()`。
4. 仅在启用这些系统时向可选系统注册资源。

## 示例

```toml
[dependencies]
scenekit-math = "1"
scenekit-core = "1"
scenekit-scene = "1"
scenekit-camera = "1"
```

## 验证

除非它们是公共契约的一部分，否则不要将渲染器、加载器、后处理、Animato 和 WASM 包含在库中。

## 相关文档

- [快速开始](../quick-start.md)
- [功能标志](../concepts/feature-flags.md)
