# 功能矩阵

| 功能 | 默认值 | 用途 |
| --- | --- | --- |
| `std` | 是 | CPU crate 的标准库支持。 |
| `scene`、`camera`、`mesh`、`material`、`light`、`texture` | 是 | CPU 场景创作。 |
| `raycaster`、`helpers` | 是 | 拾取和调试辅助几何体。 |
| `interaction` | 否 | 控件、选择、拖动、变换和小工具。 |
| `editor` | 否 | 用于编辑器面向系统的类型化检查器快照。 |
| `egui` | 否 | 用于检查器快照的只读 egui 适配器。 |
| `loader` | 否 | 资产包、资产管理器、glTF 扩展元数据、OBJ/MTL、STL、图像、KTX2、HDR/EXR 加载和导出器。 |
| `renderer` | 否 | `wgpu` 表面和无头渲染。 |
| `post` | 否 | GPU 后处理堆栈；通常与 `renderer` 一起使用。 |
| `animato` | 否 | Animato 1.7 桥接和基于剪辑的动画运行时。 |
| `wasm` | 否 | 浏览器画布包装器、DPR 感知输入映射、WebGPU 路径、WebGL2 完整回退和 WebGL1 精简回退。 |
| `serde` | 否 | 各 crate 支持时的序列化支持。 |


在决定外观 crate 中启用什么时使用此矩阵。对于库，优先使用专注的 crate，仅暴露你自己公共 API 中的功能。
