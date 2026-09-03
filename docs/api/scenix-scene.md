# `scenekit-scene`

## 角色

场景图节点、层次结构、变换、遍历、雾、精灵、LOD 辅助工具、选择状态、图层策略、捕捉和稀疏编辑器元数据。

## 依赖权重

轻量级 `no_std`；默认外观功能。

## 安装

```toml
[dependencies]
scenekit-scene = "1"
```

## 关键公共 API

`SceneGraph`、`SceneNode`、`NodeKind`、`SelectionState`、`SelectionMode`、`NodeEditorMetadata`、`LayerMask`、`LayerPolicy`、`TransformMode`、`TransformSpace`、`TransformConstraint`、`SnapSettings`、`Fog`、`LodGroup` 和 `Sprite`。

## 常见用法

```rust
use scenekit::{MaterialId, MeshId, SceneGraph, SceneNode, box_geometry};

let mesh_id = MeshId::new(1);
let material_id = MaterialId::new(1);
let geometry = box_geometry(1.0, 1.0, 1.0, 1, 1, 1);

let mut scene = SceneGraph::new();
let cube = scene.add(SceneNode::mesh("cube", mesh_id, material_id));
scene.select(cube, scenekit::SelectionMode::Replace).unwrap();
scene.update_world_transforms();
# let _ = geometry;
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [交互和编辑器原语](../concepts/interaction-and-editor.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)