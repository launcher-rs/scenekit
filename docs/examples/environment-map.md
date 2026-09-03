# 环境贴图

## 目的

注册立方体纹理并将其用作渲染器环境贴图。

## 源码

`examples/environment_map.rs`

## 相关功能标志

`renderer`

## 运行或检查

```sh
cargo run -p scenekit --example environment_map --features renderer
```

## 查看内容

- 示例应使用列出的功能编译。
- 示例应报告一个渲染的绘制和一个注册的环境纹理。
- 渲染器示例可能需要工作的原生图形后端或无头支持。

## 相关文档

- [示例索引](README.md)
- [功能标志](../concepts/feature-flags.md)