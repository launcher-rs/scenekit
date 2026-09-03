# 为 WASM 构建

## 目标

编译浏览器包装器和 WASM 查看器示例。

## 相关功能标志

`wasm`；`BrowserRenderer` 在安全时使用 WebGPU，当 WebGPU 不可用时使用 WebGL2 作为完整的浏览器回退，仅作为精简的最后手段回退使用 WebGL1。

## 步骤

1. 添加所需的 Cargo 功能。
2. 在调用者拥有的存储中保存 CPU 场景数据。
3. 变换或层次结构编辑后调用 `update_world_transforms()`。
4. 仅在启用这些系统时向可选系统注册资源。

## 示例

```sh
rustup target add wasm32-unknown-unknown
cargo check -p scenekit-wasm --target wasm32-unknown-unknown --all-features
cargo check --manifest-path examples/wasm_viewer/Cargo.toml --target wasm32-unknown-unknown
```

## 验证

对于没有 WebGPU 或 WebGL 的浏览器，使用网站回退路径。在没有 WebGPU 的普通浏览器中，`scenekit-wasm` 应仍通过 WebGL2 渲染；WebGL1 保留用于具有精简对等性的旧浏览器。

## 相关文档

- [快速开始](../quick-start.md)
- [功能标志](../concepts/feature-flags.md)