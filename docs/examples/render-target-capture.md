# 渲染目标捕获

## 目的

将场景渲染到渲染器拥有的纹理目标并读回一个像素。

## 源码

`examples/render_target_capture.rs`

## 相关功能标志

`renderer`

## 运行或检查

```sh
cargo run -p scenekit --example render_target_capture --features renderer
```

## 查看内容

- 示例应创建一个 `TextureId` 渲染目标。
- 它应将立方体渲染到该目标并打印一个 RGBA 像素。

## 相关文档

- [示例索引](README.md)
- [渲染器](../concepts/renderer.md)