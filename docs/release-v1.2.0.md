# Scenix v1.2.0 渲染器和材质对等

Scenix `1.2.0` 将渲染器从面向预览的材质颜色路径移至真正的渲染器拥有的 GPU 资源，用于纹理、灯光、环境描述符、渲染目标、诊断和生命周期工作流。

## 亮点

- 所有工作区 crate 都升级到 `1.2.0`。
- `scenekit-animato` 现在目标为 `animato = "1.5.0"`。
- `scenekit-renderer` 上传 `Texture2D`、`TextureCube` 和 `Texture3D` 资源，具有 mip 感知布局验证和采样器转换。
- PBR、物理、无光照、Lambert、卡通、法线和线框材质现在通过材质统一变量和纹理绑定组为活动渲染器路径提供数据。
- 环境光、半球光、方向光、点光源、聚光灯、区域光和光照探针数据可以注册到渲染器。
- `EnvironmentMap`、渲染器拥有的渲染目标、渲染到纹理、读回、资源诊断和生命周期 API 通过累加 API 可用。
- WebGL 回退现在在 WebGPU 不可用时优先使用真正的 WebGL2 渲染器路径，WebGL1 保留作为精简的最后手段路径。

## 安装

```toml
[dependencies]
scenekit = "1.2"
```

渲染器堆栈：

```toml
[dependencies]
scenekit = { version = "1.2", features = ["renderer", "post"] }
```

完整可选堆栈：

```toml
[dependencies]
scenekit = { version = "1.2", features = ["loader", "renderer", "post", "animato", "wasm"] }
```

## 代码示例

```rust
use scenekit::{
    Color, MaterialId, MeshId, PbrMaterial, PerspectiveCamera, Renderer, RendererConfig,
    Sampler, SceneGraph, SceneNode, Texture2D, TextureFormat, TextureId, Vec3, sphere_geometry,
};

# async fn run() -> Result<(), scenekit::ScenixError> {
let mut renderer = Renderer::headless(RendererConfig::new(512, 512)).await?;

let albedo_id = TextureId::new(10);
let albedo = Texture2D::new(
    1,
    1,
    TextureFormat::Rgba8UnormSrgb,
    vec![255, 180, 80, 255],
)?;
renderer.register_texture2d(albedo_id, &albedo, Sampler::new())?;

let mesh_id = MeshId::new(1);
let material_id = MaterialId::new(1);
renderer.register_mesh(mesh_id, &sphere_geometry(1.0, 48, 24))?;

let mut material = PbrMaterial::new()
    .albedo(Color::WHITE)
    .metallic_roughness(0.25, 0.4);
material.albedo_texture = Some(albedo_id);
renderer.register_pbr_material(material_id, &material)?;

let mut scene = SceneGraph::new();
scene.add(SceneNode::mesh("textured sphere", mesh_id, material_id));
scene.update_world_transforms();

let camera = PerspectiveCamera::new(50.0, 1.0, 0.1, 100.0)
    .position(Vec3::new(0.0, 0.0, 3.5))
    .target(Vec3::ZERO);

let stats = renderer.render(&scene, &camera)?;
println!("draws={}, textures={}", stats.opaque_draws, renderer.diagnostics().textures);
# Ok(())
# }
```

## 迁移说明

- 现有 v1 渲染器代码继续编译；新 API 是累加的。
- `Renderer::register_texture2d` 现在上传真正的 GPU 纹理，而不仅仅是存储元数据。
- 在 v1.2.0 中使用 `TextureId` 作为渲染目标：使用 `create_render_target` 创建目标，使用 `render_to_texture` 渲染，使用 `read_texture_pixel` 读取。
- 像以前一样启用 `animato` 外观功能；桥接现在解析到 Animato 1.5.0。

## WebGPU 和 WebGL

- 浏览器渲染以 WebGPU 为先。如果 WebGPU 不可用或不安全，`BrowserRenderer` 回退到 WebGL2 并通过 WebGL 纹理、材质统一变量、方向/点光源、卡通/物理近似、后处理切换、拾取和动画渲染生成的场景。
- WebGL2 通过诊断报告 `parity=full-fallback`，是 v1.2.0 演示的预期完整浏览器回退。
- WebGL1 仍然作为旧浏览器的精简最后手段回退，并报告 `parity=reduced-fallback`。

## 已知限制

- v1.2.0 保持 GPU 上传显式；加载器仍然产生 CPU 端资产。
- 物理材质传输和环境响应是实时近似。
- 阴影支持设计用于实际烟雾场景和编辑器预览；大型生产阴影系统仍然是未来的工作。
- GPU 测试仍然需要支持 Vulkan 的设备或 Mesa lavapipe。

## 链接

- 网站和演示：`https://aarambhdevhub.github.io/scenekit/`
- 文档：`https://docs.rs/scenekit`
- Crates：`https://crates.io/crates/scenekit`