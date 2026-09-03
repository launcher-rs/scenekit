# `scenekit-helpers`

## 角色

调试线几何体、保留变换小工具、编辑器选择视觉效果、捕捉网格和可选的 egui 检查器适配器。

## 依赖权重

轻量级 `no_std`；默认外观功能。

## 安装

```toml
[dependencies]
scenekit-helpers = "1"
```

## 关键公共 API

`LineGeometry`、`GridHelper`、`AxesHelper`、`BoundingBoxHelper`、`ArrowHelper`、`CameraHelper`、灯光辅助工具、`SkeletonHelper`、`TransformGizmoHelper`、`GizmoGeometry`、`SelectionHelper`、`BoundsGizmoHelper`、`SnapGridHelper` 和 `egui` 后的 `show_inspector`。

## 常见用法

```rust
use scenekit_helpers::AxesHelper;
let axes = AxesHelper::new(1.0).geometry();
# let _ = axes;
```

## 注意事项

当你需要自己的公共 API 中的边界时直接使用此 crate。构建应用程序时使用 `scenekit` 外观，当你想要一个稳定的导入表面时。

## 相关文档

- [功能标志](../concepts/feature-flags.md)
- [变换小工具示例](../examples/transform-gizmo.md)
- [编辑器检查器示例](../examples/editor-inspector.md)
- [Crate 依赖关系图](../reference/crate-dependency-map.md)