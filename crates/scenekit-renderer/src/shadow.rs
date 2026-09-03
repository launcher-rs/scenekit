/// 用于阴影投射光源的共享深度纹理数组。
#[derive(Debug)]
pub struct ShadowMapAtlas {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    size: u32,
    layers: u32,
}

impl ShadowMapAtlas {
    /// 分配一个方形深度纹理数组。
    pub fn new(device: &wgpu::Device, size: u32, layers: u32) -> Self {
        let size = size.max(1);
        let layers = layers.max(1);
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("scenekit.shadow.atlas"),
            size: wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: layers,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("scenekit.shadow.atlas.view"),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        Self {
            texture,
            view,
            size,
            layers,
        }
    }

    /// 返回着色器使用的纹理视图。
    #[inline]
    pub const fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    /// 返回底层纹理。
    #[inline]
    pub const fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    /// 返回阴影贴图宽度和高度。
    #[inline]
    pub const fn size(&self) -> u32 {
        self.size
    }

    /// 返回纹理数组层数。
    #[inline]
    pub const fn layers(&self) -> u32 {
        self.layers
    }
}
