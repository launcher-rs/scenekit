# 部署到 GitHub Pages

## 目标

构建静态 Leptos CSR 网站，用于 `/scenekit/`。

## 相关功能标志

网站 crate 使用支持 WASM 的 `scenekit`。

## 步骤

1. 添加所需的 Cargo 功能。
2. 在调用者拥有的存储中保存 CPU 场景数据。
3. 变换或层次结构编辑后调用 `update_world_transforms()`。
4. 仅在启用这些系统时向可选系统注册资源。

## 示例

```sh
cd website
trunk build --release --public-url /scenekit/
```

## 验证

Pages 工作流上传 `website/dist` 并将其部署为静态制品。

## 相关文档

- [快速开始](../quick-start.md)
- [功能标志](../concepts/feature-flags.md)