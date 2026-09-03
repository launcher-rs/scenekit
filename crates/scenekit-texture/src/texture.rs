use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;

use scenekit_core::ValidationError;

use crate::TextureFormat;

/// CPU 端 2D 纹理字节。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Texture2D {
    /// 宽度（纹素）。
    pub width: u32,
    /// 高度（纹素）。
    pub height: u32,
    /// 纹理格式。
    pub format: TextureFormat,
    /// 连续的纹理字节。
    pub data: Vec<u8>,
    /// mip 级别数量。`0` 表示基础数据，稍后自动生成。
    pub mip_levels: u32,
    /// 可选的调试标签。
    pub label: Option<String>,
}

/// CPU 端立方体纹理字节。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextureCube {
    /// 面的宽度和高度（纹素）。
    pub size: u32,
    /// 纹理格式。
    pub format: TextureFormat,
    /// 六个面，按 +X、-X、+Y、-Y、+Z、-Z 顺序排列。
    pub faces: [Vec<u8>; 6],
    /// 每个面的 mip 级别数量。
    pub mip_levels: u32,
    /// 可选的调试标签。
    pub label: Option<String>,
}

/// CPU 端 3D 纹理字节。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Texture3D {
    /// 宽度（纹素）。
    pub width: u32,
    /// 高度（纹素）。
    pub height: u32,
    /// 深度（纹素）。
    pub depth: u32,
    /// 纹理格式。
    pub format: TextureFormat,
    /// 连续的纹理字节。
    pub data: Vec<u8>,
    /// mip 级别数量。`0` 表示基础数据，稍后自动生成。
    pub mip_levels: u32,
    /// 可选的调试标签。
    pub label: Option<String>,
}

impl Texture2D {
    /// 创建基础级别纹理并验证字节长度。
    #[inline]
    pub fn new(
        width: u32,
        height: u32,
        format: TextureFormat,
        data: Vec<u8>,
    ) -> Result<Self, ValidationError> {
        Self::with_mip_levels(width, height, format, data, 1)
    }

    /// 创建具有显式 mip 级别数量的纹理并验证字节长度。
    pub fn with_mip_levels(
        width: u32,
        height: u32,
        format: TextureFormat,
        data: Vec<u8>,
        mip_levels: u32,
    ) -> Result<Self, ValidationError> {
        let texture = Self {
            width,
            height,
            format,
            data,
            mip_levels,
            label: None,
        };
        texture.validate()?;
        Ok(texture)
    }

    /// 从显式 mip 级别创建纹理并将数据展平。
    pub fn from_mips(
        width: u32,
        height: u32,
        format: TextureFormat,
        mips: Vec<Vec<u8>>,
    ) -> Result<Self, ValidationError> {
        if mips.is_empty() {
            return Err(ValidationError::OutOfRange);
        }
        validate_2d_mips(format, width, height, &mips)?;

        let mip_levels = mips.len() as u32;
        let total_len = mips.iter().try_fold(0_usize, |total, mip| {
            total
                .checked_add(mip.len())
                .ok_or(ValidationError::OutOfRange)
        })?;
        let mut data = Vec::with_capacity(total_len);
        for mip in mips {
            data.extend_from_slice(&mip);
        }

        Ok(Self {
            width,
            height,
            format,
            data,
            mip_levels,
            label: None,
        })
    }

    /// 返回带有标签的此纹理。
    #[inline]
    pub fn labeled(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// 返回基础 mip 级别的预期字节长度。
    #[inline]
    pub fn base_level_len(&self) -> Result<usize, ValidationError> {
        self.format.expected_2d_len(self.width, self.height)
    }

    /// 返回 `data` 中一个 mip 级别占用的字节范围。
    pub fn mip_level_range(&self, level: u32) -> Result<Range<usize>, ValidationError> {
        mip_level_range_2d(self.format, self.width, self.height, self.mip_levels, level)
    }

    /// 验证维度和字节长度。
    pub fn validate(&self) -> Result<(), ValidationError> {
        let expected =
            expected_2d_len_for_mip_count(self.format, self.width, self.height, self.mip_levels)?;
        if self.data.len() == expected {
            Ok(())
        } else {
            Err(ValidationError::OutOfRange)
        }
    }
}

impl TextureCube {
    /// 创建包含六个面的立方体纹理并验证每个面。
    pub fn new(
        size: u32,
        format: TextureFormat,
        faces: [Vec<u8>; 6],
    ) -> Result<Self, ValidationError> {
        let texture = Self {
            size,
            format,
            faces,
            mip_levels: 1,
            label: None,
        };
        texture.validate()?;
        Ok(texture)
    }

    /// 返回带有标签的此立方体纹理。
    #[inline]
    pub fn labeled(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// 验证维度和每个面的字节长度。
    pub fn validate(&self) -> Result<(), ValidationError> {
        let expected =
            expected_2d_len_for_mip_count(self.format, self.size, self.size, self.mip_levels)?;
        if self.faces.iter().all(|face| face.len() == expected) {
            Ok(())
        } else {
            Err(ValidationError::OutOfRange)
        }
    }

    /// 返回每个面内一个 mip 级别占用的字节范围。
    pub fn mip_level_range(&self, level: u32) -> Result<Range<usize>, ValidationError> {
        mip_level_range_2d(self.format, self.size, self.size, self.mip_levels, level)
    }
}

impl Texture3D {
    /// 创建基础级别 3D 纹理并验证字节长度。
    #[inline]
    pub fn new(
        width: u32,
        height: u32,
        depth: u32,
        format: TextureFormat,
        data: Vec<u8>,
    ) -> Result<Self, ValidationError> {
        Self::with_mip_levels(width, height, depth, format, data, 1)
    }

