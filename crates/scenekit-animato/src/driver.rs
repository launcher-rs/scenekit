use alloc::vec::Vec;

use scenekit_core::ValidationError;
use scenekit_scene::SceneGraph;

use crate::{
    CameraAnimator, CameraStoreMut, LightAnimator, LightStoreMut, MaterialAnimator,
    MorphWeightAnimator, MorphWeightStoreMut, NodeAnimator, PbrMaterialStoreMut, SkeletonPose,
    SkinnedMeshAnimator,
};

/// 每次 tick 的动画驱动器计数器。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DriverStats {
    /// 本次 tick 更新的节点动画器数量。
    pub node_animators: usize,
    /// 本次 tick 更新的相机动画器数量。
    pub camera_animators: usize,
    /// 本次 tick 更新的材质动画器数量。
    pub material_animators: usize,
    /// 本次 tick 更新的骨骼动画器数量。
    pub skeleton_animators: usize,
    /// 本次 tick 更新的灯光动画器数量（v1.4.0）。
    pub light_animators: usize,
    /// 本次 tick 更新的变形权重动画器数量（v1.4.0）。
    pub morph_animators: usize,
    /// 本次 tick 之后清理的已完成动画器数量。
    pub completed: usize,
}

/// 确定性的场景/相机/材质/灯光/变形/骨骼动画驱动器。
///
/// 这是用于一次性 Animato 补间/弹簧轨道的**过程式**驱动器。
/// 基于片段的回放（循环、交叉淡入淡出、标记、混合）请使用
/// [`crate::mixer::AnimationMixer`]。
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScenixAnimationDriver {
    node_animators: Vec<NodeAnimator>,
    camera_animators: Vec<CameraAnimator>,
    material_animators: Vec<MaterialAnimator>,
    skeleton_animators: Vec<SkinnedMeshAnimator>,
    light_animators: Vec<LightAnimator>,
    morph_animators: Vec<MorphWeightAnimator>,
    paused: bool,
}

impl ScenixAnimationDriver {
    /// 创建空的驱动器。
    #[inline]
    pub const fn new() -> Self {
        Self {
            node_animators: Vec::new(),
            camera_animators: Vec::new(),
            material_animators: Vec::new(),
            skeleton_animators: Vec::new(),
            light_animators: Vec::new(),
            morph_animators: Vec::new(),
            paused: false,
        }
    }

    /// 添加节点动画器并返回其索引。
    pub fn add_node(&mut self, animator: NodeAnimator) -> usize {
        self.node_animators.push(animator);
        self.node_animators.len() - 1
    }

    /// 添加相机动画器并返回其索引。
    pub fn add_camera(&mut self, animator: CameraAnimator) -> usize {
        self.camera_animators.push(animator);
        self.camera_animators.len() - 1
    }

    /// 添加材质动画器并返回其索引。
    pub fn add_material(&mut self, animator: MaterialAnimator) -> usize {
        self.material_animators.push(animator);
        self.material_animators.len() - 1
    }

    /// 添加骨骼动画器并返回其索引。
    pub fn add_skeleton(&mut self, animator: SkinnedMeshAnimator) -> usize {
        self.skeleton_animators.push(animator);
        self.skeleton_animators.len() - 1
    }

    /// 添加灯光动画器并返回其索引（v1.4.0）。
    pub fn add_light(&mut self, animator: LightAnimator) -> usize {
        self.light_animators.push(animator);
        self.light_animators.len() - 1
    }

    /// 添加变形权重动画器并返回其索引（v1.4.0）。
    pub fn add_morph(&mut self, animator: MorphWeightAnimator) -> usize {
        self.morph_animators.push(animator);
        self.morph_animators.len() - 1
    }

    /// 按索引移除节点动画器。
    pub fn remove_node(&mut self, index: usize) -> Option<NodeAnimator> {
        remove_stable(&mut self.node_animators, index)
    }

    /// 按索引移除相机动画器。
    pub fn remove_camera(&mut self, index: usize) -> Option<CameraAnimator> {
        remove_stable(&mut self.camera_animators, index)
    }

    /// 按索引移除材质动画器。
    pub fn remove_material(&mut self, index: usize) -> Option<MaterialAnimator> {
        remove_stable(&mut self.material_animators, index)
    }

    /// 按索引移除骨骼动画器。
    pub fn remove_skeleton(&mut self, index: usize) -> Option<SkinnedMeshAnimator> {
        remove_stable(&mut self.skeleton_animators, index)
    }

    /// 按索引移除灯光动画器（v1.4.0）。
    pub fn remove_light(&mut self, index: usize) -> Option<LightAnimator> {
        remove_stable(&mut self.light_animators, index)
    }

    /// 按索引移除变形权重动画器（v1.4.0）。
    pub fn remove_morph(&mut self, index: usize) -> Option<MorphWeightAnimator> {
        remove_stable(&mut self.morph_animators, index)
    }

