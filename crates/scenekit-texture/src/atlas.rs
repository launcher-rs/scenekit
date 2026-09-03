use alloc::string::String;
use alloc::vec::Vec;

use scenekit_core::ValidationError;

/// 纹理图集中的像素空间矩形。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AtlasRect {
    /// 左边界像素。
    pub x: u32,
    /// 上边界像素。
    pub y: u32,
    /// 宽度（像素）。
    pub width: u32,
    /// 高度（像素）。
    pub height: u32,
}

/// 纹理图集中的归一化 UV 矩形。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UvRect {
    /// 左边界 U 坐标。
    pub u0: f32,
    /// 上边界 V 坐标。
    pub v0: f32,
    /// 右边界 U 坐标。
    pub u1: f32,
    /// 下边界 V 坐标。
    pub v1: f32,
}

/// 命名的图集条目。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AtlasEntry {
    /// 条目名称。
    pub name: String,
    /// 像素空间矩形。
    pub rect: AtlasRect,
    /// 归一化 UV 矩形。
    pub uv: UvRect,
}

/// 确定性 shelf 打包纹理图集。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextureAtlas {
    /// 图集宽度（像素）。
    pub width: u32,
    /// 图集高度（像素）。
    pub height: u32,
    /// 插入在 shelf 和条目之间的内边距。
    pub padding: u32,
    cursor_x: u32,
    cursor_y: u32,
    shelf_height: u32,
    entries: Vec<AtlasEntry>,
}

impl AtlasRect {
    /// 创建像素空间矩形。
    #[inline]
    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl TextureAtlas {
    /// 创建空图集。
    #[inline]
    pub const fn new(width: u32, height: u32) -> Self {
        Self::with_padding(width, height, 0)
    }

    /// 创建带内边距的空图集。
    #[inline]
    pub const fn with_padding(width: u32, height: u32, padding: u32) -> Self {
        Self {
            width,
            height,
            padding,
            cursor_x: 0,
            cursor_y: 0,
            shelf_height: 0,
            entries: Vec::new(),
        }
    }

    /// 插入命名矩形并返回其像素空间位置。
    pub fn insert(
        &mut self,
        name: impl Into<String>,
        width: u32,
        height: u32,
    ) -> Result<AtlasRect, ValidationError> {
        if self.width == 0 || self.height == 0 || width == 0 || height == 0 {
            return Err(ValidationError::OutOfRange);
        }
        if width > self.width || height > self.height {
            return Err(ValidationError::OutOfRange);
        }

        let name = name.into();
        if self.entries.iter().any(|entry| entry.name == name) {
            return Err(ValidationError::InvalidState);
        }

        if self.cursor_x > 0 && self.cursor_x + width > self.width {
            self.cursor_x = 0;
            self.cursor_y = self
                .cursor_y
                .checked_add(self.shelf_height)
                .and_then(|value| value.checked_add(self.padding))
                .ok_or(ValidationError::OutOfRange)?;
            self.shelf_height = 0;
        }

        if self.cursor_y + height > self.height {
            return Err(ValidationError::OutOfRange);
        }

        let rect = AtlasRect::new(self.cursor_x, self.cursor_y, width, height);
        let uv = self.make_uv(rect);
        self.entries.push(AtlasEntry { name, rect, uv });

        self.cursor_x = self
            .cursor_x
            .checked_add(width)
            .and_then(|value| value.checked_add(self.padding))
            .ok_or(ValidationError::OutOfRange)?;
        self.shelf_height = self.shelf_height.max(height);

        Ok(rect)
    }

    /// 返回命名的像素空间矩形。
    #[inline]
    pub fn rect(&self, name: &str) -> Option<AtlasRect> {
        self.entries
            .iter()
            .find(|entry| entry.name == name)
            .map(|entry| entry.rect)
    }

    /// 返回命名的归一化 UV 矩形。
    #[inline]
    pub fn uv(&self, name: &str) -> Option<UvRect> {
        self.entries
            .iter()
            .find(|entry| entry.name == name)
            .map(|entry| entry.uv)
    }

    /// 按插入顺序返回所有打包的条目。
    #[inline]
    pub fn entries(&self) -> &[AtlasEntry] {
        &self.entries
    }

    /// 根据矩形计算归一化 UV 坐标。
    #[inline]
    fn make_uv(&self, rect: AtlasRect) -> UvRect {
        UvRect {
            u0: rect.x as f32 / self.width as f32,
            v0: rect.y as f32 / self.height as f32,
            u1: (rect.x + rect.width) as f32 / self.width as f32,
            v1: (rect.y + rect.height) as f32 / self.height as f32,
        }
    }
}
