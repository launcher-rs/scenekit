# 桌面部署

## 使用场景

构建默认使用 CPU 创作的原生应用，可选的 `renderer` 用于 `wgpu` 输出。

## 命令或配置

```sh
cargo build --release --features renderer
```

## 注意事项

- 保持资产小，避免将大模型打包到发布的 crate 中。
- 将渲染器初始化错误呈现给用户。
- 对你宣传的每个目标使用 CI 检查。