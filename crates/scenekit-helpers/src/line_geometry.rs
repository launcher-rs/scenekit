use alloc::vec::Vec;

use scenekit_core::{Color, ValidationError};
use scenekit_math::{Aabb, Vec3};

/// 调试辅助工具使用的经过验证的线段列表几何体。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LineGeometry {
    /// 顶点位置。
    pub positions: Vec<Vec3>,
    /// 每个顶点的颜色。为空时，消费者应使用后备颜色。
    pub colors: Vec<Color>,
    /// 可选的线段列表索引。为空时，位置按成对方式消费。
    pub indices: Vec<u32>,
}

impl LineGeometry {
    /// 创建空的线段几何体。
    #[inline]
    pub const fn new() -> Self {
        Self {
            positions: Vec::new(),
            colors: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// 返回顶点数量。
    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.positions.len()
    }

    /// 返回线段数量。
    #[inline]
    pub fn segment_count(&self) -> usize {
        if self.indices.is_empty() {
            self.positions.len() / 2
        } else {
            self.indices.len() / 2
        }
    }

    /// 返回是否没有存储任何位置数据。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    /// 清除所有几何体数据，同时保留已分配的容量。
    #[inline]
    pub fn clear(&mut self) {
        self.positions.clear();
        self.colors.clear();
        self.indices.clear();
    }

    /// 为额外的顶点和可选索引预留存储空间。
    pub fn reserve(&mut self, vertices: usize, indices: usize) {
        self.positions.reserve(vertices);
        self.colors.reserve(vertices);
        self.indices.reserve(indices);
    }

    /// 添加一条两端使用相同颜色的线段。
    #[inline]
    pub fn push_segment(&mut self, start: Vec3, end: Vec3, color: Color) {
        self.push_colored_segment(start, end, color, color);
    }

    /// 添加一条两端颜色独立的线段。
    pub fn push_colored_segment(
        &mut self,
        start: Vec3,
        end: Vec3,
        start_color: Color,
        end_color: Color,
    ) {
        self.positions.push(start);
        self.positions.push(end);
        self.colors.push(start_color);
        self.colors.push(end_color);
    }

    /// 追加另一个线段几何体，并在存在索引时偏移索引值。
    pub fn merge(&mut self, other: &Self) {
        let base = self.positions.len();
        let incoming = other.positions.len();
        let use_indices = !self.indices.is_empty() || !other.indices.is_empty();

        if use_indices && self.indices.is_empty() {
            self.indices.reserve(base);
            for index in 0..base {
                self.indices.push(index as u32);
            }
        }

        if self.colors.is_empty() && !other.colors.is_empty() {
            self.colors.resize(base, Color::WHITE);
        }
        if !self.colors.is_empty() {
            if other.colors.is_empty() {
                self.colors
                    .extend(core::iter::repeat_n(Color::WHITE, incoming));
            } else {
                self.colors.extend_from_slice(&other.colors);
            }
        }

        self.positions.extend_from_slice(&other.positions);

        if use_indices {
            if other.indices.is_empty() {
                for index in 0..incoming {
                    self.indices.push((base + index) as u32);
                }
            } else {
                for index in &other.indices {
                    self.indices.push(index + base as u32);
                }
            }
        }
    }

    /// 验证颜色长度、线段奇偶性和索引范围。
    pub fn validate(&self) -> Result<(), ValidationError> {
        let vertices = self.positions.len();
        if !self.colors.is_empty() && self.colors.len() != vertices {
            return Err(ValidationError::InvalidState);
        }
        if self.indices.is_empty() {
            if !vertices.is_multiple_of(2) {
                return Err(ValidationError::InvalidState);
            }
        } else {
            if !self.indices.len().is_multiple_of(2) {
                return Err(ValidationError::InvalidState);
            }
            for index in &self.indices {
                if *index as usize >= vertices {
                    return Err(ValidationError::OutOfRange);
                }
            }
        }
        Ok(())
    }

    /// 返回线段位置的包围盒。
    #[inline]
    pub fn aabb(&self) -> Aabb {
        Aabb::from_points(&self.positions)
    }
}
