# 添加灯光和阴影

## 目标

注册灯光数据并启用支持阴影的渲染器示例。

## 相关功能标志

默认 `light`；`renderer` 用于阴影。

## 步骤

1. 添加所需的 Cargo 功能。
2. 在调用者拥有的存储中保存 CPU 场景数据。
3. 变换或层次结构编辑后调用 `update_world_transforms()`。
4. 仅在启用这些系统时向可选系统注册资源。

## 示例

```rust
use scenekit::{DirectionalLight, Vec3};

let sun = DirectionalLight::new(Vec3::new(-1.0, -2.0, -1.0));
# let _ = sun;
```

## 验证

运行 `cargo run -p scenekit --example shadow_demo --features renderer`。

## 相关文档

- [快速开始](../quick-start.md)
- [功能标志](../concepts/feature-flags.md)