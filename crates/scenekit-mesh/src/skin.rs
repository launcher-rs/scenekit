//! 蒙皮数据模型 + CPU 蒙皮回退。
//!
//! CPU 蒙皮使无头/无 GPU 测试能够验证姿态。渲染器的 GPU 蒙皮路径
//! 位于 `scenekit-renderer::skinning`。

use alloc::vec::Vec;

use scenekit_math::{Mat4, Vec3};

use crate::Geometry;

/// 每顶点蒙皮属性（glTF `JOINTS_0` + `WEIGHTS_0`）。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkinningAttributes {
    /// 关节索引，每个顶点四个。
    pub joints: Vec<[u16; 4]>,
    /// 关节权重，每个顶点四个（调用者应归一化为总和 1）。
    pub weights: Vec<[f32; 4]>,
}

impl SkinningAttributes {
    /// 返回属性数组是否与 `vertex_count` 匹配。
    #[inline]
    pub fn matches(&self, vertex_count: usize) -> bool {
        self.joints.len() == vertex_count && self.weights.len() == vertex_count
    }
}

/// 完整蒙皮数据：每顶点属性 + 逆绑定矩阵。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkinningData {
    /// 每顶点关节/权重属性。
    pub attributes: SkinningAttributes,
    /// 逆绑定矩阵，每个关节一个。
    pub inverse_bind_matrices: Vec<Mat4>,
}

/// 网格的实时变形权重存储（每个变形目标一个权重）。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MorphWeights {
    /// 按目标栈顺序排列的权重。
    pub weights: Vec<f32>,
}

impl MorphWeights {
    /// 创建 `len` 个目标的零初始化权重栈。
    pub fn zero(len: usize) -> Self {
        Self {
            weights: alloc::vec![0.0; len],
        }
    }

    /// 设置 `index` 处的权重，限制在 `[0, 1]` 范围内。
    #[inline]
    pub fn set(&mut self, index: usize, weight: f32) {
        if let Some(w) = self.weights.get_mut(index) {
            *w = weight.clamp(0.0, 1.0);
        }
    }

    /// 返回 `index` 处的权重，若超出范围则返回 `0.0`。
    #[inline]
    pub fn get(&self, index: usize) -> f32 {
        self.weights.get(index).copied().unwrap_or(0.0)
    }

    /// 返回权重数量。
    #[inline]
    pub fn len(&self) -> usize {
        self.weights.len()
    }

    /// 返回是否没有权重。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.weights.is_empty()
    }
}

/// 从骨骼世界变换和逆绑定矩阵计算最终每关节矩阵。
///
/// `bone_world` 是各关节的拼接世界变换
///（由调用者从 `SkeletonPose` + 层级结构计算）。缺失的逆绑定矩阵
/// 默认为单位矩阵。
pub fn final_joint_matrices(bone_world: &[Mat4], inverse_bind: &[Mat4]) -> Vec<Mat4> {
    bone_world
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let ibm = inverse_bind.get(i).copied().unwrap_or(Mat4::IDENTITY);
            m.mul_mat4(ibm)
        })
        .collect()
}

/// 使用给定的最终关节矩阵对 `geometry` 进行 CPU 蒙皮。
///
/// 返回变形后的 `Geometry`。法线作为方向进行变形
///（忽略齐次除法）。若属性/矩阵数量不匹配，
/// 则返回未修改的输入几何体。
pub fn cpu_skin(geometry: &Geometry, skin: &SkinningAttributes, final_mats: &[Mat4]) -> Geometry {
    let positions = &geometry.positions;
    let normals = &geometry.normals;
    if !skin.matches(positions.len()) || final_mats.is_empty() {
        return geometry.clone();
    }
    let mut out = geometry.clone();
    for i in 0..positions.len() {
        let [j0, j1, j2, j3] = skin.joints[i];
        let [w0, w1, w2, w3] = skin.weights[i];
        let mut p = Vec3::ZERO;
        let mut n = Vec3::ZERO;
        for (j, w) in [(j0, w0), (j1, w1), (j2, w2), (j3, w3)] {
            if w <= 0.0 {
                continue;
            }
            let m = final_mats
                .get(j as usize)
                .copied()
                .unwrap_or(Mat4::IDENTITY);
            // 通过 mul_vec4 进行点变换（w=1）和方向变换（w=0）。
            let p4 = m.mul_vec4(scenekit_math::Vec4::new(
                positions[i].x,
                positions[i].y,
                positions[i].z,
                1.0,
            ));
            p += Vec3::new(p4.x, p4.y, p4.z) * w;
            let n4 = m.mul_vec4(scenekit_math::Vec4::new(
                normals[i].x,
                normals[i].y,
                normals[i].z,
                0.0,
            ));
            n += Vec3::new(n4.x, n4.y, n4.z) * w;
        }
        out.positions[i] = p;
        out.normals[i] = n.normalize();
    }
    out
}

/// 将变形目标增量按 `weights` 应用到 `geometry` 的克隆上。
///
/// 目前 v1.4 仅应用位置增量；法线增量可通过扩展此函数后续添加。
pub fn apply_morph(
    geometry: &Geometry,
    targets: &[crate::MorphTarget],
    weights: &[f32],
) -> Geometry {
    if targets.is_empty() || weights.is_empty() {
        return geometry.clone();
    }
    let mut out = geometry.clone();
    for (ti, target) in targets.iter().enumerate() {
        let w = weights.get(ti).copied().unwrap_or(0.0);
        if w == 0.0 {
            continue;
        }
        for (i, delta) in target.positions_delta.iter().enumerate() {
            if let Some(p) = out.positions.get_mut(i) {
                *p += *delta * w;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use scenekit_math::Vec4;

    #[test]
    fn final_joint_matrices_combine_world_and_inverse_bind() {
        let world = Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0));
        let ibm = Mat4::from_translation(Vec3::new(-1.0, 0.0, 0.0));
        let mats = final_joint_matrices(&[world], &[ibm]);
        // world * ibm == 恒等平移。
        let p = mats[0].mul_vec4(Vec4::new(5.0, 5.0, 5.0, 1.0));
        assert!((p.x - 5.0).abs() < 1e-4);
    }

    #[test]
    fn morph_weights_set_clamps() {
        let mut w = MorphWeights::zero(2);
        w.set(0, 1.5);
        assert_eq!(w.get(0), 1.0);
        w.set(1, -0.5);
        assert_eq!(w.get(1), 0.0);
    }
}
