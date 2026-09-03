//! 骨骼重定向：将源骨骼映射到目标骨骼。

use alloc::string::String;
use alloc::vec::Vec;

use crate::skeleton::SkeletonPose;

/// 一条源→目标骨骼映射。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RetargetEntry {
    /// 源骨骼索引。
    pub source: usize,
    /// 目标骨骼索引。
    pub target: usize,
}

/// 用于从一个骨骼复制并调整姿态到另一个骨骼的重定向映射。
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RetargetMap {
    /// 按源顺序排列的映射条目。
    pub entries: Vec<RetargetEntry>,
}

impl RetargetMap {
    /// 通过匹配 `source_names` 和 `target_names` 之间的骨骼名称构建映射。
    pub fn from_names(source_names: &[String], target_names: &[String]) -> Self {
        let mut entries = Vec::new();
        for (s_idx, s_name) in source_names.iter().enumerate() {
            if let Some(t_idx) = target_names.iter().position(|t| t == s_name) {
                entries.push(RetargetEntry {
                    source: s_idx,
                    target: t_idx,
                });
            }
        }
        Self { entries }
    }

    /// 添加显式的源→目标条目。
    #[inline]
    pub fn with_entry(mut self, source: usize, target: usize) -> Self {
        self.entries.push(RetargetEntry { source, target });
        self
    }

    /// 根据映射将 `source` 姿态的骨骼复制到 `target`。
    ///
    /// 未映射的目标骨骼保持其现有变换不变。
    pub fn apply(&self, source: &SkeletonPose, target: &mut SkeletonPose) {
        for entry in &self.entries {
            if let (Some(src), Some(dst)) = (
                source.bones.get(entry.source),
                target.bones.get_mut(entry.target),
            ) {
                dst.clone_from(src);
            }
        }
    }
}
