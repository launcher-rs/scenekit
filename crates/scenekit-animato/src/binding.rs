//! 基于片段的动画的类型化属性绑定。
//!
//! Scenix 保持类型化 ID 纪律：不使用 Three.js 那样的字符串属性路径
//! （如 `"position.x"`），绑定是 `(id, typed_property)` 对。
//! 门面 `clip_from_loaded` 辅助函数将 glTF 节点索引映射到场景 `NodeId`，
//! 并生成这些绑定。

use scenekit_core::{CameraId, LightId, MaterialId, MeshId, NodeId};

/// 片段可以驱动的节点属性。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NodeProperty {
    /// 局部平移。
    Translation,
    /// 局部旋转。
    Rotation,
    /// 局部缩放。
    Scale,
    /// 可见性标志。
    Visibility,
}

impl NodeProperty {
    /// 解码由 [`Self::as_u8`] 产生的属性判别值。
    #[inline]
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Translation),
            1 => Some(Self::Rotation),
            2 => Some(Self::Scale),
            3 => Some(Self::Visibility),
            _ => None,
        }
    }
    /// 将属性编码为稳定的判别值。
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

/// 片段可以驱动的骨骼属性。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BoneProperty {
    /// 骨骼局部平移。
    Translation,
    /// 骨骼局部旋转。
    Rotation,
    /// 骨骼局部缩放。
    Scale,
}

impl BoneProperty {
    /// 解码属性判别值。
    #[inline]
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Translation),
            1 => Some(Self::Rotation),
            2 => Some(Self::Scale),
            _ => None,
        }
    }
    /// 将属性编码为稳定的判别值。
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

/// 片段可以驱动的 PBR 材质属性。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MaterialProperty {
    /// 基础颜色（反照率）。
    Albedo,
    /// 基础颜色 Alpha 通道。
    Opacity,
    /// 自发光 RGB 颜色。
    Emissive,
    /// 粗糙度系数。
    Roughness,
    /// 金属度系数。
    Metallic,
}

impl MaterialProperty {
    /// 解码属性判别值。
    #[inline]
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Albedo),
            1 => Some(Self::Opacity),
            2 => Some(Self::Emissive),
            3 => Some(Self::Roughness),
            4 => Some(Self::Metallic),
            _ => None,
        }
    }
    /// 将属性编码为稳定的判别值。
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

/// 片段可以驱动的相机属性。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CameraProperty {
    /// 透视垂直视野角，单位为弧度。
    FovY,
    /// 相机位置。
    Position,
    /// 相机观察目标。
    Target,
    /// 相机上方向向量。
    Up,
    /// 正交投影包围盒。
    OrthographicBounds,
}

impl CameraProperty {
    /// 解码属性判别值。
    #[inline]
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::FovY),
            1 => Some(Self::Position),
            2 => Some(Self::Target),
            3 => Some(Self::Up),
            4 => Some(Self::OrthographicBounds),
            _ => None,
        }
    }
    /// 将属性编码为稳定的判别值。
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

/// 片段可以驱动的灯光属性。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LightProperty {
    /// 灯光颜色。
    Color,
    /// 标量强度。
    Intensity,
    /// 最大范围（仅点光/聚光）。
    Range,
    /// 聚光灯外锥半角，单位为弧度（仅聚光）。
    SpotAngle,
}

impl LightProperty {
    /// 解码属性判别值。
    #[inline]
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Color),
            1 => Some(Self::Intensity),
            2 => Some(Self::Range),
            3 => Some(Self::SpotAngle),
            _ => None,
        }
    }
    /// 将属性编码为稳定的判别值。
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

/// 一个动画目标，解析为具体的 scenekit 资源 + 字段。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PropertyBinding {
    /// 场景节点变换或可见性字段。
    Node {
        /// 目标节点。
        node_id: NodeId,
        /// 要动画化的字段。
        property: NodeProperty,
    },
    /// 骨骼局部变换字段。
    Bone {
        /// 混合器骨骼切片中的目标骨骼索引。
        skeleton_index: usize,
        /// 该骨骼内的目标骨骼索引。
        bone_index: usize,
        /// 要动画化的字段。
        property: BoneProperty,
    },
    /// PBR 材质字段。
    Material {
        /// 目标材质。
        material_id: MaterialId,
        /// 要动画化的字段。
        property: MaterialProperty,
    },
    /// 相机字段。
    Camera {
        /// 目标相机。
        camera_id: CameraId,
        /// 要动画化的字段。
        property: CameraProperty,
    },
    /// 灯光字段。
    Light {
        /// 目标灯光。
        light_id: LightId,
        /// 要动画化的字段。
        property: LightProperty,
    },
    /// 网格上的一个变形目标权重。
    MorphWeight {
        /// 目标网格。
        mesh_id: MeshId,
        /// 网格权重堆栈中的目标变形索引。
        target_index: usize,
    },
}

/// 从绑定派生的稳定、可哈希的键，用于混合器中的累加器查找。
/// 两个写入相同键的通道会被混合在一起。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BindingKey {
    /// 节点绑定键。
    Node { id: u64, property: u8 },
    /// 骨骼绑定键。
    Bone {
        skeleton: usize,
        bone: usize,
        property: u8,
    },
    /// 材质绑定键。
    Material { id: u64, property: u8 },
    /// 相机绑定键。
    Camera { id: u64, property: u8 },
    /// 灯光绑定键。
    Light { id: u64, property: u8 },
    /// 变形权重绑定键。
    Morph { id: u64, target: usize },
}

impl PropertyBinding {
    /// 返回此绑定的稳定累加器键。
    #[inline]
    pub fn key(&self) -> BindingKey {
        match *self {
            PropertyBinding::Node { node_id, property } => BindingKey::Node {
                id: node_id.get(),
                property: property.as_u8(),
            },
            PropertyBinding::Bone {
                skeleton_index,
                bone_index,
                property,
            } => BindingKey::Bone {
                skeleton: skeleton_index,
                bone: bone_index,
                property: property.as_u8(),
            },
            PropertyBinding::Material {
                material_id,
                property,
            } => BindingKey::Material {
                id: material_id.get(),
                property: property.as_u8(),
            },
            PropertyBinding::Camera {
                camera_id,
                property,
            } => BindingKey::Camera {
                id: camera_id.get(),
                property: property.as_u8(),
            },
            PropertyBinding::Light { light_id, property } => BindingKey::Light {
                id: light_id.get(),
                property: property.as_u8(),
            },
            PropertyBinding::MorphWeight {
                mesh_id,
                target_index,
            } => BindingKey::Morph {
                id: mesh_id.get(),
                target: target_index,
            },
        }
    }
}
