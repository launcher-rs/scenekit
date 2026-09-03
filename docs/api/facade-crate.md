# `scenekit`

## 角色

用于希望从一个包获得稳定 v1 导入的应用程序的外观 crate。

## 依赖权重

默认 CPU 创作加可选的重系统。

## 安装

```toml
[dependencies]
scenekit = "1"
```

## 关键公共 API

`InputState`、`SceneGraph`、`SceneNode`、`PerspectiveCamera`、相机动画器、`Geometry`、`PbrMaterial`、`Raycaster`、拖动/变换控件、小工具、`Inspectable`、`Renderer`、`GltfLoader`、`PostStack`、`AnimationMixer`、`BrowserRenderer` 和 `WebRenderer`。

## 常见用法

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

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [交互和编辑器原语](../concepts/interaction-and-editor.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)