# 创建 PBR 材质

## 目标

配置金属粗糙度材质数据以进行渲染器注册。

## 相关功能标志

默认 `material`；`renderer` 用于 GPU 预览。

## 步骤

1. 添加所需的 Cargo 功能。
2. 在调用者拥有的存储中保存 CPU 场景数据。
3. 变换或层次结构编辑后调用 `update_world_transforms()`。
4. 仅在启用这些系统时向可选系统注册资源。

## 示例

```rust
use scenekit::{Color, PbrMaterial};

let material = PbrMaterial::new()
    .albedo(Color::rgb(0.8, 0.6, 0.3))
    .metallic(0.1)
    .roughness(0.45);
# let _ = material;
```

## 验证

运行 `cargo run -p scenekit --example pbr_sphere --features renderer`。

## 相关文档

- [快速开始](../quick-start.md)
- [功能标志](../concepts/feature-flags.md)
