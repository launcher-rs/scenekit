# glTF 场景

## 目的

加载生成的 glTF 夹具并注册加载的数据以进行渲染。

## 源码

`examples/gltf_scene.rs`

## 相关功能标志

loader、renderer

## 运行或检查

```sh
cargo run -p scenekit --example gltf_scene --features "loader renderer"
```

## 查看内容

- 示例应使用列出的功能编译。
- CPU 示例不应需要 GPU 设置。
- 渲染器示例可能需要工作的原生图形后端或无头支持。

## 相关文档

- [示例索引](README.md)
- [功能标志](../concepts/feature-flags.md)
