# 渲染器

## 目的

使用可选的 `wgpu` 渲染器进行表面和无头渲染。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

启用 `renderer`。

## 关键规则

- 渲染器拥有设备、队列、表面、缓冲区、纹理、渲染目标、灯光统一变量和管线缓存。
- SceneGraph 存储 ID；渲染器资源注册将 ID 映射到 GPU 资源。
- 显式注册、更新、注销或清除 GPU 资源；加载器仍然产生 CPU 资产。
- 在 v1.2.0 中使用 `TextureId` 作为渲染器拥有的渲染目标。
- GPU 测试通过 `SCENIX_RUN_GPU_TESTS=1` 门控。


## 示例

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

## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)