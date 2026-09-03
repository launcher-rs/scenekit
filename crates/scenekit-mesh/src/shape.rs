use alloc::vec::Vec;

use scenekit_math::Vec2;

/// 具有外轮廓和可选孔洞的 2D 多边形形状。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Shape {
    contours: Vec<Vec<Vec2>>,
}

impl Shape {
    /// 从外轮廓创建形状。
    #[inline]
    pub fn new(exterior: Vec<Vec2>) -> Self {
        Self {
            contours: alloc::vec![exterior],
        }
    }

    /// 从外轮廓和孔洞创建形状。
    #[inline]
    pub fn with_holes(exterior: Vec<Vec2>, holes: Vec<Vec<Vec2>>) -> Self {
        let mut contours = Vec::with_capacity(holes.len() + 1);
        contours.push(exterior);
        contours.extend(holes);
        Self { contours }
    }

    /// 添加一个孔洞轮廓。
    #[inline]
    pub fn add_hole(&mut self, hole: Vec<Vec2>) {
        self.contours.push(hole);
    }

    /// 返回外轮廓（若存在）。
    #[inline]
    pub fn exterior(&self) -> Option<&[Vec2]> {
        self.contours.first().map(Vec::as_slice)
    }

    /// 返回孔洞轮廓。
    #[inline]
    pub fn holes(&self) -> &[Vec<Vec2>] {
        if self.contours.len() <= 1 {
            &[]
        } else {
            &self.contours[1..]
        }
    }

    /// 返回所有轮廓，外轮廓在前。
    #[inline]
    pub fn contours(&self) -> &[Vec<Vec2>] {
        &self.contours
    }

    /// 返回形状的外轮廓点是否少于三个。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.exterior().is_none_or(|points| points.len() < 3)
    }
}
