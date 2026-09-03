use alloc::{string::String, vec::Vec};

use scenekit_core::NodeId;
use scenekit_math::Vec3;

/// 用于过滤场景交互层的位掩码。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LayerMask(u32);

impl LayerMask {
    /// 无图层。
    pub const NONE: Self = Self(0);
    /// 所有图层位。
    pub const ALL: Self = Self(u32::MAX);

    /// 从原始位创建掩码。
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// 返回原始掩码位。
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// 返回是否有任何位重叠。
    #[inline]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    /// 返回此掩码是否包含节点的层掩码。
    #[inline]
    pub const fn matches_node(self, node_layers: u32) -> bool {
        self.0 & node_layers != 0
    }

    /// 向掩码添加位。
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    /// 从掩码移除位。
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }
}

/// 编辑器交互应用的层过滤器。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LayerPolicy {
    /// 可被选中的图层。
    pub selectable: LayerMask,
    /// 可被拖拽的图层。
    pub draggable: LayerMask,
    /// 可被变换工具操作的图层。
    pub transformable: LayerMask,
}

impl Default for LayerPolicy {
    fn default() -> Self {
        Self {
            selectable: LayerMask::ALL,
            draggable: LayerMask::ALL,
            transformable: LayerMask::ALL,
        }
    }
}

/// 与场景节点关联的稀疏编辑器专用元数据。
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NodeEditorMetadata {
    /// 节点可被选中。
    pub selectable: bool,
    /// 节点可被拖拽工具移动。
    pub draggable: bool,
    /// 节点可被变换工具修改。
    pub transformable: bool,
    /// 节点受到保护，防止编辑器修改。
    pub locked: bool,
    /// 节点在检查器快照中可见。
    pub visible_in_inspector: bool,
    /// 可选的编辑器标签，覆盖场景名称。
    pub label: Option<String>,
    /// 应用自定义的编辑器标签。
    pub tags: Vec<String>,
}

impl Default for NodeEditorMetadata {
    fn default() -> Self {
        Self {
            selectable: true,
            draggable: true,
            transformable: true,
            locked: false,
            visible_in_inspector: true,
            label: None,
            tags: Vec::new(),
        }
    }
}

/// 选择命令如何与现有选择组合。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SelectionMode {
    /// 替换整个选择。
    #[default]
    Replace,
    /// 若不存在则添加节点。
    Add,
    /// 切换成员关系。
    Toggle,
    /// 若存在则移除节点。
    Remove,
}

/// 当前图内编辑器选择状态。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectionState {
    /// 指针下的节点（若有）。
    pub hovered: Option<NodeId>,
    /// 接收主动交互的节点（若有）。
    pub active: Option<NodeId>,
    selected: Vec<NodeId>,
}

impl SelectionState {
    /// 按升序确定性顺序排列的已选 ID。
    #[inline]
    pub fn selected(&self) -> &[NodeId] {
        &self.selected
    }

    /// 返回节点是否已被选中。
    #[inline]
    pub fn contains(&self, id: NodeId) -> bool {
        self.selected.binary_search(&id).is_ok()
    }
}

/// 一次操作产生的精确选择差异。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectionChange {
    /// 操作添加的 ID。
    pub added: Vec<NodeId>,
    /// 操作移除的 ID。
    pub removed: Vec<NodeId>,
}

/// 活动变换工具。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransformMode {
    /// 移动节点。
    #[default]
    Translate,
    /// 旋转节点。
    Rotate,
    /// 缩放节点。
    Scale,
}

/// 变换工具使用的坐标系。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransformSpace {
    /// 世界坐标轴。
    #[default]
    World,
    /// 节点局部坐标轴。
    Local,
}

/// 变换操作使用的轴或平面约束。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransformConstraint {
    /// 无轴约束。
    #[default]
    Free,
    /// X 轴。
    X,
    /// Y 轴。
    Y,
    /// Z 轴。
    Z,
    /// XY 平面。
    XY,
    /// XZ 平面。
    XZ,
    /// YZ 平面。
    YZ,
}

