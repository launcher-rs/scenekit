use alloc::vec::Vec;

use scenekit_core::MeshId;

/// 按相机距离选择的网格多级细节。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LodGroup {
    levels: Vec<(f32, MeshId)>,
}

impl LodGroup {
    /// 创建一个 LOD 组，按最近阈值到最远阈值排序。
    pub fn new(mut levels: Vec<(f32, MeshId)>) -> Self {
        levels.sort_by(|a, b| a.0.total_cmp(&b.0));
        Self { levels }
    }

    /// 创建一个空的 LOD 组。
    #[inline]
    pub const fn empty() -> Self {
        Self { levels: Vec::new() }
    }

    /// 返回已排序的级别列表。
    #[inline]
    pub fn levels(&self) -> &[(f32, MeshId)] {
        &self.levels
    }

    /// 添加一个网格级别并保持阈值从近到远排序。
    pub fn add_level(&mut self, max_distance: f32, mesh_id: MeshId) {
        self.levels.push((max_distance, mesh_id));
        self.levels.sort_by(|a, b| a.0.total_cmp(&b.0));
    }

    /// 选择第一个阈值包含 `distance` 的网格。
    pub fn select(&self, distance: f32) -> Option<MeshId> {
        let farthest = self.levels.last().map(|(_, mesh_id)| *mesh_id);
        self.levels
            .iter()
            .find(|(max_distance, _)| distance <= *max_distance)
            .map(|(_, mesh_id)| *mesh_id)
            .or(farthest)
    }
}
