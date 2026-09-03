use scenekit_core::ValidationError;

/// 渲染器实例使用的渲染目标后端。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenderTargetMode {
    /// 帧呈现到平台表面。
    Surface,
    /// 帧渲染到离屏纹理，用于测试、工具或捕获。
    Headless,
}

/// 渲染器初始化和调整大小配置。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RendererConfig {
    /// 渲染目标宽度（像素）。
    pub width: u32,
    /// 渲染目标高度（像素）。
    pub height: u32,
    /// 多重采样样本数。v0.6 支持 `1` 和 `4`。
    pub sample_count: u32,
    /// 呈现是否应与显示刷新同步。
    pub vsync: bool,
    /// 渲染器是否应使用 HDR 中间颜色目标。
    pub hdr: bool,
    /// 表面呈现模式。
    pub present_mode: wgpu::PresentMode,
    /// 请求的 wgpu 后端。
    pub backends: wgpu::Backends,
    /// 绘制前使用的清除颜色。
    pub clear_color: wgpu::Color,
}

impl RendererConfig {
    /// 为给定尺寸创建渲染器配置。
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            ..Self::default()
        }
    }

    /// 验证渲染目标尺寸和多重采样数量。
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.width == 0 || self.height == 0 {
            return Err(ValidationError::OutOfRange);
        }
        if !matches!(self.sample_count, 1 | 4) {
            return Err(ValidationError::OutOfRange);
        }
        if self.backends.is_empty() {
            return Err(ValidationError::InvalidState);
        }
        Ok(())
    }

    /// 返回此配置的首选颜色格式。
    #[inline]
    pub const fn preferred_color_format(&self) -> wgpu::TextureFormat {
        if self.hdr {
            wgpu::TextureFormat::Rgba16Float
        } else {
            wgpu::TextureFormat::Bgra8UnormSrgb
        }
    }

    /// 返回尺寸已更改的副本。
    #[inline]
    pub fn resized(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// 返回配置为垂直同步的副本。
    #[inline]
    pub fn vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self.present_mode = if vsync {
            wgpu::PresentMode::Fifo
        } else {
            wgpu::PresentMode::Immediate
        };
        self
    }

    /// 返回配置为 HDR 中间目标的副本。
    #[inline]
    pub const fn hdr(mut self, hdr: bool) -> Self {
        self.hdr = hdr;
        self
    }
}

impl Default for RendererConfig {
    #[inline]
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            sample_count: 1,
            vsync: true,
            hdr: false,
            present_mode: wgpu::PresentMode::Fifo,
            backends: wgpu::Backends::all(),
            clear_color: wgpu::Color {
                r: 0.02,
                g: 0.025,
                b: 0.035,
                a: 1.0,
            },
        }
    }
}
