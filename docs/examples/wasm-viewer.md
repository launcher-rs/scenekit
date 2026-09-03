# WASM 查看器

## 目的

编译独立的浏览器查看器 crate。其 `start` 导出返回一个以 WebGPU 为先的 `BrowserRenderer`，具有 WebGL 回退，包括 v1.5 触摸、指针锁、游戏手柄、变换模式、选择和检查器方法。

## 源码

`examples/wasm_viewer`

## 相关功能标志

wasm 目标

## 运行或检查

```sh
cargo check --manifest-path examples/wasm_viewer/Cargo.toml --target wasm32-unknown-unknown
```

## 查看内容

- 示例应使用列出的功能编译。
- CPU 示例不应需要 GPU 设置。
- 渲染器示例可能需要工作的原生图形后端或无头支持。

## 相关文档

- [示例索引](README.md)
- [功能标志](../concepts/feature-flags.md)