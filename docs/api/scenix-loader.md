# `scenekit-loader`

## 角色

可选的 CPU 资产包、资产管理器、导入器、导出器、诊断、依赖跟踪、glTF/GLB 扩展元数据、OBJ/MTL、STL、图像、KTX2、HDR/EXR 和资产缓存。

## 依赖权重

重 `std` 路径；在外观上启用 `loader`。`http` 门控 URL 加载。

## 安装

```toml
[dependencies]
scenekit-loader = "1"
```

## 关键公共 API

GltfLoader、GltfAsset、AssetPackage、AssetManager、AssetCache、LoaderOptions、AssetDiagnostic、LoadedAnimationClip、LoadedSkin、LoadedMaterial、TextureTransform、MaterialVariant、通过外观的 RendererAssetExt、obj、stl、image、hdr、ktx2、export

## 常见用法

```rust
use scenekit_loader::{AssetManager, export};

# fn run() -> Result<(), scenekit_core::ScenixError> {
let mut manager = AssetManager::new();
let package = manager.load_file("scene.glb")?;
println!("{}", export::scene_json_string(&package));
# Ok(())
# }
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

`GltfAsset` 保持源兼容性。当你需要蒙皮、变形目标、导入的动画元数据、材质扩展、依赖图、诊断、导出器或通过 `RendererAssetExt` 的显式渲染器上传的 v1.3 伴随文件时，使用 `AssetPackage`。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)