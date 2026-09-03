# 概念

这些页面解释了在复制指南代码之前 Scenix 系统如何协同工作。当决定 crate 边界、功能标志、数据所有权或运行时架构时阅读它们。

- [架构概述](architecture-overview.md)
- [场景图](scene-graph.md)
- [变换](transforms.md)
- [相机](cameras.md)
- [网格和几何体](meshes-and-geometry.md)
- [材质](materials.md)
- [灯光](lights.md)
- [纹理](textures.md)
- [渲染器](renderer.md)
- [后处理](post-processing.md)
- [光线投射](raycasting.md)
- [交互和编辑器原语](interaction-and-editor.md)
- [辅助工具](helpers.md)
- [使用 Animato 动画](animation-with-animato.md)
- [WASM 和浏览器](wasm-and-browser.md)
- [功能标志](feature-flags.md)
- [no_std](no-std.md)
- [错误处理](error-handling.md)

## 通用规则

CPU crate 描述场景数据。可选的加载器、渲染器、后处理、Animato 和 WASM crate 分层在顶部并保持可选。