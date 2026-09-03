# 渲染器拾取

源码：`examples/renderer_picking.rs`

此示例创建一个无头渲染器，上传一个立方体，并请求一个编辑器像素。返回的值包括可选的节点 ID、深度、解码法线和重建的世界位置。

```sh
cargo run -p scenekit --example renderer_picking --features renderer
```

第一个请求分配编辑器目标；后续请求重用它们。当没有无头适配器可用时，示例会干净地退出。