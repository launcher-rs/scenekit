//! GPU 蒙皮 + 变形上传钩子（作为渲染器注册表的附加功能）。
//!
//! GPU 资源仍归渲染器所有。这些钩子持有每个网格的关节矩阵和变形权重缓冲区，
//! 供蒙皮 WGSL 路径使用。CPU 回退方案（`scenekit_mesh::skin::cpu_skin`）
//! 始终可用于无头/无 GPU 测试。
//!
//! v1.4.0 发布了注册表 + 上传钩子和 `SKINNING_WGSL` 代码片段。
//! 完整的着色器管线连接通过 `SKINNING` 着色器定义重用现有 `PipelineCache` 键扩展。

use std::collections::BTreeMap;

use scenekit_core::MeshId;
use scenekit_math::Mat4;

/// 渲染器拥有的每网格 GPU 蒙皮状态。
#[derive(Default)]
pub struct GpuSkinningRegistry {
    joints: BTreeMap<MeshId, Vec<Mat4>>,
    morph_weights: BTreeMap<MeshId, Vec<f32>>,
}

impl GpuSkinningRegistry {
    /// 创建空的注册表。
    pub const fn new() -> Self {
        Self {
            joints: BTreeMap::new(),
            morph_weights: BTreeMap::new(),
        }
    }

    /// 注册（或替换）`mesh_id` 的骨骼矩阵缓冲区。
    pub fn register_skin(&mut self, mesh_id: MeshId, bones: Vec<Mat4>) {
        self.joints.insert(mesh_id, bones);
    }

    /// 更新 `mesh_id` 的骨骼矩阵缓冲区。如果网格没有已注册的蒙皮则返回 `false`。
    pub fn update_bone_matrices(&mut self, mesh_id: MeshId, bones: &[Mat4]) -> bool {
        if let Some(slot) = self.joints.get_mut(&mesh_id) {
            slot.clear();
            slot.extend_from_slice(bones);
            true
        } else {
            false
        }
    }

    /// 注销 `mesh_id` 的蒙皮。
    pub fn unregister_skin(&mut self, mesh_id: MeshId) -> bool {
        self.joints.remove(&mesh_id).is_some()
    }

    /// 注册（或替换）`mesh_id` 的变形权重缓冲区。
    pub fn register_morph_targets(&mut self, mesh_id: MeshId, weights: Vec<f32>) {
        self.morph_weights.insert(mesh_id, weights);
    }

    /// 更新 `mesh_id` 的变形权重缓冲区。
    pub fn update_morph_weights(&mut self, mesh_id: MeshId, weights: &[f32]) -> bool {
        if let Some(slot) = self.morph_weights.get_mut(&mesh_id) {
            slot.clear();
            slot.extend_from_slice(weights);
            true
        } else {
            false
        }
    }

    /// 注销 `mesh_id` 的变形权重。
    pub fn unregister_morph(&mut self, mesh_id: MeshId) -> bool {
        self.morph_weights.remove(&mesh_id).is_some()
    }

    /// 返回 `mesh_id` 的骨骼矩阵切片（如果已注册）。
    #[inline]
    pub fn bones(&self, mesh_id: MeshId) -> Option<&[Mat4]> {
        self.joints.get(&mesh_id).map(|v| v.as_slice())
    }

    /// 返回 `mesh_id` 的变形权重切片（如果已注册）。
    #[inline]
    pub fn morph_weights(&self, mesh_id: MeshId) -> Option<&[f32]> {
        self.morph_weights.get(&mesh_id).map(|v| v.as_slice())
    }

    /// 返回 `mesh_id` 是否有已注册的蒙皮。
    #[inline]
    pub fn has_skin(&self, mesh_id: MeshId) -> bool {
        self.joints.contains_key(&mesh_id)
    }

    /// 返回 `mesh_id` 是否有已注册的变形权重。
    #[inline]
    pub fn has_morph(&self, mesh_id: MeshId) -> bool {
        self.morph_weights.contains_key(&mesh_id)
    }

    /// 返回已注册的蒙皮数量。
    #[inline]
    pub fn skin_count(&self) -> usize {
        self.joints.len()
    }

    /// 返回已注册的变形权重堆栈数量。
    #[inline]
    pub fn morph_count(&self) -> usize {
        self.morph_weights.len()
    }
}

/// 追加到蒙皮顶点着色器的 WGSL 代码片段。当网格有已注册的蒙皮时，
/// 渲染器通过 `SKINNING` 着色器定义嵌入此代码片段。
pub const SKINNING_WGSL: &str = r#"// Scenix GPU 蒙皮代码片段（v1.4.0）。
// 在 group 1, binding 0 处绑定关节 mat4x4<f32> 的存储缓冲区。
struct Joint { matrix: mat4x4<f32> };
@group(1) @binding(0) var<storage, read> joints: array<Joint>;

// skin_vertex 通过四个加权关节矩阵变换位置。
fn skin_position(in_pos: vec3<f32>, joint_indices: vec4<u32>, weights: vec4<f32>) -> vec4<f32> {
    let m = joints[joint_indices.x].matrix * weights.x
          + joints[joint_indices.y].matrix * weights.y
          + joints[joint_indices.z].matrix * weights.z
          + joints[joint_indices.w].matrix * weights.w;
    return m * vec4<f32>(in_pos, 1.0);
}

// skin_normal 将法线作为方向（w = 0）进行变换。
fn skin_normal(in_normal: vec3<f32>, joint_indices: vec4<u32>, weights: vec4<f32>) -> vec3<f32> {
    let m = joints[joint_indices.x].matrix * weights.x
          + joints[joint_indices.y].matrix * weights.y
          + joints[joint_indices.z].matrix * weights.z
          + joints[joint_indices.w].matrix * weights.w;
    let n = m * vec4<f32>(in_normal, 0.0);
    return normalize(n.xyz);
}
"#;
