# 选择和拖动

源码：`examples/selection_and_drag.rs`

此示例构建场景 BVH，通过透视矩形框视锥体选择节点，更新场景选择模型，并在面向相机的平面上执行捕捉拖动。

```sh
cargo run -p scenekit --example selection_and_drag
```

拖动会话可以通过 `end` 提交或通过 `cancel` 恢复。