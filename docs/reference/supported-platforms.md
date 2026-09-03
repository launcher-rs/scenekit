# 支持的平台

## 原生

CPU crate 支持稳定 Rust。渲染器路径使用 `wgpu` 并依赖于主机图形后端。

## 浏览器

WASM 构建目标为 `wasm32-unknown-unknown`。浏览器渲染首先尝试 WebGPU，然后作为完整回退路径的 WebGL2，然后为旧浏览器提供精简的 WebGL1。

## no_std

由 `no_std.md` 中列出的 CPU crate 支持：math、core、input、scene、camera、mesh、material、light、texture、raycaster、helpers 和 animato 无默认检查。

## CI 命令

```sh
cargo test --workspace --all-features
cargo check -p scenekit-wasm --target wasm32-unknown-unknown --all-features
```
