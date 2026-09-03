# GitHub Pages 部署

## 使用场景

构建静态 Leptos CSR 网站，使用 `/scenekit/` 作为公共 URL。

## 命令或配置

```sh
cd website
trunk build --release --public-url /scenekit/
```

## 注意事项

- 保持资产小，避免将大模型打包到发布的 crate 中。
- 将渲染器初始化错误呈现给用户。
- 对你宣传的每个目标使用 CI 检查。