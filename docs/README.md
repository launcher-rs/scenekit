# scenekit 开发者文档

scenekit v0.1.0 是一个模块化的 Rust 原生 3D 场景工作区。本文档为需要选择 crate、创建场景、添加交互/编辑器原语、渲染或加载资源、集成 Animato、目标 WASM 和调试性能的应用开发者编写。

## 从这里开始

- [快速入门](getting-started.md) - 从安装到场景的最短路径。
- [安装](installation.md) - 外观、选定的 crate、可选功能和 `no_std` 设置。
- [快速开始](quick-start.md) - 可复制的场景、相机、光线投射和渲染器代码片段。
- [项目设置](project-setup.md) - 推荐的应用程序布局和检查命令。

## 概念

- [概念概述](concepts/README.md)
- [架构概述](concepts/architecture-overview.md)
- [场景图](concepts/scene-graph.md)
- [变换](concepts/transforms.md)
- [相机](concepts/cameras.md)
- [网格和几何体](concepts/meshes-and-geometry.md)
- [材质](concepts/materials.md)
- [灯光](concepts/lights.md)
- [纹理](concepts/textures.md)
- [渲染器](concepts/renderer.md)
- [后处理](concepts/post-processing.md)
- [光线投射](concepts/raycasting.md)
- [交互和编辑器原语](concepts/interaction-and-editor.md)
- [辅助工具](concepts/helpers.md)
- [使用 Animato 动画](concepts/animation-with-animato.md)
- [WASM 和浏览器](concepts/wasm-and-browser.md)
- [功能标志](concepts/feature-flags.md)
- [no_std](concepts/no-std.md)
- [错误处理](concepts/error-handling.md)

## 指南

- [创建你的第一个场景](guides/create-your-first-scene.md)
- [渲染一个立方体](guides/render-a-cube.md)
- [使用轨道相机](guides/use-orbit-camera.md)
- [加载 glTF 模型](guides/load-gltf-model.md)
- [创建 PBR 材质](guides/create-pbr-material.md)
- [添加灯光和阴影](guides/add-lights-and-shadows.md)
- [使用后处理](guides/use-post-processing.md)
- [使用光线投射器拾取对象](guides/pick-objects-with-raycaster.md)
- [使用 Animato 动画场景](guides/animate-scene-with-animato.md)
- [为 WASM 构建](guides/build-for-wasm.md)
- [部署到 GitHub Pages](guides/deploy-to-github-pages.md)
- [优化大型场景](guides/optimize-large-scenes.md)
- [仅使用选定的 crate](guides/use-only-selected-crates.md)

## 参考部分

- [按 crate 的 API](api/facade-crate.md)
- [示例](examples/README.md)
- [实践方案](recipes/README.md)
- [性能](performance/README.md)
- [部署](deployment/README.md)
- [参考](reference/feature-matrix.md)

## 功能标志一览

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


## 验证命令

```sh
cargo fmt --check
cargo test --workspace --all-features
cargo test -p scenekit --test scenekit_v15 --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
```
