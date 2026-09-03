# 创建数据可视化

## 使用场景

当你需要此行为在应用程序中并想要支持它的最小 Scenix 子系统集时。

## 方法

从数据生成几何体或辅助线，并将渲染器依赖项保留为可选直到显示时。

## 示例

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

## 验证

围绕上述状态更改或命令添加专注测试。对于浏览器或 GPU 路径，保持测试门控以便正常 CPU CI 保持快速。