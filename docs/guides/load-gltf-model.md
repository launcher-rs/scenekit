# 加载 glTF 模型

## 目标

将 glTF 或 GLB 资产解码为场景、网格、材质、纹理、灯光、相机和 v1.3 资产管道元数据存储。

## 相关功能标志

`loader`；渲染结果时添加 `renderer`。

## 步骤

1. 添加所需的 Cargo 功能。
2. 当需要 v1.3 元数据时使用 `AssetManager` 或 `GltfLoader::load_package_file`。
3. 变换或层次结构编辑后调用 `update_world_transforms()`。
4. 仅在启用这些系统时向可选系统注册资源。

## 示例

```rust
use scenekit::AssetManager;

let mut manager = AssetManager::new();
let package = manager.load_file("scene.gltf")?;
println!("meshes: {}", package.meshes.len());
# Ok::<(), scenekit::scenekitError>(())
```

当只需要旧的 `GltfAsset` 形状时使用 `GltfLoader::load_file`。

## 验证

运行 `cargo run -p scenekit --example asset_pipeline --features "loader renderer"`。

## 相关文档

- [快速开始](../quick-start.md)
- [功能标志](../concepts/feature-flags.md)