impl TransformConstraint {
    /// 返回用于平移和缩放约束的分量掩码。
    pub const fn component_mask(self) -> Vec3 {
        match self {
            Self::Free => Vec3::ONE,
            Self::X => Vec3::X,
            Self::Y => Vec3::Y,
            Self::Z => Vec3::Z,
            Self::XY => Vec3::new(1.0, 1.0, 0.0),
            Self::XZ => Vec3::new(1.0, 0.0, 1.0),
            Self::YZ => Vec3::new(0.0, 1.0, 1.0),
        }
    }

    /// 返回受约束的单一轴（若适用）。
    pub const fn axis(self) -> Option<Vec3> {
        match self {
            Self::X => Some(Vec3::X),
            Self::Y => Some(Vec3::Y),
            Self::Z => Some(Vec3::Z),
            _ => None,
        }
    }
}

/// 编辑器变换的量化增量。设为零可禁用对应分量。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SnapSettings {
    /// 每轴平移增量。
    pub translation: Vec3,
    /// 旋转增量（弧度）。
    pub rotation_radians: f32,
    /// 每轴缩放增量。
    pub scale: Vec3,
}

impl SnapSettings {
    /// 相对于操作起点量化平移。
    pub fn snap_translation(self, delta: Vec3) -> Vec3 {
        snap_vec3(delta, self.translation)
    }

    /// 相对于操作起点量化旋转角度。
    pub fn snap_rotation(self, radians: f32) -> f32 {
        snap_scalar(radians, self.rotation_radians)
    }

    /// 相对于操作起点量化缩放增量。
    pub fn snap_scale(self, delta: Vec3) -> Vec3 {
        snap_vec3(delta, self.scale)
    }
}

fn snap_scalar(value: f32, increment: f32) -> f32 {
    if increment.is_finite() && increment.abs() > 1.0e-6 {
        let increment = increment.abs();
        let scaled = value / increment;
        if !scaled.is_finite() {
            return value;
        }
        let rounded = if scaled >= 0.0 {
            (scaled + 0.5) as i64
        } else {
            (scaled - 0.5) as i64
        };
        rounded as f32 * increment
    } else {
        value
    }
}

fn snap_vec3(value: Vec3, increments: Vec3) -> Vec3 {
    Vec3::new(
        snap_scalar(value.x, increments.x),
        snap_scalar(value.y, increments.y),
        snap_scalar(value.z, increments.z),
    )
}

pub(crate) fn apply_selection(
    state: &mut SelectionState,
    id: NodeId,
    mode: SelectionMode,
) -> SelectionChange {
    let previous = state.selected.clone();
    match mode {
        SelectionMode::Replace => {
            state.selected.clear();
            state.selected.push(id);
        }
        SelectionMode::Add => {
            if let Err(index) = state.selected.binary_search(&id) {
                state.selected.insert(index, id);
            }
        }
        SelectionMode::Toggle => match state.selected.binary_search(&id) {
            Ok(index) => {
                state.selected.remove(index);
            }
            Err(index) => state.selected.insert(index, id),
        },
        SelectionMode::Remove => {
            if let Ok(index) = state.selected.binary_search(&id) {
                state.selected.remove(index);
            }
        }
    }
    selection_diff(&previous, &state.selected)
}

pub(crate) fn replace_selection(
    state: &mut SelectionState,
    mut selected: Vec<NodeId>,
) -> SelectionChange {
    selected.sort_unstable();
    selected.dedup();
    let previous = core::mem::replace(&mut state.selected, selected);
    selection_diff(&previous, &state.selected)
}

fn selection_diff(previous: &[NodeId], current: &[NodeId]) -> SelectionChange {
    SelectionChange {
        added: current
            .iter()
            .copied()
            .filter(|id| previous.binary_search(id).is_err())
            .collect(),
        removed: previous
            .iter()
            .copied()
            .filter(|id| current.binary_search(id).is_err())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snap_settings_quantize_relative_deltas() {
        let snap = SnapSettings {
            translation: Vec3::new(0.5, 1.0, 0.0),
            rotation_radians: 0.25,
            scale: Vec3::new(0.1, 0.0, 0.5),
        };
        assert_eq!(
            snap.snap_translation(Vec3::new(0.74, 1.6, 0.3)),
            Vec3::new(0.5, 2.0, 0.3)
        );
        assert_eq!(snap.snap_rotation(0.38), 0.5);
    }
}
