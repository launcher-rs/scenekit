# Scenix v1.4.0 — 动画运行时

Scenix `1.4.0` 在现有的 Animato 缓动/弹簧桥接基础上添加了基于剪辑的动画运行时，使 scenekit 与 Three.js 的 `AnimationClip` / `AnimationAction` / `AnimationMixer` 模型对等，同时保持 scenekit 的类型化 ID、确定性和渲染器无关的规则。

## 亮点

- 工作区 crate 升级到 `1.4.0`；Animato 升级到 `1.7.0`（之前的 `1.6.0` 发布门控现在已解决）。
- 新运行时：`AnimationClip`、`AnimationAction`、`AnimationMixer`、`PropertyBinding`、`LoopMode`、`AnimationMarker` / `AnimationEvent`、交叉淡入淡出、累加混合、确定性每帧采样。
- 新关键帧轨道（`KeyframeScalar` / `Vec3` / `Quat` / `Color` / `Bool`），具有 `Linear`、`Step` 和 `CubicSpline` 插值。
- 新灯光（`LightAnimator`）和变形权重（`MorphWeightAnimator`）目标。
- 骨骼动画：`scenekit-mesh` `SkinningData` + `cpu_skin` / `apply_morph` CPU 回退；`scenekit-renderer` `register_skin` / `update_bone_matrices` / `register_morph_targets` / `update_morph_weights` GPU 钩子 + `SKINNING_WGSL`。
- 重定向辅助工具（`RetargetMap`）和动画/姿势调试辅助工具（`AnimationPathHelper`、`PoseHelper`）。
- 加载器现在将动画访问器输出字节解码为 `LoadedAnimationChannel::output`。
- 外观 `clip_from_loaded` 将导入的剪辑桥接到运行时。

## 安装

```toml
[dependencies]
scenekit = "1.4"
```

动画运行时：

```toml
[dependencies]
scenekit = { version = "1.4", features = ["animato"] }
```

带有导入剪辑的动画运行时：

```toml
[dependencies]
scenekit = { version = "1.4", features = ["animato", "loader"] }
```

## 代码示例

```rust
use scenekit::{
    AnimationClip, AnimationMixer, ClipChannel, ClipTrack, KeyframeInterpolation,
    KeyframeVec3, LoopMode, NodeProperty, PropertyBinding, SceneGraph, SceneNode, Vec3,
};

let mut scene = SceneGraph::new();
let node = scene.add(SceneNode::new("mover"));

let clip = AnimationClip::empty("move").with_channel(ClipChannel {
    binding: PropertyBinding::Node { node_id: node, property: NodeProperty::Translation },
    track: ClipTrack::Vec3(KeyframeVec3::new(
        vec![0.0, 1.0],
        vec![Vec3::ZERO, Vec3::new(5.0, 0.0, 0.0)],
        KeyframeInterpolation::Linear,
    )),
});

let mut mixer = AnimationMixer::new();
let clip_index = mixer.add_clip(clip);
let action = mixer.add_action(clip_index);
mixer.action_mut(action).unwrap().set_loop_mode(LoopMode::REPEAT);
mixer.action_mut(action).unwrap().play(0.0);

// 每帧：
// mixer.tick(dt, &mut scene, &mut cameras, &mut materials,
//            &mut lights, &mut skeletons, &mut morphs)
```

## 迁移说明

- `ScenixAnimationDriver::tick` 现在除了 `skeletons` 外还接受 `lights` 和 `morphs` 存储。如果未使用，请传递空存储。
- `LoadedAnimationChannel` 获得了 `output: Vec<f32>` 字段；相应更新结构体字面量。
- Animato `1.5.0` → `1.7.0` 对于 `std` / `tween` / `spring` / `serde` 功能集是即插即用的；不需要 scenekit 端更改。

## 已知限制

- 三次样条插值完全实用于标量轨道；vec3 / quat / color 三次通道在 v1.4 中回退到线性采样。
- 累加混合将加权增量累积到相同的普通累加器中用于 v1.4；完全的基于基础剪辑的累加计划中。
- GPU 蒙皮发布了注册表、上传钩子和 `SKINNING_WGSL` 代码片段；完全的着色器管线接线在 `SKINNING` 定义后面，在后续补丁中累加。

## 链接

- 网站和演示：`https://aarambhdevhub.github.io/scenekit/`
- 文档：`https://docs.rs/scenekit`
- Crates：`https://crates.io/crates/scenekit`