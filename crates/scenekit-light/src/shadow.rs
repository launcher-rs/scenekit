use scenekit_core::ValidationError;

/// 投射阴影的光源类型共享的阴影贴图配置。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShadowConfig {
    /// 阴影贴图宽度和高度（纹素）。
    pub map_size: u32,
    /// 阴影相机的近裁剪距离。
    pub near: f32,
    /// 阴影相机的远裁剪距离。
    pub far: f32,
    /// 用于减少阴影痤疮的深度偏移。
    pub bias: f32,
    /// PCF 内核半径（纹素）。`0` 表示硬阴影。
    pub pcf_radius: u32,
    /// 方向光的级联数量。有效范围为 `1..=4`。
    pub cascades: u8,
}

impl ShadowConfig {
    /// 创建具有保守默认值的阴影配置。
    #[inline]
    pub const fn new(map_size: u32, near: f32, far: f32) -> Self {
        Self {
            map_size,
            near,
            far,
            bias: 0.0005,
            pcf_radius: 1,
            cascades: 1,
        }
    }

    /// 返回设置深度偏移的此配置。
    #[inline]
    pub const fn bias(mut self, bias: f32) -> Self {
        self.bias = bias;
        self
    }

    /// 返回设置 PCF 半径的此配置。
    #[inline]
    pub const fn pcf_radius(mut self, pcf_radius: u32) -> Self {
        self.pcf_radius = pcf_radius;
        self
    }

    /// 返回设置级联数量的此配置。
    #[inline]
    pub const fn cascades(mut self, cascades: u8) -> Self {
        self.cascades = cascades;
        self
    }

    /// 验证贴图大小、裁剪平面和级联数量。
    pub const fn validate(self) -> Result<(), ValidationError> {
        if self.map_size == 0 || !self.map_size.is_power_of_two() {
            return Err(ValidationError::OutOfRange);
        }
        if self.near <= 0.0 || self.far <= self.near {
            return Err(ValidationError::OutOfRange);
        }
        if self.cascades == 0 || self.cascades > 4 {
            return Err(ValidationError::OutOfRange);
        }
        Ok(())
    }
}

impl Default for ShadowConfig {
    #[inline]
    fn default() -> Self {
        Self::new(1024, 0.1, 100.0)
    }
}
