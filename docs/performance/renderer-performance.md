# 渲染器性能

## 目标

在热帧路径之外注册资源，保持管线和绑定组缓存温暖，避免每帧材质或纹理变化。

## 先测量

使用专注的命令，一次比较一个更改。当只有一个二进制文件或示例需要重型功能时，避免全局启用它们。

## 命令或模式

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

## 实际检查

- 保持仅 CPU crate 轻量级。
- 除非输入发生变化，否则避免每帧重建数据结构。
- 分析时分离加载、注册、更新、渲染目标、读回和渲染成本。
- 使用 `renderer.diagnostics()` 和 `renderer.resource_stats()` 跟踪纹理、几何体、统一变量和渲染目标内存。