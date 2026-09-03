# 变换小工具

源码：`examples/transform_gizmo.rs`

此示例将变换小工具写入可重用的线和句柄存储，然后在不为句柄构建渲染网格的情况下执行分析射线命中测试。

```sh
cargo run -p scenekit --example transform_gizmo
```

更改 `TransformMode` 以生成平移、旋转或缩放句柄。
