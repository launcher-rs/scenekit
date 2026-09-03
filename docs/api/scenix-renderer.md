# `scenekit-renderer`

## 角色

可选的 `wgpu` 渲染器、GPU 资源存储、材质纹理上传、灯光统一变量、渲染目标、管线缓存、帧统计、阴影、无头渲染和按需对象 ID/法线/深度编辑器拾取。

## 依赖权重

重 `std` 路径；在外观上启用 `renderer`。

## 安装

```toml
[dependencies]
scenekit-renderer = "1"
```

## 关键公共 API

`Renderer`、`RendererConfig`、`FrameStats`、`RendererDiagnostics`、`ResourceStats`、`EditorPickRequest`、`EditorPickResult`、`EditorBufferStats`、`EnvironmentMap`、`RenderTargetDescriptor`、`GpuScene`、`GpuMaterial`、`PipelineCache`、`GBuffer` 和 `ShadowMapAtlas`。

## 常见用法

```rust
use scenekit::{PerspectiveCamera, Renderer, RendererConfig, Vec3};

# async fn run(scene: &scenekit::SceneGraph) -> Result<(), scenekit::ScenixError> {
let mut renderer = Renderer::headless(RendererConfig::new(512, 512)).await?;
let camera = PerspectiveCamera::new(60.0, 1.0, 0.1, 100.0)
    .position(Vec3::new(0.0, 0.0, 4.0))
    .target(Vec3::ZERO);
renderer.render(scene, &camera)?;
let picked = renderer.pick(scene, &camera, scenekit::EditorPickRequest::new(256, 256))?;
# let _ = picked;
# Ok(())
# }
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [渲染器拾取示例](../examples/renderer-picking.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)