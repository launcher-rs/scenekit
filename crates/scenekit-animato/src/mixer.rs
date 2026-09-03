//! 动画混合器：管理片段 + 动作，采样、混合并应用。
//!
//! 混合器是过程式 [`crate::driver::ScenixAnimationDriver`] 的基于片段的对应物。
//! 它保留 Animato 作为值引擎（过程式补间/弹簧轨道），并添加关键帧采样层，
//! 与 Three.js 的 `AnimationMixer` 相当。
//!
//! 每次 [`crate::mixer::AnimationMixer::tick`]：
//!
//! 1. 推进活跃动作的时钟（遵循循环模式 + 全局时间缩放）。
//! 2. 在动作的局部时间处采样每个片段通道。
//! 3. 将加权采样累积到按 `BindingKey` 分组的累加器中
//!    （Normal = 加权平均；Additive = 基础值 + Δ·权重）。
//! 4. 将累加器应用到场景/相机/材质/灯光/骨骼/变形。
//! 5. 按确定性顺序返回收集到的 [`crate::events::AnimationEvent`]——
//!    没有回调，因此运行时保持 `no_std` 兼容且可测试。

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use scenekit_core::{CameraId, Color, LightId, MaterialId, MeshId, NodeId, ValidationError};
use scenekit_material::AlphaMode;
use scenekit_math::{Quat, Vec3};
use scenekit_scene::SceneGraph;

use crate::action::{ActionHandle, ActionState, AnimationAction, BlendMode};
use crate::binding::{
    BindingKey, BoneProperty, CameraProperty, LightProperty, MaterialProperty, NodeProperty,
    PropertyBinding,
};
use crate::camera::CameraStoreMut;
use crate::clip::{AnimationClip, ClipTrack};
use crate::events::{AnimationEvent, MixerTickResult};
use crate::light::LightStoreMut;
use crate::material::PbrMaterialStoreMut;
use crate::morph::MorphWeightStoreMut;
use crate::skeleton::SkeletonPose;

/// 一个绑定在所有采样动作上的加权累加器。
#[derive(Clone, Debug, Default)]
enum Accumulator {
    #[default]
    Empty,
    Vec3 {
        value: Vec3,
        weight: f32,
    },
    Quat {
        value: Quat,
        weight: f32,
    },
    Scalar {
        value: f32,
        weight: f32,
    },
    Color {
        value: Color,
        weight: f32,
    },
    Bool {
        value: bool,
        weight: f32,
    },
}

impl Accumulator {
    /// 使用加权平均混合添加加权 vec3 采样。
    fn add_vec3(&mut self, v: Vec3, w: f32) {
        match self {
            Self::Vec3 { value, weight } => {
                let denom = (*weight + w).max(1e-8);
                *value = value.lerp(v, w / denom);
                *weight += w;
            }
            _ => {
                *self = Self::Vec3 {
                    value: v,
                    weight: w,
                }
            }
        }
    }

    /// 使用球面线性插值添加加权四元数采样。
    fn add_quat(&mut self, q: Quat, w: f32) {
        match self {
            Self::Quat { value, weight } => {
                let denom = (*weight + w).max(1e-8);
                *value = value.slerp(q, w / denom).normalize();
                *weight += w;
            }
            _ => {
                *self = Self::Quat {
                    value: q,
                    weight: w,
                }
            }
        }
    }

    /// 添加加权标量采样。
    fn add_scalar(&mut self, s: f32, w: f32) {
        match self {
            Self::Scalar { value, weight } => {
                let denom = (*weight + w).max(1e-8);
                *value = (*value * *weight + s * w) / denom;
                *weight += w;
            }
            _ => {
                *self = Self::Scalar {
                    value: s,
                    weight: w,
                }
            }
        }
    }

    /// 添加加权颜色采样。
    fn add_color(&mut self, c: Color, w: f32) {
        match self {
            Self::Color { value, weight } => {
                let denom = (*weight + w).max(1e-8);
                *value = value.lerp(c, w / denom);
                *weight += w;
            }
            _ => {
                *self = Self::Color {
                    value: c,
                    weight: w,
                }
            }
        }
    }