    /// 创建具有显式 mip 级别数量的 3D 纹理并验证字节长度。
    pub fn with_mip_levels(
        width: u32,
        height: u32,
        depth: u32,
        format: TextureFormat,
        data: Vec<u8>,
        mip_levels: u32,
    ) -> Result<Self, ValidationError> {
        let texture = Self {
            width,
            height,
            depth,
            format,
            data,
            mip_levels,
            label: None,
        };
        texture.validate()?;
        Ok(texture)
    }

    /// 返回带有标签的此纹理。
    #[inline]
    pub fn labeled(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// 验证维度和字节长度。
    pub fn validate(&self) -> Result<(), ValidationError> {
        let levels = self.mip_levels.max(1);
        if levels > max_mip_levels_3d(self.width, self.height, self.depth)? {
            return Err(ValidationError::OutOfRange);
        }
        let mut expected = 0_usize;
        for level in 0..levels {
            let (width, height) = TextureFormat::mip_dimensions(self.width, self.height, level);
            let depth = mip_dimension(self.depth, level);
            expected = expected
                .checked_add(self.format.expected_3d_len(width, height, depth)?)
                .ok_or(ValidationError::OutOfRange)?;
        }
        if self.data.len() == expected {
            Ok(())
        } else {
            Err(ValidationError::OutOfRange)
        }
    }

    /// 返回 `data` 中一个 mip 级别占用的字节范围。
    pub fn mip_level_range(&self, level: u32) -> Result<Range<usize>, ValidationError> {
        let levels = self.mip_levels.max(1);
        if level >= levels || levels > max_mip_levels_3d(self.width, self.height, self.depth)? {
            return Err(ValidationError::OutOfRange);
        }
        let mut offset = 0_usize;
        for current in 0..level {
            let (width, height) = TextureFormat::mip_dimensions(self.width, self.height, current);
            let depth = mip_dimension(self.depth, current);
            offset = offset
                .checked_add(self.format.expected_3d_len(width, height, depth)?)
                .ok_or(ValidationError::OutOfRange)?;
        }
        let (width, height) = TextureFormat::mip_dimensions(self.width, self.height, level);
        let depth = mip_dimension(self.depth, level);
        let len = self.format.expected_3d_len(width, height, depth)?;
        Ok(offset..offset + len)
    }
}

/// 计算 2D 纹理 mip 级别的字节范围。
fn mip_level_range_2d(
    format: TextureFormat,
    width: u32,
    height: u32,
    mip_levels: u32,
    level: u32,
) -> Result<Range<usize>, ValidationError> {
    let levels = mip_levels.max(1);
    if level >= levels || levels > max_mip_levels_2d(width, height)? {
        return Err(ValidationError::OutOfRange);
    }
    let mut offset = 0_usize;
    for current in 0..level {
        let (w, h) = TextureFormat::mip_dimensions(width, height, current);
        offset = offset
            .checked_add(format.expected_2d_len(w, h)?)
            .ok_or(ValidationError::OutOfRange)?;
    }
    let (w, h) = TextureFormat::mip_dimensions(width, height, level);
    let len = format.expected_2d_len(w, h)?;
    Ok(offset..offset + len)
}

/// 验证 2D mip 链的尺寸。
fn validate_2d_mips(
    format: TextureFormat,
    width: u32,
    height: u32,
    mips: &[Vec<u8>],
) -> Result<(), ValidationError> {
    if mips.len() > max_mip_levels_2d(width, height)? as usize {
        return Err(ValidationError::OutOfRange);
    }
    for (level, mip) in mips.iter().enumerate() {
        let (w, h) = TextureFormat::mip_dimensions(width, height, level as u32);
        if mip.len() != format.expected_2d_len(w, h)? {
            return Err(ValidationError::OutOfRange);
        }
    }
    Ok(())
}

/// 计算指定 mip 级别数量的预期总字节长度。
fn expected_2d_len_for_mip_count(
    format: TextureFormat,
    width: u32,
    height: u32,
    mip_levels: u32,
) -> Result<usize, ValidationError> {
    let levels = mip_levels.max(1);
    if levels > max_mip_levels_2d(width, height)? {
        return Err(ValidationError::OutOfRange);
    }
    let mut expected = 0_usize;
    for level in 0..levels {
        let (w, h) = TextureFormat::mip_dimensions(width, height, level);
        expected = expected
            .checked_add(format.expected_2d_len(w, h)?)
            .ok_or(ValidationError::OutOfRange)?;
    }
    Ok(expected)
}

/// 计算指定 mip 级别的维度值。
fn mip_dimension(value: u32, level: u32) -> u32 {
    if level >= u32::BITS {
        1
    } else {
        (value >> level).max(1)
    }
}

/// 计算 2D 纹理的最大 mip 级别数量。
fn max_mip_levels_2d(width: u32, height: u32) -> Result<u32, ValidationError> {
    let max_dimension = width.max(height);
    if max_dimension == 0 {
        Err(ValidationError::OutOfRange)
    } else {
        Ok(u32::BITS - max_dimension.leading_zeros())
    }
}

/// 计算 3D 纹理的最大 mip 级别数量。
fn max_mip_levels_3d(width: u32, height: u32, depth: u32) -> Result<u32, ValidationError> {
    let max_dimension = width.max(height).max(depth);
    if max_dimension == 0 {
        Err(ValidationError::OutOfRange)
    } else {
        Ok(u32::BITS - max_dimension.leading_zeros())
    }
}
