# scenekit

> 模块化的 Rust 原生 3D 场景框架，适用于原生和 WASM 应用。

[![CI](https://github.com/launcher-rs/scenekit/actions/workflows/ci.yml/badge.svg)](https://github.com/launcher-rs/scenekit/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

scenekit `1.5.0` 是当前的稳定版本。它新增了跨平台控件、触摸/游戏手柄/指针锁定输入、选择与变换原语、可复用 Gizmo、检查器快照、可选 egui 渲染，以及按需 GPU 拾取。CPU 创作默认保持轻量，GPU、浏览器和 UI 路径均为可选启用。

## 安装

大多数应用从门面 crate 开始：

```toml
[dependencies]
scenekit = "1"
```

仅在需要时启用可选系统：

```toml
[dependencies]
scenekit = { version = "1", features = ["loader"] }
scenekit = { version = "1", features = ["renderer", "post"] }
scenekit = { version = "1", features = ["animato"] }
scenekit = { version = "1", features = ["wasm"] }
scenekit = { version = "1", features = ["interaction"] }
scenekit = { version = "1", features = ["editor", "egui"] }
```

可以单独使用功能专注的 crate：

```toml
[dependencies]
scenekit-math = "1"
scenekit-core = "1"
scenekit-input = "1"
scenekit-scene = "1"
scenekit-camera = "1"
scenekit-mesh = "1"
scenekit-material = "1"
scenekit-light = "1"
scenekit-texture = "1"
scenekit-loader = "1"
scenekit-renderer = "1"
scenekit-post = "1"
scenekit-raycaster = "1"
scenekit-helpers = "1"
scenekit-animato = "1"
scenekit-wasm = "1"
```

用于 `no_std` CPU 创作：

```toml
[dependencies]
scenekit-math = { version = "1", default-features = false, features = ["libm"] }
scenekit-core = { version = "1", default-features = false }
scenekit-input = { version = "1", default-features = false }
scenekit-scene = { version = "1", default-features = false }
scenekit-camera = { version = "1", default-features = false }
scenekit-mesh = { version = "1", default-features = false }
scenekit-material = { version = "1", default-features = false }
scenekit-light = { version = "1", default-features = false }
scenekit-texture = { version = "1", default-features = false }
scenekit-raycaster = { version = "1", default-features = false }
scenekit-helpers = { version = "1", default-features = false }
```

`scenekit-loader`、`scenekit-renderer`、`scenekit-post`、`scenekit-animato` 和 `scenekit-wasm` 是可选的 `std` 路径。`scenekit-animato` 继续支持 Animato `1.7.0`。

## Feature 标志

| Feature | 默认值 | 说明 |
| --- | --- | --- |
| `std` | 是 | CPU crate 的标准库支持。 |
| `scene`、`camera`、`mesh`、`material`、`light`、`texture` | 是 | CPU 创作 crate。 |
| `raycaster`、`helpers` | 是 | BVH 拾取和调试线辅助数据。 |
| `interaction` | 否 | 场景、相机、光线投射器和辅助工具的便捷功能包。 |
| `editor` | 否 | 检查器快照和编辑器端的选择/变换数据。 |
| `egui` | 只读 egui 检查器适配器；宿主拥有其窗口/渲染循环。 |
| `loader` | 否 | 资产包、资产管理器、glTF/GLB 扩展元数据、OBJ/MTL、STL、图像、KTX2、HDR/EXR 加载以及导出器。 |
| `renderer` | 否 | 基于 `wgpu` 的渲染器，支持 surface/headless 目标。 |
| `post` | 否 | 全屏后处理管线；需与 `renderer` 配合使用。 |
| `animato` | 否 | Animato 桥接：过程式补间/弹簧轨迹**以及**基于片段的动画运行时（片段、动作、混合器、循环模式、交叉淡入淡出、叠加混合、标记/事件、灯光/变形目标、重定向、CPU 蒙皮）。目标 Animato `1.7.0`。 |
| `wasm` | 否 | 浏览器 Canvas 封装，优先使用 WebGPU，WebGL2 完整回退，WebGL1 精简回退，并包含生成的演示场景。 |
| `serde` | 否 | 在功能专注的 crate 支持时提供序列化支持。 |

## 快速开始

### 控件与交互

```rust
use scenekit::{
    ArcballController, InputState, PerspectiveCamera, PointerButton, Vec2, Vec3,
    ViewportMetrics,
};

let mut input = InputState::new(ViewportMetrics::new(Vec2::new(1280.0, 720.0), 2.0));
input.on_pointer_down(PointerButton::Left);
input.on_pointer_move(Vec2::new(24.0, 12.0));
input.on_scroll(-0.25);

let mut controls = ArcballController::new(Vec3::ZERO, 5.0);
let mut camera = PerspectiveCamera::default();
controls.update_from_input(&input, 1.0 / 60.0);
controls.apply_to_perspective(&mut camera);
input.end_frame();
```

`InputState` 还支持固定容量的触摸触点、双指手势、四种标准游戏手柄以及指针锁定的相对运动，且在事件/控制热路径中不会产生分配。

### 场景创作与动画

```rust
use std::collections::BTreeMap;
use scenekit::{
    CameraId, CameraStores, Geometry, MaterialId, MeshId, NodeAnimationTarget, NodeAnimator,
    PbrMaterial, PerspectiveCamera, SceneGraph, SceneNode, ScenixAnimationDriver, Vec3,
    Vec3Track, box_geometry,
};

# fn run() -> Result<(), scenekit::ValidationError> {
let mesh_id = MeshId::new(1);
let material_id = MaterialId::new(1);

let mut meshes = BTreeMap::<MeshId, Geometry>::new();
meshes.insert(mesh_id, box_geometry(1.0, 1.0, 1.0, 1, 1, 1));

let mut scene = SceneGraph::new();
let cube = scene.add(SceneNode::mesh("cube", mesh_id, material_id));

let mut driver = ScenixAnimationDriver::new();
driver.add_node(NodeAnimator::new(
    cube,
    NodeAnimationTarget::Translation(Vec3Track::tween(
        Vec3::ZERO,
        Vec3::new(1.0, 0.0, 0.0),
        0.5,
    )),
));

let camera_id = CameraId::new(1);
let mut perspective = BTreeMap::from([(
    camera_id,
    PerspectiveCamera::new(60.0, 1.0, 0.1, 100.0)
        .position(Vec3::new(0.0, 0.0, 4.0))
        .target(Vec3::ZERO),
)]);
let mut orthographic = BTreeMap::new();
let mut cameras = CameraStores {
    perspective: &mut perspective,
    orthographic: &mut orthographic,
};
let mut materials = BTreeMap::from([(material_id, PbrMaterial::new())]);
let mut skeletons = Vec::new();

driver.tick(0.5, &mut scene, &mut cameras, &mut materials, &mut skeletons)?;
scene.update_world_transforms();
# Ok(())
# }
```

### 无头渲染

```rust
use scenekit::{Renderer, RendererConfig, PerspectiveCamera, Vec3};

# async fn run(scene: &scenekit::SceneGraph) -> Result<(), scenekit::ScenixError> {
let mut renderer = Renderer::headless(RendererConfig::new(256, 256)).await?;
let camera = PerspectiveCamera::new(60.0, 1.0, 0.1, 100.0)
    .position(Vec3::new(0.0, 0.0, 5.0))
    .target(Vec3::ZERO);
let stats = renderer.render(scene, &camera)?;
assert!(stats.frame_index > 0);
# Ok(())
# }
```

### 加载资产

```rust
use scenekit::{AssetManager, RendererAssetExt};

# async fn run() -> Result<(), scenekit::ScenixError> {
let mut manager = AssetManager::new();
let package = manager.load_file("scene.glb")?;

let mut renderer = scenekit::Renderer::headless(scenekit::RendererConfig::new(512, 512)).await?;
let uploaded = renderer.register_asset_package(&package)?;
println!("meshes={}, textures={}", uploaded.meshes, uploaded.textures);
# Ok(())
# }
```

### 动画运行时

v1.4.0 的 `AnimationMixer` 在 Animato 之上播放基于片段的动画：

```rust
use scenekit::{
    AnimationClip, AnimationMixer, ClipChannel, ClipTrack, KeyframeInterpolation,
    KeyframeVec3, LoopMode, NodeProperty, PropertyBinding, SceneGraph, SceneNode, Vec3,
};

let mut scene = SceneGraph::new();
let node = scene.add(SceneNode::new("mover"));

let clip = AnimationClip::empty("move").with_channel(ClipChannel {
    binding: PropertyBinding::Node { node_id: node, property: NodeProperty::Translation },
    track: ClipTrack::Vec3(KeyframeVec3::new(
        vec![0.0, 1.0],
        vec![Vec3::ZERO, Vec3::new(5.0, 0.0, 0.0)],
        KeyframeInterpolation::Linear,
    )),
});

let mut mixer = AnimationMixer::new();
let clip_index = mixer.add_clip(clip);
let action = mixer.add_action(clip_index);
mixer.action_mut(action).unwrap().set_loop_mode(LoopMode::REPEAT);
mixer.action_mut(action).unwrap().play(0.0);

// 每帧调用：
// mixer.tick(dt, &mut scene, &mut cameras, &mut materials,
//            &mut lights, &mut skeletons, &mut morphs)
```

### 光线投射

```rust
use std::collections::BTreeMap;
use scenekit::{
    Geometry, MaterialId, MeshId, PerspectiveCamera, Raycaster, SceneGraph, SceneNode,
    Vec2, Vec3, box_geometry,
};

let mesh_id = MeshId::new(1);
let material_id = MaterialId::new(1);
let meshes = BTreeMap::<MeshId, Geometry>::from([(
    mesh_id,
    box_geometry(1.0, 1.0, 1.0, 1, 1, 1),
)]);

let mut scene = SceneGraph::new();
scene.add(SceneNode::mesh("cube", mesh_id, material_id));
scene.update_world_transforms();

let camera = PerspectiveCamera::new(60.0, 1.0, 0.1, 100.0)
    .position(Vec3::new(0.0, 0.0, 4.0))
    .target(Vec3::ZERO);
let ray = Raycaster::from_camera_ndc(&camera, Vec2::ZERO);

let mut raycaster = Raycaster::new();
raycaster.build_bvh(&scene, &meshes).unwrap();
assert!(raycaster.cast_ray(ray, &scene, &meshes).is_some());
```

## 工作空间

| Crate | 角色 |
| --- | --- |
| `scenekit` | 门面 crate，提供稳定的 v1 feature 标志。 |
| `scenekit-math` | `no_std` 向量、矩阵、四元数、变换、射线和包围盒。 |
| `scenekit-core` | ID、颜色、错误类型和共享 trait。 |
| `scenekit-input` | 指针、键盘、触摸、手势、游戏手柄、指针锁定和视口状态。 |
| `scenekit-scene` | 场景图，含编辑器元数据、层、对齐和选择状态。 |
| `scenekit-camera` | 相机，含轨道、飞行、弧球、轨迹球、地图、第一人称和指针锁定控件。 |
| `scenekit-mesh` | 几何缓冲区、图元、实例化、批处理和变形目标。 |
| `scenekit-material` | 无 GPU 依赖的材质描述和管线键。 |
| `scenekit-light` | 灯光、阴影设置和光照探针。 |
| `scenekit-texture` | CPU 纹理、采样器、图集、视频更新和 Mipmap。 |
| `scenekit-loader` | 可选 CPU 资产包、资产管理器、导入器、导出器、诊断工具和缓存。 |
| `scenekit-renderer` | 可选 `wgpu` 渲染器，支持按需 ID/法线/深度编辑器拾取。 |
| `scenekit-post` | 可选 `wgpu` 后处理效果。 |
| `scenekit-raycaster` | BVH 射线拾取、框选、拖拽平面和变换交互。 |
| `scenekit-helpers` | 调试线、可复用 Gizmo、选择可视化、对齐网格和可选 egui。 |
| `scenekit-animato` | 可选 Animato 桥接。 |
| `scenekit-wasm` | 可选浏览器 Canvas 封装，支持 WebGPU 和 WebGL 路径。 |

## 示例

门面 crate 注册了 [ARCHITECTURE.md](./ARCHITECTURE.md) 中的示例集：

```sh
cargo run -p scenekit --example hello_cube --features renderer
cargo run -p scenekit --example pbr_sphere --features renderer
cargo run -p scenekit --example physical_material --features renderer
cargo run -p scenekit --example toon_shading --features renderer
cargo run -p scenekit --example gltf_scene --features "loader renderer"
cargo run -p scenekit --example asset_pipeline --features "loader renderer"
cargo run -p scenekit --example asset_manager --features loader
cargo run -p scenekit --example export_scene --features loader
cargo run -p scenekit --example animation_import --features loader
cargo run -p scenekit --example compressed_assets --features loader
cargo run -p scenekit --example shadow_demo --features renderer
cargo run -p scenekit --example raycasting
cargo run -p scenekit --example post_processing --features "renderer post"
cargo run -p scenekit --example instanced_mesh
cargo run -p scenekit --example animato_integration --features animato
cargo run -p scenekit --example animation_runtime --features "animato scene"
cargo run -p scenekit --example animation_mixer --features "animato scene material light"
cargo run -p scenekit --example skeleton_skinning --features "mesh animato"
cargo run -p scenekit --example animation_events --features "animato scene light"
cargo run -p scenekit --example orbit_camera
cargo run -p scenekit --example lod_demo
cargo run -p scenekit --example morph_targets
cargo run -p scenekit --example fog_demo
cargo run -p scenekit --example helpers_demo
cargo run -p scenekit --example controls_showcase
cargo run -p scenekit --example selection_and_drag
cargo run -p scenekit --example transform_gizmo
cargo run -p scenekit --example editor_inspector --features egui
cargo run -p scenekit --example renderer_picking --features renderer
cargo run -p scenekit --example sprite_particles
cargo run -p scenekit --example environment_map --features renderer
cargo run -p scenekit --example render_target_capture --features renderer
```

浏览器示例位于 `examples/wasm_viewer`：

```sh
rustup target add wasm32-unknown-unknown
cargo check --manifest-path examples/wasm_viewer/Cargo.toml --target wasm32-unknown-unknown
```

## 网站

静态网站是 `website/` 下的独立 Leptos CSR 应用。它被有意放在主工作空间之外，以便网站依赖不会影响普通库用户。

```sh
cd website
trunk serve
trunk build --release --public-url /scenekit/
```

GitHub Pages 部署由 `.github/workflows/pages.yml` 处理，该工作流使用 Trunk 构建 `website/dist` 并将其部署到 `/scenekit/`。演示使用 `scenekit-wasm`：在安全情况下尝试 WebGPU，当 WebGPU 不可用时回退到具有相同生成场景/材质/灯光控件的 WebGL2 渲染器，仅在 GPU 路径均不可用时使用精简的 WebGL1，Canvas2D 预览仅在两条 GPU 路径均不可用时才使用。

## 开发检查

```sh
cargo fmt --check
cargo clippy --workspace --all-features -- -D warnings
cargo test --workspace
cargo test --workspace --all-features
cargo test -p scenekit-math -p scenekit-core -p scenekit-input -p scenekit-scene -p scenekit-camera -p scenekit-mesh -p scenekit-material -p scenekit-light -p scenekit-texture -p scenekit-raycaster -p scenekit-helpers -p scenekit-animato --no-default-features
cargo test -p scenekit-loader --all-features
cargo test -p scenekit-raycaster -p scenekit-helpers --all-features
cargo test -p scenekit-animato --all-features
cargo test -p scenekit --test scenekit_v15 --all-features
cargo check -p scenekit --no-default-features --features interaction
cargo check -p scenekit --no-default-features --features editor
cargo check -p scenekit --no-default-features --features egui
cargo check -p scenekit-wasm --target wasm32-unknown-unknown --all-features
cargo check --manifest-path examples/wasm_viewer/Cargo.toml --target wasm32-unknown-unknown
SCENIX_RUN_GPU_TESTS=1 WGPU_BACKEND=vulkan cargo test -p scenekit-renderer -p scenekit-post --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
cargo bench --workspace --no-run
cargo llvm-cov --workspace --all-features
```

## 文档

- [开发者文档](./docs/README.md)
- [架构](./ARCHITECTURE.md)
- [路线图](./ROADMAP.md)
- [更新日志](./CHANGELOG.md)
- [入门指南](./docs/getting-started.md)
- [安装](./docs/installation.md)
- [快速开始](./docs/quick-start.md)
- [核心概念](./docs/concepts/README.md)
- [使用指南](./docs/guides/create-your-first-scene.md)
- [API 参考](./docs/api/facade-crate.md)
- [示例](./docs/examples/README.md)
- [实用技巧](./docs/recipes/README.md)
- [性能优化](./docs/performance/README.md)
- [部署](./docs/deployment/README.md)
- [迁移指南](./docs/migration/from-0.9-to-1.0.md)
- [参考资料](./docs/reference/feature-matrix.md)
- [v1.5.0 发布说明](./.github/release-notes/1.5.0.md)
- [v1.4.0 发布说明](./.github/release-notes/1.4.0.md)
- [v1.3.0 发布说明](./.github/release-notes/v1.3.0.md)

## 已知限制

- 渲染器现在通过真实的 GPU 资源上传材质纹理、灯光数据、环境描述符和渲染目标。高级物理着色是一种实用的实时路径，而非离线影视渲染器。
- Loader API 生成 CPU 端的 scenekit 数据；`RendererAssetExt` 仍然显式上传到渲染器拥有的 GPU 资源中，只是一个便捷桥接。
- Draco 和 meshopt 压缩的 glTF 资产目前会产生明确的诊断信息，除非经过外部转换器预处理。
- WebGL2 是当 WebGPU 不可用时生成渲染器场景的完整浏览器回退。WebGL1 仍然是针对旧浏览器的精简最后手段回退。
- 网站演示不包含大型模型资产。
- GPU 测试需要支持 Vulkan 的设备或 Mesa lavapipe。

## 许可证

采用以下任一许可证：

- Apache License, Version 2.0
- MIT license
