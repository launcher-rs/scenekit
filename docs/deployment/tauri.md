# Tauri 部署

## 使用场景

正常使用 Scenix CPU crate，并将渲染器/WASM 决策与 Tauri 窗口策略集成。

## 命令或配置

```toml
scenekit = { version = "1", features = ["renderer"] }
```

## 注意事项

- 保持资产小，避免将大模型打包到发布的 crate 中。
- 将渲染器初始化错误呈现给用户。
- 对你宣传的每个目标使用 CI 检查。