# 故障排除

## 浏览器演示无法启动

检查浏览器 WebGPU 和 WebGL 支持。网站首先尝试 WebGPU，其次 WebGL2，第三精简 WebGL1，当两个 GPU 路径都不可用时使用 Canvas2D 预览。

## 渲染器测试在 CI 中失败

仅在配置的后端上运行 GPU 测试：

```sh
scenekit_RUN_GPU_TESTS=1 WGPU_BACKEND=vulkan cargo test -p scenekit-renderer -p scenekit-post --all-features
```

## 加载器无法解码资产

确认加载器功能和支持的格式。在 v1.3 中，`AssetPackage::diagnostics` 报告已识别但不支持的功能，如 Draco 或 meshopt 压缩。`scenekit-loader` 将资产解码为 CPU 数据；上传通过渲染器注册或 `RendererAssetExt` 保持显式。

## 光线投射器错过对象

变换编辑后调用 `scene.update_world_transforms()`，并在场景或几何体更改后重建 BVH。

## no_std 构建失败

在 CPU crate 上禁用默认功能，不要在无默认目标中包含加载器、渲染器、后处理或 WASM crate。
