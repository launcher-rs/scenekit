use alloc::vec::Vec;
use core::cmp::Ordering;

use scenekit_core::NodeId;
use scenekit_math::{Aabb, Ray3};

/// 场景级 BVH 叶节点条目。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BvhEntry {
    /// 此条目表示的场景节点。
    pub node_id: NodeId,
    /// 世界空间节点包围盒。
    pub aabb: Aabb,
}

impl BvhEntry {
    /// 从节点 ID 和世界空间包围盒创建 BVH 条目。
    #[inline]
    pub const fn new(node_id: NodeId, aabb: Aabb) -> Self {
        Self { node_id, aabb }
    }
}

/// 紧凑的 BVH 节点。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BvhNode {
    /// 此节点的世界空间包围盒。
    pub aabb: Aabb,
    /// 内部节点的左子节点索引。
    pub left: u32,
    /// 内部节点的右子节点索引。
    pub right: u32,
    /// 叶节点的起始条目索引。
    pub start: u32,
    /// 叶节点的条目数量。内部节点 `count == 0`。
    pub count: u32,
}

impl BvhNode {
    /// 返回此节点是否为叶节点。
    #[inline]
    pub const fn is_leaf(self) -> bool {
        self.count > 0
    }
}

/// 基于表面积启发式的场景节点 AABB BVH。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bvh {
    nodes: Vec<BvhNode>,
    entries: Vec<BvhEntry>,
    leaf_size: usize,
}

impl Bvh {
    /// 从世界空间场景条目构建 BVH。
    pub fn build(entries: &[BvhEntry]) -> Self {
        Self::build_with_leaf_size(entries, 4)
    }

    /// 使用自定义最大叶大小构建 BVH。
    pub fn build_with_leaf_size(entries: &[BvhEntry], leaf_size: usize) -> Self {
        let mut bvh = Self {
            nodes: Vec::new(),
            entries: entries.to_vec(),
            leaf_size: leaf_size.max(1),
        };
        if !bvh.entries.is_empty() {
            bvh.build_node(0, bvh.entries.len());
        }
        bvh
    }

    /// 返回可能被 `ray` 命中的所有节点。
    pub fn traverse(&self, ray: Ray3) -> Vec<NodeId> {
        let mut node_ids = Vec::new();
        self.traverse_into(ray, &mut node_ids);
        node_ids
    }

    /// 将所有可能命中的节点 ID 写入可重用向量。
    pub fn traverse_into(&self, ray: Ray3, node_ids: &mut Vec<NodeId>) {
        node_ids.clear();
        self.visit_ray(ray, |node_id| node_ids.push(node_id));
    }

    /// 不分配内存地访问可能命中的节点 ID。
    pub fn visit_ray(&self, ray: Ray3, mut visitor: impl FnMut(NodeId)) {
        if !self.nodes.is_empty() {
            self.visit_ray_node(0, ray, &mut visitor);
        }
    }

    /// 返回 BVH 是否没有条目。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 返回条目数量。
    #[inline]
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// 返回内部节点和叶节点的总数。
    #[inline]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// 访问包围盒匹配层级宽谓词的条目。
    pub fn visit_bounds(
        &self,
        intersects: &impl Fn(Aabb) -> bool,
        mut visitor: impl FnMut(NodeId),
    ) {
        if !self.nodes.is_empty() {
            self.visit_bounds_node(0, intersects, &mut visitor);
        }
    }

    /// 递归访问光线可能命中的节点。
    fn visit_ray_node(&self, index: usize, ray: Ray3, visitor: &mut impl FnMut(NodeId)) {
        let node = self.nodes[index];
        if ray.intersect_aabb(node.aabb).is_none() {
            return;
        }
        if node.is_leaf() {
            let start = node.start as usize;
            let end = start + node.count as usize;
            for entry in &self.entries[start..end] {
                if ray.intersect_aabb(entry.aabb).is_some() {
                    visitor(entry.node_id);
                }
            }
        } else {
            self.visit_ray_node(node.left as usize, ray, visitor);
            self.visit_ray_node(node.right as usize, ray, visitor);
        }
    }

