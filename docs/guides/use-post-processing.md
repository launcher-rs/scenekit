# 使用后处理

## 目标

将后处理堆栈附加到渲染器输出，用于泛光、SSAO、色调映射和抗锯齿。

## 相关功能标志

`renderer`、`post`

## 步骤

1. 添加所需的 Cargo 功能。
2. 在调用者拥有的存储中保存 CPU 场景数据。
3. 变换或层次结构编辑后调用 `update_world_transforms()`。
4. 仅在启用这些系统时向可选系统注册资源。

## 示例

```rust
use scenekit::{FxaaConfig, PostStack, ToneMapper};

let stack = PostStack::new()
    .with_tonemap(ToneMapper::Aces)
    .with_fxaa(FxaaConfig::default());
# let _ = stack;
```

## 验证

运行 `cargo run -p scenekit --example post_processing --features "renderer post"`。

## 相关文档

- [快速开始](../quick-start.md)
- [功能标志](../concepts/feature-flags.md)