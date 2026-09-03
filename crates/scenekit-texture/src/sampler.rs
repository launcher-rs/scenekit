/// 纹理过滤模式。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FilterMode {
    /// 最近邻采样。
    Nearest,
    /// 线性插值。
    #[default]
    Linear,
}

/// 纹理坐标寻址模式。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AddressMode {
    /// 在 `[0, 1]` 范围外重复。
    Repeat,
    /// 镜像重复每个周期。
    MirrorRepeat,
    /// 将坐标钳制到边缘纹素。
    #[default]
    ClampToEdge,
}

/// 可选的深度比较函数，用于阴影/深度采样。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CompareFunction {
    /// 采样值小于参考值时通过。
    Less,
    /// 采样值小于或等于参考值时通过。
    LessEqual,
    /// 采样值大于参考值时通过。
    Greater,
    /// 采样值大于或等于参考值时通过。
    GreaterEqual,
    /// 采样值等于参考值时通过。
    Equal,
    /// 采样值不等于参考值时通过。
    NotEqual,
    /// 始终通过。
    Always,
    /// 从不通过。
    Never,
}

/// CPU 端采样器配置。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sampler {
    /// 放大过滤器。
    pub mag_filter: FilterMode,
    /// 缩小过滤器。
    pub min_filter: FilterMode,
    /// Mipmap 过滤器。
    pub mip_filter: FilterMode,
    /// U 坐标的寻址模式。
    pub address_u: AddressMode,
    /// V 坐标的寻址模式。
    pub address_v: AddressMode,
    /// W 坐标的寻址模式。
    pub address_w: AddressMode,
    /// 各向异性级别，钳制到 `1..=16`。
    pub anisotropy: u8,
    /// 可选的深度比较函数。
    pub compare: Option<CompareFunction>,
}

impl Sampler {
    /// 创建默认的线性钳制采样器。
    #[inline]
    pub const fn new() -> Self {
        Self {
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mip_filter: FilterMode::Linear,
            address_u: AddressMode::ClampToEdge,
            address_v: AddressMode::ClampToEdge,
            address_w: AddressMode::ClampToEdge,
            anisotropy: 1,
            compare: None,
        }
    }

    /// 返回设置过滤模式的此采样器。
    #[inline]
    pub const fn filters(
        mut self,
        mag_filter: FilterMode,
        min_filter: FilterMode,
        mip_filter: FilterMode,
    ) -> Self {
        self.mag_filter = mag_filter;
        self.min_filter = min_filter;
        self.mip_filter = mip_filter;
        self
    }

    /// 返回设置寻址模式的此采样器。
    #[inline]
    pub const fn address_modes(
        mut self,
        address_u: AddressMode,
        address_v: AddressMode,
        address_w: AddressMode,
    ) -> Self {
        self.address_u = address_u;
        self.address_v = address_v;
        self.address_w = address_w;
        self
    }

    /// 返回将各向异性钳制到 `1..=16` 的此采样器。
    #[inline]
    pub const fn anisotropy(mut self, anisotropy: u8) -> Self {
        self.anisotropy = if anisotropy < 1 {
            1
        } else if anisotropy > 16 {
            16
        } else {
            anisotropy
        };
        self
    }

    /// 返回设置比较函数的此采样器。
    #[inline]
    pub const fn compare(mut self, compare: Option<CompareFunction>) -> Self {
        self.compare = compare;
        self
    }
}

impl Default for Sampler {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