    /// 递归访问包围盒相交的节点。
    fn visit_bounds_node(
        &self,
        index: usize,
        intersects: &impl Fn(Aabb) -> bool,
        visitor: &mut impl FnMut(NodeId),
    ) {
        let node = self.nodes[index];
        if !intersects(node.aabb) {
            return;
        }
        if node.is_leaf() {
            let start = node.start as usize;
            let end = start + node.count as usize;
            for entry in &self.entries[start..end] {
                if intersects(entry.aabb) {
                    visitor(entry.node_id);
                }
            }
        } else {
            self.visit_bounds_node(node.left as usize, intersects, visitor);
            self.visit_bounds_node(node.right as usize, intersects, visitor);
        }
    }

    /// 递归构建 BVH 节点。
    fn build_node(&mut self, start: usize, end: usize) -> usize {
        let node_index = self.nodes.len();
        let aabb = bounds_for(&self.entries[start..end]);
        self.nodes.push(BvhNode {
            aabb,
            left: 0,
            right: 0,
            start: start as u32,
            count: (end - start) as u32,
        });

        let count = end - start;
        if count <= self.leaf_size {
            return node_index;
        }

        let Some(split) = self.find_sah_split(start, end) else {
            return node_index;
        };

        let left = self.build_node(start, split);
        let right = self.build_node(split, end);
        self.nodes[node_index] = BvhNode {
            aabb,
            left: left as u32,
            right: right as u32,
            start: 0,
            count: 0,
        };
        node_index
    }

    /// 查找 SAH 最优分割点。
    fn find_sah_split(&mut self, start: usize, end: usize) -> Option<usize> {
        let count = end - start;
        if count <= 1 {
            return None;
        }

        let centers = center_bounds(&self.entries[start..end]);
        let extent = centers.max - centers.min;
        let axis = if extent.x >= extent.y && extent.x >= extent.z {
            0
        } else if extent.y >= extent.z {
            1
        } else {
            2
        };
        if extent[axis].abs() <= 1.0e-6 {
            return None;
        }

        self.entries[start..end].sort_by(|a, b| {
            let lhs = a.aabb.center()[axis];
            let rhs = b.aabb.center()[axis];
            lhs.total_cmp(&rhs).then_with(|| {
                let lhs_id = a.node_id.get();
                let rhs_id = b.node_id.get();
                if lhs_id < rhs_id {
                    Ordering::Less
                } else if lhs_id > rhs_id {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
        });

        let mut prefix = Vec::with_capacity(count);
        let mut running = self.entries[start].aabb;
        prefix.push(running);
        for entry in &self.entries[start + 1..end] {
            running = running.merge(entry.aabb);
            prefix.push(running);
        }

        let mut suffix = alloc::vec![Aabb::ZERO; count];
        running = self.entries[end - 1].aabb;
        suffix[count - 1] = running;
        for offset in (0..count - 1).rev() {
            running = running.merge(self.entries[start + offset].aabb);
            suffix[offset] = running;
        }

        let mut best_split = count / 2;
        let mut best_cost = f32::INFINITY;
        for split in 1..count {
            let left_count = split as f32;
            let right_count = (count - split) as f32;
            let cost = prefix[split - 1].surface_area() * left_count
                + suffix[split].surface_area() * right_count;
            if cost < best_cost {
                best_cost = cost;
                best_split = split;
            }
        }

        Some(start + best_split)
    }
}

/// 计算条目集合的包围盒。
fn bounds_for(entries: &[BvhEntry]) -> Aabb {
    let Some((first, rest)) = entries.split_first() else {
        return Aabb::ZERO;
    };
    let mut bounds = first.aabb;
    for entry in rest {
        bounds = bounds.merge(entry.aabb);
    }
    bounds
}

/// 计算条目中心点的包围盒。
fn center_bounds(entries: &[BvhEntry]) -> Aabb {
    let Some((first, rest)) = entries.split_first() else {
        return Aabb::ZERO;
    };
    let mut bounds = Aabb::new(first.aabb.center(), first.aabb.center());
    for entry in rest {
        let center = entry.aabb.center();
        bounds = bounds.merge(Aabb::new(center, center));
    }
    bounds
}
