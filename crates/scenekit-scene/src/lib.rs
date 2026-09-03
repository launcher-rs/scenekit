#![cfg_attr(not(feature = "std"), no_std)]

//! 不依赖 GPU 的场景图类型，用于 scenekit。
//!
//! 本 crate 负责场景节点层级结构、局部变换、缓存的世界变换、
//! 遍历、雾效设置、精灵和 LOD 辅助功能。它不依赖渲染器、网格、
//! 材质、加载器或平台 crate。

extern crate alloc;

pub mod editor;
pub mod fog;
pub mod graph;
#[cfg(feature = "inspector")]
mod inspector;
pub mod iter;
pub mod lod;
pub mod node;
pub mod sprite;

pub use editor::{
    LayerMask, LayerPolicy, NodeEditorMetadata, SelectionChange, SelectionMode, SelectionState,
    SnapSettings, TransformConstraint, TransformMode, TransformSpace,
};
pub use fog::Fog;
pub use graph::SceneGraph;
pub use iter::{BreadthFirstIter, DepthFirstIter};
pub use lod::LodGroup;
pub use node::{NodeKind, SceneNode};
pub use sprite::{BillboardMode, Sprite};
