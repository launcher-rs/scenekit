use scenekit_core::{NodeId, ValidationError};
use scenekit_scene::SceneGraph;

use crate::{BoolTrack, QuatTrack, Vec3Track};

/// 可被动画轨道驱动的场景节点字段。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NodeAnimationTarget {
    /// 动画化局部平移。
    Translation(Vec3Track),
    /// 动画化局部旋转。
    Rotation(QuatTrack),
    /// 动画化局部缩放。
    Scale(Vec3Track),
    /// 动画化可见性。
    Visibility(BoolTrack),
}

impl NodeAnimationTarget {
    /// 推进包含的轨道并返回是否仍在运行。
    pub fn update(&mut self, dt: f32) -> bool {
        match self {
            Self::Translation(track) | Self::Scale(track) => track.update(dt),
            Self::Rotation(track) => track.update(dt),
            Self::Visibility(track) => track.update(dt),
        }
    }

    /// 返回包含的轨道是否已完成。
    pub fn is_complete(&self) -> bool {
        match self {
            Self::Translation(track) | Self::Scale(track) => track.is_complete(),
            Self::Rotation(track) => track.is_complete(),
            Self::Visibility(track) => track.is_complete(),
        }
    }
}

/// 将 Animato 支持的轨道应用到单个场景节点。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NodeAnimator {
    /// 目标场景节点。
    pub node_id: NodeId,
    /// 被动画化的字段。
    pub target: NodeAnimationTarget,
}

impl NodeAnimator {
    /// 创建节点动画器。
    #[inline]
    pub const fn new(node_id: NodeId, target: NodeAnimationTarget) -> Self {
        Self { node_id, target }
    }

    /// 推进动画器，应用当前值，并返回完成状态。
    pub fn update(&mut self, dt: f32, scene: &mut SceneGraph) -> Result<bool, ValidationError> {
        self.target.update(dt);
        let node = scene
            .get_mut(self.node_id)
            .ok_or(ValidationError::InvalidId)?;
        match &self.target {
            NodeAnimationTarget::Translation(track) => {
                node.transform.translation = track.value();
            }
            NodeAnimationTarget::Rotation(track) => {
                node.transform.rotation = track.value();
            }
            NodeAnimationTarget::Scale(track) => {
                node.transform.scale = track.value();
            }
            NodeAnimationTarget::Visibility(track) => {
                node.visible = track.value();
            }
        }
        Ok(self.target.is_complete())
    }
}
