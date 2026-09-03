# `scenekit-texture`

## 角色

原始 CPU 纹理、采样器、mipmap、图集和视频帧更新。

## 依赖权重

轻量级 `no_std`；默认外观功能。

## 安装

```toml
[dependencies]
scenekit-texture = "1"
```

## 关键公共 API

Texture2D、TextureCube、Texture3D、TextureFormat、Sampler、TextureAtlas、VideoTexture

## 常见用法

```rust
use scenekit_texture::{Sampler, TextureFormat};
let format = TextureFormat::Rgba8UnormSrgb;
let sampler = Sampler::default();
# let _ = (format, sampler);
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)