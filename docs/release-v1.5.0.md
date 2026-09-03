# Scenix v1.5.0 — 控件、交互和编辑器原语

版本 1.5 将现有的场景、相机、光线投射器、渲染器和浏览器层转变为一个连贯的交互工具包。应用程序可以将平台事件保留在其边界，馈送一个 `InputState`，并在原生和浏览器前端之间共享相同的控件和场景选择模型。

## 数据流

```text
native/DOM events
        |
        v
   InputState --------> camera controls
        |                    |
        v                    v
 ray/frustum pick ------> SceneGraph selection
        |                    |
        v                    v
 drag/transform <------ gizmo handle
        |
        +----> InspectorSnapshot ----> egui / JSON / custom UI
        |
        +----> optional Renderer ID + normal + depth readback
```

CPU 选择和变换操作无需渲染器即可工作。GPU 拾取是一个可选的精度路径，仅在第一次编辑器请求时分配其目标和暂存缓冲区，之后重用它们。

## 新公共表面

- `scenekit-input`：`InputState`、`TouchState`、`GestureRecognizer`、`GamepadStates`、`PointerLockState` 和 `ViewportMetrics`。
- `scenekit-camera`：`ArcballController`、`TrackballController`、`MapController`、`FirstPersonController` 和 `PointerLockController`。
- `scenekit-scene`：选择状态/模式、编辑器元数据和策略、`LayerMask`、`TransformMode`、`TransformSpace`、`TransformConstraint` 和 `SnapSettings`。
- `scenekit-raycaster`：`SelectionRect`、`SelectionFrustum`、`DragPlane`、`DragController` 和 `TransformController`。
- `scenekit-helpers`：保留变换小工具、分析句柄、选择和边界辅助工具、捕捉网格，以及 `egui` 后的 `show_inspector`。
- `scenekit-core`：`Inspectable` trait 和类型化检查器快照树。
- `scenekit-renderer`：`EditorPickRequest`、`EditorPickResult`、`EditorBufferStats` 和显式渲染/读取/拾取方法。
- `scenekit-wasm`：DPR 感知指标加上触摸、指针锁、游戏手柄、变换模式和检查器 JSON 转发。

## 选择功能

使用 `interaction` 进行控件、CPU 拾取、拖动、选择和小工具。当 UI 需要类型化检查器模型时使用 `editor`。仅当主机 UI 使用 egui 时才添加 `egui`。独立启用 `renderer` 用于渲染和按需 GPU 拾取；启用 `wasm` 用于浏览器绑定。

```toml
[dependencies]
scenekit = { version = "1.5", features = ["editor", "egui", "renderer"] }
```

## 性能设计

- 触摸和游戏手柄存储是固定容量的。
- 瞬态输入由 `InputState::end_frame` 就地清除。
- 控件消耗借用的快照，每次更新不分配。
- BVH 和光线投射器 API 可以写入调用者拥有的输出缓冲区。
- 小工具和辅助几何体可以重新生成到保留缓冲区中。
- 场景编辑器元数据是稀疏的，因此普通运行时节点不支付元数据分配成本。
- GPU 拾取使用密集临时对象 ID、`pick` 的一像素裁剪、容量增长统一变量和持久读回缓冲区。

## 迁移

没有 v1.4 交互 API 被移除。现有的 `KeyboardState`、`PointerState`、轨道/飞行控件、场景遍历和 `Raycaster::cast_ray` 调用继续工作。应用程序可以通过首先将平台事件聚合到 `InputState` 中，然后采用选择或变换原语来逐步迁移。

## 验证

该发布受单元和集成测试、Rust 1.89 和稳定检查、所有外观功能通道、无默认 CPU 构建、所有示例、`wasm32-unknown-unknown`、独立浏览器查看器、Leptos 网站、rustdoc 警告即错误、打包和 Vulkan/lavapipe GPU 拾取测试的覆盖。

相关阅读：

- [交互和编辑器概念](concepts/interaction-and-editor.md)
- [示例](examples/README.md)
- [GitHub 发布说明](../.github/release-notes/1.5.0.md)