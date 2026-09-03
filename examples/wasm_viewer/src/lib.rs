use wasm_bindgen::prelude::*;

/// 创建具有 WebGPU 优先/WebGL 回退的生成场景渲染器。
#[wasm_bindgen]
pub async fn start(canvas: web_sys::HtmlCanvasElement) -> Result<scenekit::BrowserRenderer, JsValue> {
    scenekit::set_panic_hook();
    scenekit::BrowserRenderer::new(canvas).await
}
