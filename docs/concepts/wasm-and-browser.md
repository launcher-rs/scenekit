# WASM 和浏览器

## 目的

使用浏览器包装器、DOM 输入映射、生成的场景演示和浏览器后端回退。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

启用 `wasm`。`scenekit-wasm::BrowserRenderer` 在安全的地方尝试 WebGPU，当 WebGPU 缺失或不合适时使用 WebGL2 作为完整的生成场景回退，仅对旧浏览器使用精简的 WebGL1，并允许应用程序在 WebGL 不可用时回退到自己的 Canvas2D 预览。

## 关键规则

- `scenekit-wasm` 包装画布设置和输入转发。
- `WebRenderer` 是直接的 WebGPU 路径。
- `WebGlRenderer` 是直接的 WebGL 回退路径；它首先请求 WebGL2，仅在需要时请求 WebGL1。
- `BrowserRenderer` 在安全浏览器上首先选择 WebGPU，否则在可用时选择 WebGL2。
- v1.3 资产包保留在 CPU 端。浏览器应用应通过其 Web 框架获取字节，然后将字节传递给 `AssetManager` 或 `GltfLoader::load_package_bytes`。
- 网站是 Leptos CSR，使用 Trunk 构建。
- 回退 UI 应干净地处理不可用的 WebGPU 和 WebGL。

## 最小浏览器渲染器

```rust
use scenekit_wasm::BrowserRenderer;
use wasm_bindgen::JsCast;

# async fn start() -> Result<(), wasm_bindgen::JsValue> {
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


## 示例

```sh
rustup target add wasm32-unknown-unknown
cargo check -p scenekit-wasm --target wasm32-unknown-unknown --all-features
```

## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)
