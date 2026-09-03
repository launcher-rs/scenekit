# Dioxus 部署

## 使用场景

使用 Scenix 作为场景/数据层，并根据 Dioxus 目标和 WebGPU 可用性集成渲染。

## 命令或配置

```toml
scenekit = { version = "1", features = ["wasm"] }
```

## 注意事项

- 保持资产小，避免将大模型打包到发布的 crate 中。
- 将渲染器初始化错误呈现给用户。
- 对你宣传的每个目标使用 CI 检查。