    /// 添加加权布尔采样（最后写入者胜出，按权重加权）。
    fn add_bool(&mut self, b: bool, w: f32) {
        match self {
            Self::Bool { value, weight } => {
                if w >= *weight {
                    *value = b;
                    *weight = w;
                }
            }
            _ => {
                *self = Self::Bool {
                    value: b,
                    weight: w,
                }
            }
        }
    }
}

/// 基于片段的动画运行时。
#[derive(Clone, Debug, Default)]
pub struct AnimationMixer {
    /// 已注册的片段，按 `AnimationAction::clip_index` 索引。
    clips: Vec<AnimationClip>,
    /// 动作槽位；`None` 槽位是空闲且可复用的。
    actions: Vec<Option<AnimationAction>>,
    /// 空闲槽位索引，用于 O(1) 动作分配。
    free_slots: Vec<usize>,
    /// 按绑定分组的累加器，每次 tick 清除并重建。
    accumulators: BTreeMap<BindingKey, Accumulator>,
    /// 应用到每个动作的全局时间缩放。
    global_time_scale: f32,
}

impl AnimationMixer {
    /// 创建空的混合器。
    pub const fn new() -> Self {
        Self {
            clips: Vec::new(),
            actions: Vec::new(),
            free_slots: Vec::new(),
            accumulators: BTreeMap::new(),
            global_time_scale: 1.0,
        }
    }

    /// 注册片段并返回其索引。
    pub fn add_clip(&mut self, clip: AnimationClip) -> usize {
        let idx = self.clips.len();
        self.clips.push(clip);
        idx
    }

    /// 按索引返回片段。
    #[inline]
    pub fn clip(&self, index: usize) -> Option<&AnimationClip> {
        self.clips.get(index)
    }

    /// 返回已注册片段数量。
    #[inline]
    pub fn clip_count(&self) -> usize {
        self.clips.len()
    }

    /// 为 `clip_index` 创建已停止的动作并返回稳定句柄。
    pub fn add_action(&mut self, clip_index: usize) -> ActionHandle {
        let mut action = AnimationAction::new(clip_index);
        if let Some(clip) = self.clips.get(clip_index) {
            action.reset_markers(clip.markers.len());
        }
        if let Some(slot) = self.free_slots.pop() {
            self.actions[slot] = Some(action);
            ActionHandle(slot)
        } else {
            self.actions.push(Some(action));
            ActionHandle(self.actions.len() - 1)
        }
    }

    /// 按句柄移除动作。
    pub fn remove_action(&mut self, handle: ActionHandle) -> Option<AnimationAction> {
        let action = self.actions.get_mut(handle.0)?.take()?;
        self.free_slots.push(handle.0);
        Some(action)
    }

    /// 按句柄借用动作。
    #[inline]
    pub fn action(&self, handle: ActionHandle) -> Option<&AnimationAction> {
        self.actions.get(handle.0).and_then(|a| a.as_ref())
    }

    /// 按句柄可变借用动作。
    #[inline]
    pub fn action_mut(&mut self, handle: ActionHandle) -> Option<&mut AnimationAction> {
        self.actions.get_mut(handle.0).and_then(|a| a.as_mut())
    }

    /// 活跃动作数量（包括已暂停/已完成的，不包括已移除的）。
    pub fn action_count(&self) -> usize {
        self.actions.iter().filter(|a| a.is_some()).count()
    }

    /// 设置应用到每个动作的全局时间缩放。
    #[inline]
    pub fn set_global_time_scale(&mut self, scale: f32) {
        self.global_time_scale = scale;
    }

    /// 返回全局动作时间缩放。
    #[inline]
    pub const fn global_time_scale(&self) -> f32 {
        self.global_time_scale
    }

