# 交互和编辑器原语

Scenix 将平台输入、交互策略、场景状态、UI 模型和 GPU 加速分离。这使得相机移动和选择测试无需窗口即可进行，并允许原生和浏览器应用程序共享行为。

## 帧生命周期

将平台事件转发到 `InputState`，更新控件和交互一次，然后调用 `end_frame`。按住的键和按钮保留；增量和边缘转换被清除。

```rust
use scenekit::{ArcballController, InputState, PointerButton, Vec2, Vec3};

let mut input = InputState::default();
input.on_pointer_down(PointerButton::Left);
input.on_pointer_move(Vec2::new(18.0, 4.0));

let mut controls = ArcballController::new(Vec3::ZERO, 4.0);
let camera_transform = controls.update_from_input(&input, 1.0 / 60.0);
input.end_frame();
# let _ = camera_transform;
```

指针和触摸增量表示事件位移，不乘以帧时间。连续键盘和游戏手柄移动按帧时间缩放。

## 选择和策略

`SceneGraph` 是选定、悬停和活动节点的权威。`SelectionMode` 使替换/添加/切换/删除操作具有确定性。稀疏的 `NodeEditorMetadata` 仅在需要时添加标签和应用程序数据；图层遮罩和 `LayerPolicy` 门控选择、可见性和变换。

## 拾取路径

对于便携式精确网格命中和 BVH 加速的矩形框/视锥体选择，使用 CPU `Raycaster`。当 WebGPU 对象 ID、法线、深度和重建世界位置有用时使用 `Renderer::pick`。渲染器路径是按需的，不替代 CPU 回退。

## 可逆变换

`DragController` 和 `TransformController` 捕获起始变换。`end` 提交操作；`cancel` 恢复操作。捕捉通过 `SnapSettings` 配置，而变换模式、坐标空间和轴/平面约束保持为可由任何 UI 驱动的显式值。

## UI 边界

`Inspectable` 返回拥有的、类型化的 `InspectorSnapshot`。主机可以使用可选的 egui 适配器渲染它，将浏览器端快照序列化为 JSON，或将其映射到另一个 UI 工具包，而不会将 CPU crate 耦合到该工具包。