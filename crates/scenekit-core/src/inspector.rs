use alloc::{string::String, vec::Vec};

use scenekit_math::{Vec2, Vec3, Vec4};

use crate::Color;

/// 用于将检查器行与应用数据关联的不透明标识符。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InspectorId(pub u64);

/// 类型化的检查器字段值。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InspectorValue {
    /// 布尔值。
    Bool(bool),
    /// 有符号整数值。
    Integer(i64),
    /// 无符号整数值。
    Unsigned(u64),
    /// 浮点值。
    Number(f64),
    /// 人类可读的文本或枚举标签。
    Text(String),
    /// 二维向量。
    Vec2(Vec2),
    /// 三维向量。
    Vec3(Vec3),
    /// 四维向量。
    Vec4(Vec4),
    /// 线性颜色。
    Color(Color),
    /// 使用资源感知格式渲染的字节数。
    Bytes(u64),
}

impl From<bool> for InspectorValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl From<f32> for InspectorValue {
    fn from(value: f32) -> Self {
        Self::Number(value as f64)
    }
}
impl From<f64> for InspectorValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl From<u64> for InspectorValue {
    fn from(value: u64) -> Self {
        Self::Unsigned(value)
    }
}
impl From<usize> for InspectorValue {
    fn from(value: usize) -> Self {
        Self::Unsigned(value as u64)
    }
}
impl From<Vec2> for InspectorValue {
    fn from(value: Vec2) -> Self {
        Self::Vec2(value)
    }
}
impl From<Vec3> for InspectorValue {
    fn from(value: Vec3) -> Self {
        Self::Vec3(value)
    }
}
impl From<Vec4> for InspectorValue {
    fn from(value: Vec4) -> Self {
        Self::Vec4(value)
    }
}
impl From<Color> for InspectorValue {
    fn from(value: Color) -> Self {
        Self::Color(value)
    }
}
impl From<String> for InspectorValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
impl From<&str> for InspectorValue {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}

/// 检查器项目中的一个命名属性。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InspectorField {
    /// 字段标签。
    pub name: String,
    /// 类型化的字段值。
    pub value: InspectorValue,
}

impl InspectorField {
    /// 创建命名字段。
    pub fn new(name: impl Into<String>, value: impl Into<InspectorValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// 带有字段和子项的层次化检查器行。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InspectorItem {
    /// 稳定的快照局部项标识符。
    pub id: InspectorId,
    /// 显示标签。
    pub label: String,
    /// 简短的类型/类别标签。
    pub kind: String,
    /// 只读的类型化字段。
    pub fields: Vec<InspectorField>,
    /// 嵌套项。
    pub children: Vec<InspectorItem>,
}

impl InspectorItem {
    /// 创建空的检查器项。
    pub fn new(id: InspectorId, label: impl Into<String>, kind: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            kind: kind.into(),
            fields: Vec::new(),
            children: Vec::new(),
        }
    }

    /// 添加类型化字段。
    pub fn field(mut self, name: impl Into<String>, value: impl Into<InspectorValue>) -> Self {
        self.fields.push(InspectorField::new(name, value));
        self
    }

    /// 添加子项。
    pub fn child(mut self, child: InspectorItem) -> Self {
        self.children.push(child);
        self
    }
}

/// 拥有所有权的、可序列化的检查器树。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InspectorSnapshot {
    /// 快照根节点。
    pub roots: Vec<InspectorItem>,
}

impl InspectorSnapshot {
    /// 创建空快照。
    pub const fn new() -> Self {
        Self { roots: Vec::new() }
    }

    /// 清除内容但保留已分配的容量。
    pub fn clear(&mut self) {
        self.roots.clear();
    }

    /// 添加根项。
    pub fn push(&mut self, item: InspectorItem) {
        self.roots.push(item);
    }
}

/// 将值的结构化视图追加到检查器快照。
pub trait Inspectable {
    /// 追加一个或多个根节点，不清除现有内容。
    fn inspect(&self, snapshot: &mut InspectorSnapshot);

    /// 构建新的拥有所有权的快照。
    fn inspector_snapshot(&self) -> InspectorSnapshot {
        let mut snapshot = InspectorSnapshot::new();
        self.inspect(&mut snapshot);
        snapshot
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inspector_tree_builders_preserve_typed_values() {
        let item = InspectorItem::new(InspectorId(1), "Camera", "perspective")
            .field("fov", 60.0_f32)
            .child(InspectorItem::new(InspectorId(2), "Transform", "transform"));
        assert_eq!(item.children.len(), 1);
        assert_eq!(item.fields[0].value, InspectorValue::Number(60.0));
    }
}