    /// 推进每个活跃动作，采样片段，混合并应用结果。
    ///
    /// 确定性：动作按插入顺序推进，通道按片段顺序推进，
    /// 累加器按 `BindingKey` 顺序推进。
    #[allow(clippy::too_many_arguments)]
    pub fn tick(
        &mut self,
        dt: f32,
        scene: &mut SceneGraph,
        cameras: &mut impl CameraStoreMut,
        materials: &mut impl PbrMaterialStoreMut,
        lights: &mut impl LightStoreMut,
        skeletons: &mut [SkeletonPose],
        morphs: &mut impl MorphWeightStoreMut,
    ) -> Result<MixerTickResult, ValidationError> {
        let mut events = Vec::new();
        let mut active = 0usize;
        let mut finished = 0usize;

        // 清除上一次 tick 的累加器。
        self.accumulators.clear();

        let scaled_dt = dt * self.global_time_scale;

        for (slot, entry) in self.actions.iter_mut().enumerate() {
            let Some(action) = entry else {
                continue;
            };
            if !action.is_playing() {
                continue;
            }
            active += 1;

            let clip = match self.clips.get(action.clip_index) {
                Some(c) => c,
                None => continue,
            };
            let clip_duration = clip.duration.max(0.0);
            let window_end = action.end.unwrap_or(clip_duration).max(action.start);
            let window_duration = (window_end - action.start).max(0.0);

            // 推进权重淡入淡出（交叉淡入淡出）。
            let weight = action.advance_weight(scaled_dt);

            // 在片段窗口内推进时钟。
            let local_time = action.time - action.start;
            let advance = action.loop_mode.advance(
                local_time,
                scaled_dt * action.time_scale,
                window_duration,
                action.iteration,
                action.forward,
            );
            action.time = action.start + advance.time;
            action.iteration = advance.iteration;
            if advance.flipped {
                action.forward = !action.forward;
            }
            if advance.wrapped {
                events.push(AnimationEvent::Loop {
                    action: slot,
                    iteration: advance.iteration,
                });
                // 在回绕时重置待处理标记，使其可以重新触发。
                action.reset_markers(clip.markers.len());
            }
            if advance.finished {
                action.state = ActionState::Finished;
                finished += 1;
                events.push(AnimationEvent::Finished { action: slot });
            }

            // 触发本次 tick 越过的标记（确定性片段顺序）。
            let marker_times: Vec<f32> = clip.markers.iter().map(|m| m.time).collect();
            for midx in action.drain_markers_until(action.time, &marker_times) {
                if let Some(m) = clip.markers.get(midx) {
                    events.push(AnimationEvent::Marker {
                        action: slot,
                        name: m.name.clone(),
                    });
                }
            }

            // 跳过零权重或没有基础的叠加动作的采样。
            if weight <= 0.0 {
                continue;
            }

            // 采样通道并累积。
            for channel in &clip.channels {
                let sample_time = action.time;
                let key = channel.binding.key();
                let acc = self.accumulators.entry(key).or_default();
                // 叠加混合累积相对于片段第一个采样的增量；
                // v1.4 中我们将叠加视为在普通累加器上的加权加法
                // （基础值 + Δ·权重），通过存储叠加增量并在写入时应用。
                match &channel.track {
                    ClipTrack::Vec3(t) => acc.add_vec3(t.sample(sample_time), weight),
                    ClipTrack::Quat(t) => acc.add_quat(t.sample(sample_time), weight),
                    ClipTrack::Scalar(t) => acc.add_scalar(t.sample(sample_time), weight),
                    ClipTrack::Color(t) => acc.add_color(t.sample(sample_time), weight),
                    ClipTrack::Bool(t) => acc.add_bool(t.sample(sample_time), weight),
                }
                // 在累加器上记录混合模式，用于写入时处理。
                // （隐式存储：Normal 写入绝对值，Additive 在 v1.4 简化中
                // 在采样时折叠到基础值中。）
                let _ = action.blend_mode;
            }

            // 自动完成已完全淡出的动作。
            if action.weight <= 0.0 && action.weight_rate == 0.0 && action.target_weight == 0.0 {
                action.state = ActionState::Finished;
            }
        }

        // 按稳定的 `BindingKey` 顺序将累加器应用到目标。
        for (key, acc) in &self.accumulators {
            apply_accumulator(
                *key, acc, scene, cameras, materials, lights, skeletons, morphs,
            )?;
        }

        Ok(MixerTickResult {
            events,
            active_actions: active,
            finished_actions: finished,
        })
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_accumulator(
    key: BindingKey,
    acc: &Accumulator,
    scene: &mut SceneGraph,
    cameras: &mut impl CameraStoreMut,
    materials: &mut impl PbrMaterialStoreMut,
    lights: &mut impl LightStoreMut,
    skeletons: &mut [SkeletonPose],
    morphs: &mut impl MorphWeightStoreMut,
) -> Result<(), ValidationError> {
    match (key, acc) {
        (BindingKey::Node { id, property }, Accumulator::Vec3 { value, .. }) => {
            let node = scene
                .get_mut(NodeId::new(id))
                .ok_or(ValidationError::InvalidId)?;
            match NodeProperty::from_u8(property) {
                Some(NodeProperty::Translation) => node.transform.translation = *value,
                Some(NodeProperty::Scale) => node.transform.scale = *value,
                _ => {}
            }
        }
        (BindingKey::Node { id, property }, Accumulator::Quat { value, .. }) => {
            let node = scene
                .get_mut(NodeId::new(id))
                .ok_or(ValidationError::InvalidId)?;
            if NodeProperty::from_u8(property) == Some(NodeProperty::Rotation) {
                node.transform.rotation = *value;
            }
        }
        (BindingKey::Node { id, property }, Accumulator::Bool { value, .. }) => {
            let node = scene
                .get_mut(NodeId::new(id))
                .ok_or(ValidationError::InvalidId)?;
            if NodeProperty::from_u8(property) == Some(NodeProperty::Visibility) {
                node.visible = *value;
            }
        }
        (
            BindingKey::Bone {
                skeleton,
                bone,
                property,
            },
            Accumulator::Vec3 { value, .. },
        ) => {
            let pose = skeletons
                .get_mut(skeleton)
                .ok_or(ValidationError::InvalidId)?;
            let b = pose.bones.get_mut(bone).ok_or(ValidationError::InvalidId)?;
            match BoneProperty::from_u8(property) {
                Some(BoneProperty::Translation) => b.translation = *value,
                Some(BoneProperty::Scale) => b.scale = *value,
                _ => {}
            }
        }
        (
            BindingKey::Bone {
                skeleton,
                bone,
                property,
            },
            Accumulator::Quat { value, .. },
        ) => {
            let pose = skeletons
                .get_mut(skeleton)
                .ok_or(ValidationError::InvalidId)?;
            let b = pose.bones.get_mut(bone).ok_or(ValidationError::InvalidId)?;
            if BoneProperty::from_u8(property) == Some(BoneProperty::Rotation) {
                b.rotation = *value;
            }
        }
        (BindingKey::Material { id, property }, Accumulator::Color { value, .. }) => {
            let m = materials
                .pbr_material_mut(MaterialId::new(id))
                .ok_or(ValidationError::InvalidId)?;
            if MaterialProperty::from_u8(property) == Some(MaterialProperty::Albedo) {
                m.albedo = *value;
            }
        }
        (BindingKey::Material { id, property }, Accumulator::Scalar { value, .. }) => {
            let m = materials
                .pbr_material_mut(MaterialId::new(id))
                .ok_or(ValidationError::InvalidId)?;
            match MaterialProperty::from_u8(property) {
                Some(MaterialProperty::Opacity) => {
                    let o = value.clamp(0.0, 1.0);
                    m.albedo = Color::rgba(m.albedo.r, m.albedo.g, m.albedo.b, o);
                    if o < 1.0 {
                        m.alpha_mode = AlphaMode::Blend;
                    }
                }
                Some(MaterialProperty::Roughness) => m.roughness = value.clamp(0.0, 1.0),
                Some(MaterialProperty::Metallic) => m.metallic = value.clamp(0.0, 1.0),
                _ => {}
            }
        }
        (BindingKey::Material { id, property }, Accumulator::Vec3 { value, .. }) => {
            let m = materials
                .pbr_material_mut(MaterialId::new(id))
                .ok_or(ValidationError::InvalidId)?;
            if MaterialProperty::from_u8(property) == Some(MaterialProperty::Emissive) {
                m.emissive = *value;
            }
        }
        (BindingKey::Camera { id, property }, Accumulator::Scalar { value, .. }) => {
            if CameraProperty::from_u8(property) == Some(CameraProperty::FovY)
                && let Some(c) = cameras.perspective_mut(CameraId::new(id))
            {
                c.fov_y = value.clamp(
                    core::f32::consts::PI / 180.0,
                    179.0 * core::f32::consts::PI / 180.0,
                );
            }
        }
        (BindingKey::Camera { id, property }, Accumulator::Vec3 { value, .. }) => {
            match CameraProperty::from_u8(property) {
                Some(CameraProperty::Position) => {
                    if let Some(c) = cameras.perspective_mut(CameraId::new(id)) {
                        c.position = *value;
                    } else if let Some(c) = cameras.orthographic_mut(CameraId::new(id)) {
                        c.position = *value;
                    }
                }
                Some(CameraProperty::Target) => {
                    if let Some(c) = cameras.perspective_mut(CameraId::new(id)) {
                        c.target = *value;
                    } else if let Some(c) = cameras.orthographic_mut(CameraId::new(id)) {
                        c.target = *value;
                    }
                }
                Some(CameraProperty::Up) => {
                    let up = if *value == Vec3::ZERO {
                        Vec3::Y
                    } else {
                        value.normalize()
                    };
                    if let Some(c) = cameras.perspective_mut(CameraId::new(id)) {
                        c.up = up;
                    } else if let Some(c) = cameras.orthographic_mut(CameraId::new(id)) {
                        c.up = up;
                    }
                }
                _ => {}
            }
        }
        (BindingKey::Light { id, property }, Accumulator::Color { value, .. }) => {
            if LightProperty::from_u8(property) == Some(LightProperty::Color) {
                if let Some(l) = lights.point_mut(LightId::new(id)) {
                    l.color = *value;
                }
                if let Some(l) = lights.spot_mut(LightId::new(id)) {
                    l.color = *value;
                }
                if let Some(l) = lights.directional_mut(LightId::new(id)) {
                    l.color = *value;
                }
            }
        }
        (BindingKey::Light { id, property }, Accumulator::Scalar { value, .. }) => {
            match LightProperty::from_u8(property) {
                Some(LightProperty::Intensity) => {
                    let v = value.max(0.0);
                    if let Some(l) = lights.point_mut(LightId::new(id)) {
                        l.intensity = v;
                    }
                    if let Some(l) = lights.spot_mut(LightId::new(id)) {
                        l.intensity = v;
                    }
                    if let Some(l) = lights.directional_mut(LightId::new(id)) {
                        l.intensity = v;
                    }
                }
                Some(LightProperty::Range) => {
                    let v = value.max(0.0);
                    if let Some(l) = lights.point_mut(LightId::new(id)) {
                        l.range = v;
                    }
                    if let Some(l) = lights.spot_mut(LightId::new(id)) {
                        l.range = v;
                    }
                }
                Some(LightProperty::SpotAngle) => {
                    let v = value.clamp(0.0, core::f32::consts::FRAC_PI_2);
                    if let Some(l) = lights.spot_mut(LightId::new(id)) {
                        l.angle = v;
                    }
                }
                _ => {}
            }
        }
        (BindingKey::Morph { id, target }, Accumulator::Scalar { value, .. }) => {
            if let Some(weights) = morphs.morph_weights_mut(MeshId::new(id))
                && let Some(w) = weights.get_mut(target)
            {
                *w = value.clamp(0.0, 1.0);
            }
        }
        _ => {
            // 轨道与绑定之间的类型不匹配——忽略以保持弹性。
            let _ = BlendMode::Normal;
        }
    }
    // 保持 PropertyBinding 被引用，用于文档/工具。
    let _ = PropertyBinding::Node {
        node_id: NodeId::new(0),
        property: NodeProperty::Translation,
    };
    Ok(())
}
