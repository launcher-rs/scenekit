use alloc::vec::Vec;

use scenekit_core::ValidationError;
use scenekit_math::Transform;

use crate::{QuatTrack, Vec3Track};

/// 以扁平数组形式表示的 CPU 骨骼姿态。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkeletonPose {
    /// 按调用者定义的关节顺序排列的骨骼变换。
    pub bones: Vec<Transform>,
}

impl SkeletonPose {
    /// 从骨骼变换创建姿态。
    #[inline]
    pub const fn new(bones: Vec<Transform>) -> Self {
        Self { bones }
    }

    /// 创建具有 `len` 个骨骼的单位姿态。
    pub fn identity(len: usize) -> Self {
        Self {
            bones: alloc::vec![Transform::IDENTITY; len],
        }
    }

    /// 返回骨骼数量。
    #[inline]
    pub fn len(&self) -> usize {
        self.bones.len()
    }

    /// 返回姿态是否不包含任何骨骼。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bones.is_empty()
    }
}

/// 可被动画化的骨骼变换字段。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BoneAnimationTarget {
    /// 动画化局部平移。
    Translation(Vec3Track),
    /// 动画化局部旋转。
    Rotation(QuatTrack),
    /// 动画化局部缩放。
    Scale(Vec3Track),
}

impl BoneAnimationTarget {
    /// 推进包含的轨道。
    pub fn update(&mut self, dt: f32) -> bool {
        match self {
            Self::Translation(track) | Self::Scale(track) => track.update(dt),
            Self::Rotation(track) => track.update(dt),
        }
    }

    /// 返回包含的轨道是否已完成。
    pub fn is_complete(&self) -> bool {
        match self {
            Self::Translation(track) | Self::Scale(track) => track.is_complete(),
            Self::Rotation(track) => track.is_complete(),
        }
    }
}

/// [`SkeletonPose`] 中单个骨骼的动画。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoneAnimation {
    /// 目标姿态中的骨骼索引。
    pub bone_index: usize,
    /// 被动画化的字段。
    pub target: BoneAnimationTarget,
}

impl BoneAnimation {
    /// 创建骨骼动画。
    #[inline]
    pub const fn new(bone_index: usize, target: BoneAnimationTarget) -> Self {
        Self { bone_index, target }
    }

    /// 推进动画器，应用当前值，并返回完成状态。
    pub fn update(&mut self, dt: f32, pose: &mut SkeletonPose) -> Result<bool, ValidationError> {
        self.target.update(dt);
        let bone = pose
            .bones
            .get_mut(self.bone_index)
            .ok_or(ValidationError::InvalidId)?;
        match &self.target {
            BoneAnimationTarget::Translation(track) => {
                bone.translation = track.value();
            }
            BoneAnimationTarget::Rotation(track) => {
                bone.rotation = track.value();
            }
            BoneAnimationTarget::Scale(track) => {
                bone.scale = track.value();
            }
        }
        Ok(self.target.is_complete())
    }
}

/// 按索引驱动调用者骨骼存储中的一个骨骼姿态。
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkinnedMeshAnimator {
    /// 驱动器骨骼切片中的目标姿态索引。
    pub skeleton_index: usize,
    /// 按确定性顺序更新的骨骼动画。
    pub bones: Vec<BoneAnimation>,
}

impl SkinnedMeshAnimator {
    /// 创建骨骼动画器。
    #[inline]
    pub const fn new(skeleton_index: usize, bones: Vec<BoneAnimation>) -> Self {
        Self {
            skeleton_index,
            bones,
        }
    }

    /// 添加一个骨骼动画。
    #[inline]
    pub fn push(&mut self, animation: BoneAnimation) {
        self.bones.push(animation);
    }

    /// 推进所有骨骼动画，当所有轨道完成时返回完成状态。
    pub fn update(
        &mut self,
        dt: f32,
        skeletons: &mut [SkeletonPose],
    ) -> Result<bool, ValidationError> {
        let pose = skeletons
            .get_mut(self.skeleton_index)
            .ok_or(ValidationError::InvalidId)?;
        let mut all_complete = true;
        for animation in &mut self.bones {
            all_complete &= animation.update(dt, pose)?;
        }
        Ok(all_complete)
    }
}
