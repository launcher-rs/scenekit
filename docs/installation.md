# 安装

本页说明了依赖 scenekit v0.1.0 的支持方式。对于应用程序代码使用外观 crate，对于构建库或非常小的工具时使用专注的 crate。

## 外观安装

```toml
[dependencies]
scenekit = "1"
```

默认外观以 CPU 为先。除非启用这些功能，否则它不会引入 loader、renderer、后处理、Animato 或浏览器依赖项。

## 可选系统

```toml
[dependencies]
scenekit = { version = "0.1", features = ["loader"] }
scenekit = { version = "0.1", features = ["renderer"] }
scenekit = { version = "0.1", features = ["renderer", "post"] }
scenekit = { version = "0.1", features = ["animato"] }
scenekit = { version = "0.1", features = ["wasm"] }
scenekit = { version = "0.1", features = ["interaction"] }
scenekit = { version = "0.1", features = ["editor", "egui"] }
```

## 选定的 Crate

```toml
[dependencies]
scenekit-math = "1"
scenekit-scene = "1"
scenekit-camera = "1"
scenekit-mesh = "1"
scenekit-raycaster = "1"
```

对于不应暴露完整外观依赖表面的库，使用选定的 crate。

## no_std CPU 设置

```toml
[dependencies]
scenekit-math = { version = "0.1", default-features = false, features = ["libm"] }
scenekit-core = { version = "0.1", default-features = false }
scenekit-scene = { version = "0.1", default-features = false }
scenekit-camera = { version = "0.1", default-features = false }
```

加载器、渲染器、后处理和 WASM 路径面向 `std`。

## 功能矩阵

| 功能 | 默认值 | 用途 |
| --- | --- | --- |
| `std` | 是 | CPU crate 的标准库支持。 |
| `scene`、`camera`、`mesh`、`material`、`light`、`texture` | 是 | CPU 场景创作。 |
| `raycaster`、`helpers` | 是 | 拾取和调试辅助几何体。 |
| `interaction` | 否 | 控件、选择、拖动、变换和小工具。 |
| `editor` | 否 | 用于编辑器面向系统的类型化检查器快照。 |
| `egui` | 否 | 用于检查器快照的只读 egui 适配器。 |
| `loader` | 否 | 资产包、资产管理器、glTF 扩展元数据、导入器和导出器。 |
| `renderer` | 否 | `wgpu` 表面和无头渲染。 |
| `post` | 否 | GPU 后处理堆栈；通常与 `renderer` 一起使用。 |
| `animato` | 否 | Animato 1.7 桥接和基于剪辑的动画运行时。 |
| `wasm` | 否 | 浏览器画布包装器、DOM 输入映射、WebGPU 路径、WebGL2 完整回退和 WebGL1 精简回退。 |
| `serde` | 否 | 各 crate 支持时的序列化支持。 |
