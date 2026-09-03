#![cfg_attr(not(feature = "std"), no_std)]

//! scenekit 的 CPU 端 BVH 光线投射和场景拾取。
//!
//! 本 crate 与渲染器无关。调用者提供 `SceneGraph` 和网格几何存储，
//! 然后光线投射返回与网格三角形的世界空间交点。

extern crate alloc;

pub mod bvh;
pub mod interaction;
pub mod intersection;
pub mod raycaster;

pub use bvh::{Bvh, BvhEntry, BvhNode};
pub use interaction::{
    DragController, DragPlane, InteractionError, InteractionUpdate, SelectionContainment,
    SelectionFrustum, SelectionRect, TransformController,
};
pub use intersection::Intersection;
pub use raycaster::{GeometryProvider, Raycaster};