    /// 清除所有动画器。
    pub fn clear(&mut self) {
        self.node_animators.clear();
        self.camera_animators.clear();
        self.material_animators.clear();
        self.skeleton_animators.clear();
        self.light_animators.clear();
        self.morph_animators.clear();
    }

    /// 暂停驱动器。
    #[inline]
    pub const fn pause(&mut self) {
        self.paused = true;
    }

    /// 恢复驱动器。
    #[inline]
    pub const fn resume(&mut self) {
        self.paused = false;
    }

    /// 返回驱动器是否已暂停。
    #[inline]
    pub const fn is_paused(&self) -> bool {
        self.paused
    }

    /// 返回是否没有注册任何动画器。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.node_animators.is_empty()
            && self.camera_animators.is_empty()
            && self.material_animators.is_empty()
            && self.skeleton_animators.is_empty()
            && self.light_animators.is_empty()
            && self.morph_animators.is_empty()
    }

    /// 返回已注册的动画器总数。
    #[inline]
    pub fn len(&self) -> usize {
        self.node_animators.len()
            + self.camera_animators.len()
            + self.material_animators.len()
            + self.skeleton_animators.len()
            + self.light_animators.len()
            + self.morph_animators.len()
    }

    /// 返回已注册的节点动画器数量。
    #[inline]
    pub fn node_len(&self) -> usize {
        self.node_animators.len()
    }

    /// 返回已注册的相机动画器数量。
    #[inline]
    pub fn camera_len(&self) -> usize {
        self.camera_animators.len()
    }

    /// 返回已注册的材质动画器数量。
    #[inline]
    pub fn material_len(&self) -> usize {
        self.material_animators.len()
    }

    /// 返回已注册的骨骼动画器数量。
    #[inline]
    pub fn skeleton_len(&self) -> usize {
        self.skeleton_animators.len()
    }

    /// 返回已注册的灯光动画器数量（v1.4.0）。
    #[inline]
    pub fn light_len(&self) -> usize {
        self.light_animators.len()
    }

    /// 返回已注册的变形权重动画器数量（v1.4.0）。
    #[inline]
    pub fn morph_len(&self) -> usize {
        self.morph_animators.len()
    }

    /// 按确定性的插入顺序推进所有动画器。
    ///
    /// v1.4.0 添加了 `lights` 和 `morphs` 存储参数，用于新的灯光和
    /// 变形权重动画器系列。未使用时传入空存储。
    #[allow(clippy::too_many_arguments)]
    pub fn tick(
        &mut self,
        dt: f32,
        scene: &mut SceneGraph,
        cameras: &mut impl CameraStoreMut,
        materials: &mut impl PbrMaterialStoreMut,
        lights: &mut impl LightStoreMut,
        morphs: &mut impl MorphWeightStoreMut,
        skeletons: &mut [SkeletonPose],
    ) -> Result<DriverStats, ValidationError> {
        let stats = DriverStats {
            node_animators: self.node_animators.len(),
            camera_animators: self.camera_animators.len(),
            material_animators: self.material_animators.len(),
            skeleton_animators: self.skeleton_animators.len(),
            light_animators: self.light_animators.len(),
            morph_animators: self.morph_animators.len(),
            completed: 0,
        };

        if self.paused {
            return Ok(stats);
        }

        let mut completed = 0;
        prune_completed(&mut self.node_animators, &mut completed, |animator| {
            animator.update(dt, scene)
        })?;
        prune_completed(&mut self.camera_animators, &mut completed, |animator| {
            animator.update(dt, cameras)
        })?;
        prune_completed(&mut self.material_animators, &mut completed, |animator| {
            animator.update(dt, materials)
        })?;
        prune_completed(&mut self.light_animators, &mut completed, |animator| {
            animator.update(dt, lights)
        })?;
        prune_completed(&mut self.morph_animators, &mut completed, |animator| {
            animator.update(dt, morphs)
        })?;
        prune_completed(&mut self.skeleton_animators, &mut completed, |animator| {
            animator.update(dt, skeletons)
        })?;

        Ok(DriverStats { completed, ..stats })
    }
}

fn remove_stable<T>(items: &mut Vec<T>, index: usize) -> Option<T> {
    if index < items.len() {
        Some(items.remove(index))
    } else {
        None
    }
}

fn prune_completed<T>(
    items: &mut Vec<T>,
    completed: &mut usize,
    mut update: impl FnMut(&mut T) -> Result<bool, ValidationError>,
) -> Result<(), ValidationError> {
    let mut error = None;
    items.retain_mut(|item| {
        if error.is_some() {
            return true;
        }
        match update(item) {
            Ok(true) => {
                *completed += 1;
                false
            }
            Ok(false) => true,
            Err(err) => {
                error = Some(err);
                true
            }
        }
    });
    if let Some(err) = error {
        Err(err)
    } else {
        Ok(())
    }
}
