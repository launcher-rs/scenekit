# 快速开始

当你想要可复制的最常见 Scenix 任务代码时，请使用此页面。除非专注的 crate 更合适，否则代码片段使用 `scenekit` 外观 crate。

## 创建场景

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

## 添加相机

```rust
use scenekit::{PerspectiveCamera, Vec3};

let camera = PerspectiveCamera::new(60.0, 16.0 / 9.0, 0.1, 100.0)
    .position(Vec3::new(0.0, 1.5, 4.0))
    .target(Vec3::ZERO);
```

## 从相机进行光线投射

```rust
use scenekit::{PerspectiveCamera, Raycaster, Vec2, Vec3};

let camera = PerspectiveCamera::new(60.0, 1.0, 0.1, 100.0)
    .position(Vec3::new(0.0, 0.0, 4.0))
    .target(Vec3::ZERO);
let ray = Raycaster::from_camera_ndc(&camera, Vec2::ZERO);
# let _ = ray;
```

## 无头渲染

首先启用 `renderer`：

```toml
scenekit = { version = "1", features = ["renderer"] }
```

```rust
use scenekit::{PerspectiveCamera, Renderer, RendererConfig, Vec3};

# async fn run(scene: &scenekit::SceneGraph) -> Result<(), scenekit::ScenixError> {
let mut renderer = Renderer::headless(RendererConfig::new(512, 512)).await?;
let camera = PerspectiveCamera::new(60.0, 1.0, 0.1, 100.0)
    .position(Vec3::new(0.0, 0.0, 4.0))
    .target(Vec3::ZERO);
renderer.render(scene, &camera)?;
# Ok(())
# }
```

## 动画节点

首先启用 `animato`：

```toml
scenekit = { version = "1", features = ["animato"] }
```

```rust
use scenekit::{NodeAnimationTarget, NodeAnimator, ScenixAnimationDriver, Vec3, Vec3Track};

let mut driver = ScenixAnimationDriver::new();
driver.add_node(NodeAnimator::new(
    scenekit::NodeId::new(1),
    NodeAnimationTarget::Translation(Vec3Track::tween(
        Vec3::ZERO,
        Vec3::new(1.0, 0.0, 0.0),
        0.5,
    )),
));
```