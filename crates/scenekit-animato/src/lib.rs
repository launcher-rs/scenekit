#![cfg_attr(not(feature = "std"), no_std)]

//! scenekit 的 Animato 桥接类型。
//!
//! 本 crate 保留 Animato 作为时序/插值引擎，并提供 scenekit 原生的
//! 场景节点、相机、PBR 材质、灯光、变形权重和显式骨骼姿态数组适配器。
//!
//! Scenix v1.4.0 在过程式 Animato 补间/弹簧轨道之上添加了基于片段的
//! 动画运行时（`AnimationClip`、`AnimationAction`、`AnimationMixer`），
//! 以及属性绑定、循环模式、标记/事件、交叉淡入淡出、叠加混合、重定向、
//! 灯光/变形目标和确定性采样。
//!
//! Animato 1.7.0 是目标发布版本；scenekit 桥接使用稳定的 `std`、
//! `tween`、`spring` 和 `serde` feature 集。

extern crate alloc;

mod action;
mod binding;
mod camera;
mod clip;
mod driver;
mod events;
#[cfg(feature = "inspector")]
mod inspector;
mod keyframe;
mod light;
mod loop_mode;
mod material;
mod mixer;
mod morph;
mod retarget;
mod scene;
mod skeleton;
mod tracks;
mod values;

pub use action::{ActionHandle, ActionState, AnimationAction, BlendMode};
pub use animato::{Easing, SpringConfig};
pub use binding::{
    BindingKey, BoneProperty, CameraProperty, LightProperty, MaterialProperty, NodeProperty,
    PropertyBinding,
};
pub use camera::{
    CameraAnimationTarget, CameraAnimator, CameraStoreMut, CameraStores, OrthographicBounds,
    OrthographicBoundsTrack,
};
pub use clip::{AnimationClip, AnimationMarker, ClipChannel, ClipTrack};
pub use driver::{DriverStats, ScenixAnimationDriver};
pub use events::{AnimationEvent, MixerTickResult};
pub use keyframe::{
    KeyframeBool, KeyframeColor, KeyframeInterpolation, KeyframeQuat, KeyframeScalar, KeyframeVec3,
};
pub use light::{LightAnimationTarget, LightAnimator, LightStoreMut, LightStores};
pub use loop_mode::{LoopAdvance, LoopMode};
pub use material::{MaterialAnimationTarget, MaterialAnimator, PbrMaterialStoreMut};
pub use mixer::AnimationMixer;
pub use morph::{MorphWeightAnimator, MorphWeightStoreMut};
pub use retarget::{RetargetEntry, RetargetMap};
pub use scene::{NodeAnimationTarget, NodeAnimator};
pub use skeleton::{BoneAnimation, BoneAnimationTarget, SkeletonPose, SkinnedMeshAnimator};
pub use tracks::{BoolTrack, ColorTrack, QuatTrack, ScalarTrack, Vec3Track};
pub use values::{AnimColor, AnimQuat, AnimVec3};
