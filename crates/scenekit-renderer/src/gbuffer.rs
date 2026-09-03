/// 渲染器拥有目标所使用的 GPU 纹理和视图对。
#[derive(Debug)]
pub struct TextureTarget {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    format: wgpu::TextureFormat,
    width: u32,
    height: u32,
}

/// 渲染器拥有的渲染目标类型。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenderTargetKind {
    /// 渲染为 2D 纹理的颜色目标。
    Color2D,
    /// 渲染为 2D 纹理的 HDR 颜色目标。
    Hdr2D,
    /// 仅深度目标。
    Depth,
    /// 立方体目标元数据。v1.2 通过 2D 视图渲染各个捕获。
    Cube,
}

/// 通过 `TextureId` 注册的渲染器拥有渲染目标的描述符。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderTargetDescriptor {
    /// 宽度（像素）。
    pub width: u32,
    /// 高度（像素）。
    pub height: u32,
    /// 纹理格式。
    pub format: wgpu::TextureFormat,
    /// 目标类型。
    pub kind: RenderTargetKind,
    /// 多重采样样本数。v1.2 渲染到纹理使用单个样本。
    pub sample_count: u32,
}

impl RenderTargetDescriptor {
    /// 创建颜色 2D 渲染目标描述符。
    #[inline]
    pub const fn color(width: u32, height: u32, format: wgpu::TextureFormat) -> Self {
        Self {
            width,
            height,
            format,
            kind: RenderTargetKind::Color2D,
            sample_count: 1,
        }
    }

    /// 创建 HDR 颜色 2D 渲染目标描述符。
    #[inline]
    pub const fn hdr(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            format: wgpu::TextureFormat::Rgba16Float,
            kind: RenderTargetKind::Hdr2D,
            sample_count: 1,
        }
    }

    /// 创建深度渲染目标描述符。
    #[inline]
    pub const fn depth(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            format: wgpu::TextureFormat::Depth32Float,
            kind: RenderTargetKind::Depth,
            sample_count: 1,
        }
    }
}

impl TextureTarget {
    /// 分配一个可渲染的纹理目标。
    pub fn new(
        device: &wgpu::Device,
        label: &'static str,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        usage: wgpu::TextureUsages,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self {
            texture,
            view,
            format,
            width,
            height,
        }
    }

    /// 返回纹理视图。
    #[inline]
    pub const fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    /// 返回底层纹理。
    #[inline]
    pub const fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    /// 返回目标格式。
    #[inline]
    pub const fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    /// 返回宽度（像素）。
    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// 返回高度（像素）。
    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }
}

/// 延迟渲染器 G缓冲区 纹理。
#[derive(Debug)]
pub struct GBuffer {
    albedo: TextureTarget,
    normal: TextureTarget,
    material: TextureTarget,
    depth: TextureTarget,
    width: u32,
    height: u32,
}

impl GBuffer {
    /// 分配 G缓冲区 附件。
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let color_usage =
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        let depth_usage =
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        Self {
            albedo: TextureTarget::new(
                device,
                "scenekit.gbuffer.albedo",
                width,
                height,
                wgpu::TextureFormat::Rgba8Unorm,
                color_usage,
            ),
            normal: TextureTarget::new(
                device,
                "scenekit.gbuffer.normal",
                width,
                height,
                wgpu::TextureFormat::Rgba16Float,
                color_usage,
            ),
            material: TextureTarget::new(
                device,
                "scenekit.gbuffer.material",
                width,
                height,
                wgpu::TextureFormat::Rgba8Unorm,
                color_usage,
            ),
            depth: TextureTarget::new(
                device,
                "scenekit.gbuffer.depth",
                width,
                height,
                wgpu::TextureFormat::Depth32Float,
                depth_usage,
            ),
            width,
            height,
        }
    }

    /// 当目标尺寸变化时重新分配附件。
    #[inline]
    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        if self.width != width || self.height != height {
            *self = Self::new(device, width, height);
        }
    }

    /// 返回宽度（像素）。
    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// 返回高度（像素）。
    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// 返回反照率附件。
    #[inline]
    pub const fn albedo(&self) -> &TextureTarget {
        &self.albedo
    }

    /// 返回法线附件。
    #[inline]
    pub const fn normal(&self) -> &TextureTarget {
        &self.normal
    }

    /// 返回材质附件。
    #[inline]
    pub const fn material(&self) -> &TextureTarget {
        &self.material
    }

    /// 返回深度附件。
    #[inline]
    pub const fn depth(&self) -> &TextureTarget {
        &self.depth
    }
}
