# Scenix v1.1.0 浏览器回退

Scenix `1.1.0` 通过在 `scenekit-wasm` 中添加真正的 WebGL 回退来提高浏览器可靠性。稳定的 v1 API 保持累加性：现有的 WebGPU `WebRenderer` 仍然可用，应用程序现在可以在需要自动后端选择时使用 `BrowserRenderer`。

## 亮点

- 所有工作区 crate 都升级到 `1.1.0`。
- `scenekit-wasm` 现在提供 `BrowserRenderer`、`WebGlRenderer`、`BrowserBackendPreference` 和 `BrowserBackendKind`。
- 浏览器启动现在在安全的地方选择 WebGPU，当 WebGPU 不可用或不合适时使用 WebGL，并将 Canvas2D 留作最终网站回退。
- 网站演示使用 crate 级别的 `BrowserRenderer` 而不是仅网站的 WebGPU 检测。
- WebGL 使用深度测试、相机矩阵、材质颜色预览、辅助工具可见性、线框预览、动画控件和 CPU 光线投射拾取来渲染生成的 Scenix Engine Lab 场景。
- 发布和 Pages 工作流使用 Trunk 兼容的 `NO_COLOR=false` 环境构建网站。

## 安装

```toml
[dependencies]
scenekit = "1.1"
```

通过外观的浏览器支持：

```toml
[dependencies]
scenekit = { version = "1.1", features = ["wasm"] }
```

可选完整堆栈：

```toml
[dependencies]
scenekit = { version = "1.1", features = ["loader", "renderer", "post", "animato", "wasm"] }
```

## 与 1.0.0 的变化

- 将所有工作区 crate 从 `1.0.0` 升级到 `1.1.0`。
- 在 `scenekit-wasm` 中添加了 WebGL 兼容性渲染器；不需要新的 crate。
- 通过 `BrowserRenderer` 添加了自动浏览器后端选择。
- 更新了网站桥接，以便 Firefox 和没有可用 WebGPU 的浏览器在 Canvas2D 之前尝试 WebGL。
- 更新了文档、更新日志、测试和 GitHub Release 自动化以支持 `v1.1.0` 发布。

## 代码示例

```rust
use scenekit_wasm::BrowserRenderer;
use wasm_bindgen::JsCast;

# async fn run() -> Result<(), wasm_bindgen::JsValue> {
let canvas = web_sys::window()
    .unwrap()
    .document()
    .unwrap()
    .get_element_by_id("scenekit-canvas")
    .unwrap()
    .dyn_into::<web_sys::HtmlCanvasElement>()?;

let mut renderer = BrowserRenderer::new(canvas).await?;
renderer.tick(0.0)?;
web_sys::console::log_1(&renderer.backend_label().into());
# Ok(())
# }
```

## 迁移说明

- 如果现有 `WebRenderer` 用户直接需要 WebGPU，则无需更改代码。
- 浏览器应用程序应优先使用 `BrowserRenderer` 用于生产演示和网站，因为它处理 WebGPU 到 WebGL 的回退。
- Canvas2D 回退仍然是应用程序级别的 UI 行为；它不作为 Scenix 渲染后端公开。
- 为加载器、渲染器、后处理、Animato 和 WASM 路径保留显式功能标志。

## 已知限制

- WebGL 浏览器渲染是兼容性预览路径；它不是完整的 `wgpu` 功能对等。
- WebGL 未实现真正的阴影、后处理、GPU 纹理上传对等或物理准确的 PBR。
- 加载器 API 解码 CPU 资产，但不会自动将它们上传到任一渲染器。
- GPU 测试仍然依赖于工作的 Vulkan 后端或 Mesa lavapipe。

## 链接

- 网站和演示：`https://aarambhdevhub.github.io/scenekit/`
- 文档：`https://docs.rs/scenekit`
- Crates：`https://crates.io/crates/scenekit`