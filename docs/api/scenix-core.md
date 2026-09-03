# `scenekit-core`

## 角色

跨 crate 使用的共享 ID、颜色、错误和 trait。

## 依赖权重

轻量级 `no_std`。

## 安装

```toml
[dependencies]
scenekit-core = "1"
```

## 关键公共 API

NodeId、MeshId、MaterialId、TextureId、LightId、CameraId、Color、ScenixError、ValidationError

## 常见用法

```rust
use scenekit_core::{Color, MeshId};
let mesh = MeshId::new(1);
let color = Color::rgb(1.0, 0.8, 0.2);
# let _ = (mesh, color);
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)