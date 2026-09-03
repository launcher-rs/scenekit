# 编辑器检查器

源码：`examples/editor_inspector.rs`

此示例通过 `Inspectable` 对场景进行快照，并使用可选的 egui 适配器渲染共享的类型化模型。Scenix 不拥有 egui 上下文或事件循环。

```sh
cargo run -p scenekit --example editor_inspector --features egui
```