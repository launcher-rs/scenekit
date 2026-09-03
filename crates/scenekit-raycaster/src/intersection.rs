use scenekit_core::{MaterialId, MeshId, NodeId};
use scenekit_math::{Vec2, Vec3};

/// 与场景网格节点的世界空间光线交点。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Intersection {
    /// 被光线命中的场景节点。
    pub node_id: NodeId,
    /// 附加到命中节点的网格资源。
    pub mesh_id: MeshId,
    /// 附加到命中节点的材质资源。
    pub material_id: MaterialId,
    /// 沿光线的参数距离。
    pub distance: f32,
    /// 世界空间命中点。
    pub point: Vec3,
    /// 世界空间表面法线。
    pub normal: Vec3,
    /// 插值的主 UV 坐标，不可用时为零。
    pub uv: Vec2,
}
