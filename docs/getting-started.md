# 快速入门

当你刚接触 Scenix 并想创建场景而无需预先选择每个子系统时，请使用此页面。默认外观为你提供 CPU 场景创作、相机、几何体、材质、灯光、纹理、光线投射和辅助工具。

## 安装

```toml
[dependencies]
scenekit = "1"
```

仅在需要时添加更重的系统：

```toml
scenekit = { version = "1", features = ["renderer"] }
scenekit = { version = "1", features = ["loader", "renderer"] }
scenekit = { version = "1", features = ["renderer", "post"] }
scenekit = { version = "1", features = ["animato"] }
scenekit = { version = "1", features = ["wasm"] }
```

## 第一个场景

```rust
use scenekit::{MaterialId, MeshId, SceneGraph, SceneNode, box_geometry};

let mesh_id = MeshId::new(1);
let material_id = MaterialId::new(1);
let geometry = box_geometry(1.0, 1.0, 1.0, 1, 1, 1);

let mut scene = SceneGraph::new();
scene.add(SceneNode::mesh("cube", mesh_id, material_id));
scene.update_world_transforms();
# let _ = geometry;
```

## 接下来阅读什么

- [安装](installation.md) 了解功能和 crate 选择。
- [快速开始](quick-start.md) 获取可复制的代码片段。
- [场景图](concepts/scene-graph.md) 了解层次结构和变换规则。
- [渲染一个立方体](guides/render-a-cube.md) 当你准备使用 `wgpu` 时。