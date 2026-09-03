# Leptos 部署

## 使用场景

使用 CSR 进行静态托管，并将网站 crate 与主工作区依赖图分离。

## 命令或配置

```toml
leptos = { version = "0.8", default-features = false, features = ["csr"] }
```

## 注意事项

- 保持资产小，避免将大模型打包到发布的 crate 中。
- 将渲染器初始化错误呈现给用户。
- 对你宣传的每个目标使用 CI 检查。