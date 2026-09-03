# `scenekit-wasm`

## 角色

可选的浏览器画布包装器、DPR 感知 DOM 输入映射、触摸、指针锁和游戏手柄转发、生成的场景设置、WebGPU/WebGL 后端选择、变换模式、场景选择、检查器 JSON 和演示获取器。

## 依赖权重

面向浏览器的可选路径；在外观上启用 `wasm`。这会引入浏览器绑定、用于 WebGPU 的 `scenekit-renderer`，以及用于 WebGPU 不可用的浏览器的 WebGL2 优先回退渲染器。

## 安装

```toml
[dependencies]
scenekit-wasm = "1"
```

## 关键公共 API

`BrowserRenderer`、`BrowserBackendPreference`、`BrowserBackendKind`、`WebRenderer`、`WebGlRenderer`、`WebGlCapabilityLevel`、`CanvasMetrics`、`set_panic_hook`、`key_code_from_dom`、`pointer_button_from_dom`、`touch_phase_from_dom`、标准游戏手柄映射、`canvas_metrics`、触摸和指针锁方法、`set_transform_mode` 和 `inspector_snapshot_json`。

## 后端选择

对于应该在浏览器中工作的应用程序和演示，使用 `BrowserRenderer`：

```rust
use scenekit_wasm::BrowserRenderer;
use wasm_bindgen::JsCast;

# async fn run() -> Result<(), wasm_bindgen::JsValue> {
let canvas = web_sys::window()
    .unwrap()
    .document()
    .unwrap()
    .get_element_by_id("canvas")
    .unwrap()
    .dyn_into::<web_sys::HtmlCanvasElement>()?;

let mut renderer = BrowserRenderer::new(canvas).await?;
renderer.tick(0.0)?;
let active_backend = renderer.backend_label();
# let _ = active_backend;
# Ok(())
# }
```

仅当你专门需要 WebGPU 时使用 `WebRenderer`。在浏览器测试或产品演示中使用 `WebGlRenderer` 强制 WebGL 路径；它首先请求 WebGL2，当 WebGL2 活动时报告 `parity=full-fallback`。

## 常见用法

```sh
cargo check -p scenekit-wasm --target wasm32-unknown-unknown --all-features
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [交互和编辑器原语](../concepts/interaction-and-editor.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)