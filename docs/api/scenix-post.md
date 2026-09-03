# `scenekit-post`

## 角色

可选的 GPU 后处理堆栈和效果配置。

## 依赖权重

重 `std`/`wgpu` 路径；与渲染器一起使用。

## 安装

```toml
[dependencies]
scenekit-post = "1"
```

## 关键公共 API

PostStack、BloomConfig、SsaoConfig、ToneMapper、FxaaConfig、TaaConfig、SmaaConfig、DofConfig、FogPostConfig、OutlineConfig、MotionBlurConfig

## 常见用法

```rust
use scenekit_post::{PostStack, ToneMapper};
let stack = PostStack::new().with_tonemap(ToneMapper::Aces);
# let _ = stack;
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)