# 纹理

## 目的

存储原始 CPU 像素数据、采样器、mipmap、图集和视频帧更新。

## 何时使用

当子系统影响你的应用架构或依赖项选择时阅读此页面。有关实现步骤，请将其与 `../guides/` 中的相应指南配对。

## 相关功能标志

默认外观功能包括 `texture`；图像解码需要 `loader`。

## 关键规则

- 纹理 crate 本身不解码 PNG/JPEG。
- 对于图像文件使用 `scenekit-loader`。
- 显式向渲染器注册纹理。


## 示例

```rust
use scenekit::{Sampler, Texture2D, TextureFormat};

let data = vec![255; 4 * 4 * 4];
let texture = Texture2D::new(4, 4, TextureFormat::Rgba8UnormSrgb, data).unwrap();
let sampler = Sampler::default();
# let _ = (texture, sampler);
```

## 相关文档

- [功能标志](feature-flags.md)
- [架构概述](architecture-overview.md)
- [API 参考](../api/facade-crate.md)