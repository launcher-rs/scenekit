# 功能标志

## 目的

为每个应用或库选择最小的依赖表面。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

所有外观功能都在此处记录。

## 关键规则

- 默认是 CPU 创作加光线投射器/辅助工具。
- 重系统是可选的。
- 仅在需要序列化时转发 `serde`。


## 示例

| 功能 | 默认值 | 用途 |
| --- | --- | --- |
| `std` | 是 | CPU crate 的标准库支持。 |
| `scene`、`camera`、`mesh`、`material`、`light`、`texture` | 是 | CPU 场景创作。 |
| `raycaster`、`helpers` | 是 | 拾取和调试辅助几何体。 |
| `interaction` | 否 | 控件、选择、拖动、变换和小工具。 |
| `editor` | 否 | 用于编辑器面向系统的类型化检查器快照。 |
| `egui` | 否 | 用于检查器快照的只读 egui 适配器。 |
| `loader` | 否 | glTF/GLB、OBJ/MTL、STL、图像、KTX2、HDR/EXR 加载。 |
| `renderer` | 否 | `wgpu` 表面和无头渲染。 |
| `post` | 否 | GPU 后处理堆栈；通常与 `renderer` 一起使用。 |
| `animato` | 否 | Animato 1.7 桥接和基于剪辑的动画运行时。 |
| `wasm` | 否 | 浏览器画布包装器、DPR 感知输入映射、WebGPU 路径、WebGL2 完整回退和 WebGL1 精简回退。 |
| `serde` | 否 | 各 crate 支持时的序列化支持。 |


## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)
