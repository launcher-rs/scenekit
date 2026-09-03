# Web WASM 部署

## 使用场景

为 `wasm32-unknown-unknown` 编译浏览器代码并提供生成的静态文件。

## 命令或配置

```sh
rustup target add wasm32-unknown-unknown
```

## 注意事项

- 保持资产小，避免将大模型打包到发布的 crate 中。
- 将渲染器初始化错误呈现给用户。
- 对你宣传的每个目标使用 CI 检查。
