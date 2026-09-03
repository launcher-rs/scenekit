use scenekit_core::ValidationError;

/// CPU 端纹理格式元数据。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextureFormat {
    /// 四通道 8 位归一化线性 RGBA。
    Rgba8Unorm,
    /// 四通道 8 位归一化 sRGB RGBA。
    Rgba8UnormSrgb,
    /// 四通道 16 位浮点 RGBA。
    Rgba16Float,
    /// 32 位浮点深度。
    Depth32Float,
    /// BC7 压缩 RGBA，4x4 块，每块 16 字节。
    Bc7RgbaUnorm,
    /// ASTC 4x4 压缩 RGBA，每块 16 字节。
    Astc4x4RgbaUnorm,
    /// ETC2 RGBA8 压缩数据，4x4 块，每块 16 字节。
    Etc2Rgba8Unorm,
}

impl TextureFormat {
    /// 返回是否为块压缩格式。
    #[inline]
    pub const fn is_compressed(self) -> bool {
        matches!(
            self,
            Self::Bc7RgbaUnorm | Self::Astc4x4RgbaUnorm | Self::Etc2Rgba8Unorm
        )
    }

    /// 返回非压缩格式的每像素字节数。
    #[inline]
    pub const fn bytes_per_pixel(self) -> Option<usize> {
        match self {
            Self::Rgba8Unorm | Self::Rgba8UnormSrgb | Self::Depth32Float => Some(4),
            Self::Rgba16Float => Some(8),
            Self::Bc7RgbaUnorm | Self::Astc4x4RgbaUnorm | Self::Etc2Rgba8Unorm => None,
        }
    }

    /// 返回压缩块的 `(width, height)` 维度。
    #[inline]
    pub const fn block_dimensions(self) -> Option<(u32, u32)> {
        if self.is_compressed() {
            Some((4, 4))
        } else {
            None
        }
    }

    /// 返回每个压缩块的字节数。
    #[inline]
    pub const fn bytes_per_block(self) -> Option<usize> {
        if self.is_compressed() { Some(16) } else { None }
    }

    /// 返回 mip 级别的维度。
    #[inline]
    pub const fn mip_dimensions(width: u32, height: u32, level: u32) -> (u32, u32) {
        let w = shr_clamped(width, level);
        let h = shr_clamped(height, level);
        (if w == 0 { 1 } else { w }, if h == 0 { 1 } else { h })
    }

    /// 返回 2D 纹理级别的预期字节长度。
    #[inline]
    pub fn expected_2d_len(self, width: u32, height: u32) -> Result<usize, ValidationError> {
        self.expected_3d_len(width, height, 1)
    }

    /// 返回 3D 纹理级别的预期字节长度。
    pub fn expected_3d_len(
        self,
        width: u32,
        height: u32,
        depth: u32,
    ) -> Result<usize, ValidationError> {
        if width == 0 || height == 0 || depth == 0 {
            return Err(ValidationError::OutOfRange);
        }

        if let Some(bytes_per_pixel) = self.bytes_per_pixel() {
            checked_area(width, height, depth)?
                .checked_mul(bytes_per_pixel)
                .ok_or(ValidationError::OutOfRange)
        } else {
            let (block_w, block_h) = self.block_dimensions().unwrap_or((4, 4));
            let blocks_x = width.div_ceil(block_w);
            let blocks_y = height.div_ceil(block_h);
            checked_area(blocks_x, blocks_y, depth)?
                .checked_mul(self.bytes_per_block().unwrap_or(16))
                .ok_or(ValidationError::OutOfRange)
        }
    }
}

/// 安全右移，防止移位量超过位宽。
#[inline]
const fn shr_clamped(value: u32, shift: u32) -> u32 {
    if shift >= u32::BITS {
        0
    } else {
        value >> shift
    }
}

/// 安全计算体积，防止溢出。
#[inline]
fn checked_area(width: u32, height: u32, depth: u32) -> Result<usize, ValidationError> {
    (width as usize)
        .checked_mul(height as usize)
        .and_then(|value| value.checked_mul(depth as usize))
        .ok_or(ValidationError::OutOfRange)
}
