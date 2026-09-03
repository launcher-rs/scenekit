# 渲染一个立方体

## 目标

通过可选的 `wgpu` 渲染器渲染一个生成的立方体。

## 相关功能标志

`renderer`

## 步骤

1. 添加所需的 Cargo 功能。
2. 在调用者拥有的存储中保存 CPU 场景数据。
3. 变换或层次结构编辑后调用 `update_world_transforms()`。
4. 仅在启用这些系统时向可选系统注册资源。

## 示例

```rust
use scenekit::{PerspectiveCamera, Renderer, RendererConfig, Vec3};

# async fn run(scene: &scenekit::SceneGraph) -> Result<(), scenekit::scenekitError> {
let mut renderer = Renderer::headless(RendererConfig::new(512, 512)).await?;
let camera = PerspectiveCamera::new(60.0, 1.0, 0.1, 100.0)
    .position(Vec3::new(0.0, 0.0, 4.0))
    .target(Vec3::ZERO);
renderer.render(scene, &camera)?;
# Ok(())
# }
```

## 验证

运行 `cargo run -p scenekit --example hello_cube --features renderer`。

## 相关文档

- [快速开始](../quick-start.md)
- [功能标志](../concepts/feature-flags.md)
