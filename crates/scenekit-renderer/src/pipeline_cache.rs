use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use scenekit_material::{PipelineAlphaMode, PipelineKey, ShaderKind};

/// 用于管线选择的渲染通道族。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenderPassKind {
    /// 不透明 G缓冲区 几何通道。
    Geometry,
    /// 延迟光照解析通道。
    Lighting,
    /// 前向透明/无光照通道。
    Forward,
    /// 仅深度阴影通道。
    Shadow,
}

/// 完整的渲染器管线缓存键。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RendererPipelineKey {
    /// 材质级别管线键。
    pub material: PipelineKey,
    /// 通道族。
    pub pass: RenderPassKind,
    /// 颜色目标格式。
    pub color_format: wgpu::TextureFormat,
    /// 多重采样样本数。
    pub sample_count: u32,
    /// 是否启用深度测试。
    pub depth: bool,
}

impl RendererPipelineKey {
    /// 创建渲染器管线键。
    #[inline]
    pub const fn new(
        material: PipelineKey,
        pass: RenderPassKind,
        color_format: wgpu::TextureFormat,
        sample_count: u32,
        depth: bool,
    ) -> Self {
        Self {
            material,
            pass,
            color_format,
            sample_count,
            depth,
        }
    }
}

/// 以渲染器管线状态为键的惰性渲染管线缓存。
#[derive(Default)]
pub struct PipelineCache {
    pipelines: HashMap<RendererPipelineKey, Arc<wgpu::RenderPipeline>>,
}

impl PipelineCache {
    /// 创建空的管线缓存。
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 返回缓存的管线数量。
    #[inline]
    pub fn len(&self) -> usize {
        self.pipelines.len()
    }

    /// 返回缓存是否为空。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pipelines.is_empty()
    }

    /// 获取现有管线或惰性创建一条。
    pub fn get_or_create(
        &mut self,
        device: &wgpu::Device,
        key: RendererPipelineKey,
    ) -> Arc<wgpu::RenderPipeline> {
        if let Some(pipeline) = self.pipelines.get(&key) {
            return Arc::clone(pipeline);
        }

        let pipeline = Arc::new(create_pipeline(device, key));
        self.pipelines.insert(key, Arc::clone(&pipeline));
        pipeline
    }
}

fn create_pipeline(device: &wgpu::Device, key: RendererPipelineKey) -> wgpu::RenderPipeline {
    let (vertex_source, fragment_source) = shader_sources(key);
    let vertex = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("scenekit.pipeline.vertex"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(vertex_source)),
    });
    let fragment = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("scenekit.pipeline.fragment"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(fragment_source)),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("scenekit.pipeline.layout"),
        bind_group_layouts: &[],
        immediate_size: 0,
    });
    let blend = match key.material.alpha {
        PipelineAlphaMode::Blend => Some(wgpu::BlendState::ALPHA_BLENDING),
        PipelineAlphaMode::Opaque | PipelineAlphaMode::Mask => Some(wgpu::BlendState::REPLACE),
    };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("scenekit.pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &vertex,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: key.sample_count,
            ..Default::default()
        },
        fragment: Some(wgpu::FragmentState {
            module: &fragment,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: key.color_format,
                blend,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        multiview_mask: None,
        cache: None,
    })
}

fn shader_sources(key: RendererPipelineKey) -> (&'static str, &'static str) {
    match key.pass {
        RenderPassKind::Shadow => (
            include_str!("shaders/shadow_depth.vert.wgsl"),
            include_str!("shaders/pbr.frag.wgsl"),
        ),
        RenderPassKind::Lighting => (
            include_str!("shaders/deferred_resolve.wgsl"),
            include_str!("shaders/deferred_resolve.wgsl"),
        ),
        RenderPassKind::Geometry | RenderPassKind::Forward => match key.material.shader {
            ShaderKind::Unlit => (
                include_str!("shaders/pbr.vert.wgsl"),
                include_str!("shaders/unlit.frag.wgsl"),
            ),
            ShaderKind::Pbr | ShaderKind::Lambert => (
                include_str!("shaders/pbr.vert.wgsl"),
                include_str!("shaders/pbr.frag.wgsl"),
            ),
            _ => (
                include_str!("shaders/pbr.vert.wgsl"),
                include_str!("shaders/unlit.frag.wgsl"),
            ),
        },
    }
}
