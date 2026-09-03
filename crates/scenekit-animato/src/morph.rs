//! 变形目标权重动画。

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use scenekit_core::{MeshId, ValidationError};

use crate::ScalarTrack;

/// 变形动画器和混合器使用的可变变形权重查找。
pub trait MorphWeightStoreMut {
    /// 返回 `mesh_id` 的可变权重切片（如果存在）。
    fn morph_weights_mut(&mut self, mesh_id: MeshId) -> Option<&mut [f32]>;
}

impl MorphWeightStoreMut for BTreeMap<MeshId, Vec<f32>> {
    #[inline]
    fn morph_weights_mut(&mut self, mesh_id: MeshId) -> Option<&mut [f32]> {
        self.get_mut(&mesh_id).map(|v| v.as_mut_slice())
    }
}

/// 使用标量轨道驱动网格上的单个变形目标权重。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MorphWeightAnimator {
    /// 目标网格。
    pub mesh_id: MeshId,
    /// 网格权重堆栈中的目标变形索引。
    pub target_index: usize,
    /// 标量权重轨道。
    pub track: ScalarTrack,
}

impl MorphWeightAnimator {
    /// 创建变形权重动画器。
    #[inline]
    pub const fn new(mesh_id: MeshId, target_index: usize, track: ScalarTrack) -> Self {
        Self {
            mesh_id,
            target_index,
            track,
        }
    }

    /// 推进、应用并返回完成状态。
    pub fn update(
        &mut self,
        dt: f32,
        morphs: &mut impl MorphWeightStoreMut,
    ) -> Result<bool, ValidationError> {
        self.track.update(dt);
        if let Some(weights) = morphs.morph_weights_mut(self.mesh_id)
            && let Some(w) = weights.get_mut(self.target_index)
        {
            *w = self.track.value().clamp(0.0, 1.0);
        }
        Ok(self.track.is_complete())
    }
}
