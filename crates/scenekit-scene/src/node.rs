use alloc::string::String;

#[cfg(feature = "std")]
use scenekit_core::Named;
use scenekit_core::{CameraId, LightId, MaterialId, MeshId};
use scenekit_math::Transform;

use crate::Sprite;

/// 附加到场景节点的渲染或逻辑负载。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NodeKind {
    /// 无负载。
    #[default]
    Empty,
    /// 逻辑分组节点。
    Group,
    /// 网格可渲染对象。
    Mesh {
        /// 网格资源标识符。
        mesh_id: MeshId,
        /// 材质资源标识符。
        material_id: MaterialId,
    },
    /// 光源附件。
    Light {
        /// 光源资源标识符。
        light_id: LightId,
    },
    /// 相机附件。
    Camera {
        /// 相机资源标识符。
        camera_id: CameraId,
    },
    /// 精灵附件。
    Sprite(Sprite),
}

/// 节点的公共、用户可编辑的场景数据。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SceneNode {
    /// 人类可读的节点名称。
    pub name: String,
    /// 相对于父节点的局部变换。
    pub transform: Transform,
    /// 此节点及其渲染负载是否应被视为可见。
    pub visible: bool,
    /// 相机剔除层位掩码。
    pub layer: u32,
    /// 节点负载。
    pub kind: NodeKind,
}

impl SceneNode {
    /// 创建一个空节点。
    #[inline]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            transform: Transform::IDENTITY,
            visible: true,
            layer: u32::MAX,
            kind: NodeKind::Empty,
        }
    }

    /// 创建一个空节点。
    #[inline]
    pub fn empty(name: impl Into<String>) -> Self {
        Self::new(name)
    }

    /// 创建一个逻辑分组节点。
    #[inline]
    pub fn group(name: impl Into<String>) -> Self {
        Self::new(name).kind(NodeKind::Group)
    }

    /// 创建一个网格节点。
    #[inline]
    pub fn mesh(name: impl Into<String>, mesh_id: MeshId, material_id: MaterialId) -> Self {
        Self::new(name).kind(NodeKind::Mesh {
            mesh_id,
            material_id,
        })
    }

    /// 创建一个光源节点。
    #[inline]
    pub fn light(name: impl Into<String>, light_id: LightId) -> Self {
        Self::new(name).kind(NodeKind::Light { light_id })
    }

    /// 创建一个相机节点。
    #[inline]
    pub fn camera(name: impl Into<String>, camera_id: CameraId) -> Self {
        Self::new(name).kind(NodeKind::Camera { camera_id })
    }

    /// 创建一个精灵节点。
    #[inline]
    pub fn sprite(name: impl Into<String>, sprite: Sprite) -> Self {
        Self::new(name).kind(NodeKind::Sprite(sprite))
    }

    /// 返回设置了局部变换的节点。
    #[inline]
    pub fn transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    /// 返回设置了可见性的节点。
    #[inline]
    pub const fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// 返回设置了层位掩码的节点。
    #[inline]
    pub const fn layer(mut self, layer: u32) -> Self {
        self.layer = layer;
        self
    }

    /// 返回设置了负载类型的节点。
    #[inline]
    pub fn kind(mut self, kind: NodeKind) -> Self {
        self.kind = kind;
        self
    }
}

impl Default for SceneNode {
    #[inline]
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(feature = "std")]
impl Named for SceneNode {
    #[inline]
    fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }
}
