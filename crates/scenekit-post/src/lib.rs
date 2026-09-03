//! scenekit 的 GPU 后处理栈。
//!
//! `scenekit-post` 拥有全屏 wgpu 通道，不依赖渲染器 crate。
//! 渲染器可以向其提供源纹理视图和最终目标视图，同时避免 Cargo 依赖循环。

use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use scenekit_core::{ScenixError, ValidationError};
use scenekit_math::Vec2;

/// 每帧后处理上下文。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PostContext {
    /// 单调递增帧索引。
    pub frame_index: u64,
    /// 目标分辨率（像素）。
    pub resolution: Vec2,
    /// 输出颜色格式。
    pub color_format: wgpu::TextureFormat,
}

/// 后处理调度的 CPU 计数器。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PostStats {
    /// 提交的已启用后处理通道数。
    pub passes: u32,
    /// 本次调度中调整大小的临时目标数。
    pub resized_targets: u32,
}

/// 泛光高亮提取和叠加辉光配置。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BloomConfig {
    /// 泛光开始的亮度阈值。
    pub threshold: f32,
    /// 泛光贡献乘数。
    pub intensity: f32,
    /// 近似模糊半径（像素）。
    pub radius: f32,
}

impl BloomConfig {
    /// 返回钳位后的配置。
    pub fn normalized(self) -> Self {
        Self {
            threshold: self.threshold.clamp(0.0, 16.0),
            intensity: self.intensity.clamp(0.0, 16.0),
            radius: self.radius.clamp(0.0, 64.0),
        }
    }
}

impl Default for BloomConfig {
    #[inline]
    fn default() -> Self {
        Self {
            threshold: 1.0,
            intensity: 0.35,
            radius: 4.0,
        }
    }
}

/// 屏幕空间环境光遮蔽后处理配置。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SsaoConfig {
    /// 视图空间单位的采样半径。
    pub radius: f32,
    /// 遮蔽强度。
    pub intensity: f32,
    /// 自遮蔽偏移。
    pub bias: f32,
}

impl SsaoConfig {
    /// 返回钳位后的配置。
    pub fn normalized(self) -> Self {
        Self {
            radius: self.radius.clamp(0.0, 16.0),
            intensity: self.intensity.clamp(0.0, 4.0),
            bias: self.bias.clamp(0.0, 1.0),
        }
    }
}

impl Default for SsaoConfig {
    #[inline]
    fn default() -> Self {
        Self {
            radius: 0.5,
            intensity: 1.0,
            bias: 0.025,
        }
    }
}

/// 色调映射运算符。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ToneMapper {
    /// 保持颜色不变。
    None,
    /// Reinhard 色调映射。
    Reinhard,
    /// 类 ACES 的电影感曲线。
    #[default]
    Aces,
    /// 指数曝光曲线。
    Exposure(f32),
}

/// 快速近似抗锯齿配置。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FxaaConfig {
    /// 触发平滑的最小对比度。
    pub contrast_threshold: f32,
    /// 相对对比度阈值。
    pub relative_threshold: f32,
}

impl FxaaConfig {
    /// 返回钳位后的配置。
    pub fn normalized(self) -> Self {
        Self {
            contrast_threshold: self.contrast_threshold.clamp(0.0, 1.0),
            relative_threshold: self.relative_threshold.clamp(0.0, 1.0),
        }
    }
}

impl Default for FxaaConfig {
    #[inline]
    fn default() -> Self {
        Self {
            contrast_threshold: 0.0312,
            relative_threshold: 0.125,
        }
    }
}

/// 时间抗锯齿混合配置。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaaConfig {
    /// 历史反馈值，范围 `0..=1`。
    pub feedback: f32,
    /// 抖动量（像素）。
    pub jitter: f32,
}

impl TaaConfig {
    /// 返回钳位后的配置。
    pub fn normalized(self) -> Self {
        Self {
            feedback: self.feedback.clamp(0.0, 1.0),
            jitter: self.jitter.clamp(0.0, 2.0),
        }
    }
}

impl Default for TaaConfig {
    #[inline]
    fn default() -> Self {
        Self {
            feedback: 0.9,
            jitter: 0.5,
        }
    }
}

/// SMAA 质量预设。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SmaaQuality {
    /// 低质量。
    Low,
    /// 均衡质量。
    #[default]
    Medium,
    /// 高质量。
    High,
    /// 超高质量。
    Ultra,
}

/// SMAA 配置。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SmaaConfig {
    /// 质量预设。
    pub quality: SmaaQuality,
}

/// 景深后处理配置。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DofConfig {
    /// 场景单位的对焦距离。
    pub focus_distance: f32,
    /// 光圈强度。
    pub aperture: f32,
    /// 最大模糊半径（像素）。
    pub max_blur_radius: f32,
}

impl DofConfig {
    /// 返回钳位后的配置。
    pub fn normalized(self) -> Self {
        Self {
            focus_distance: self.focus_distance.max(0.001),
            aperture: self.aperture.clamp(0.0, 32.0),
            max_blur_radius: self.max_blur_radius.clamp(0.0, 64.0),
        }
    }
}

impl Default for DofConfig {
    #[inline]
    fn default() -> Self {
        Self {
            focus_distance: 10.0,
            aperture: 1.4,
            max_blur_radius: 8.0,
        }
    }
}

/// 雾混合后处理配置。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FogPostConfig {
    /// 线性 RGB 雾颜色。
    pub color: [f32; 3],
    /// 雾密度。
    pub density: f32,
}

impl FogPostConfig {
    /// 返回钳位后的配置。
    pub fn normalized(self) -> Self {
        Self {
            color: [
                self.color[0].clamp(0.0, 1.0),
                self.color[1].clamp(0.0, 1.0),
                self.color[2].clamp(0.0, 1.0),
            ],
            density: self.density.clamp(0.0, 1.0),
        }
    }
}

impl Default for FogPostConfig {
    #[inline]
    fn default() -> Self {
        Self {
            color: [0.5, 0.6, 0.7],
            density: 0.05,
        }
    }
}

/// 边缘轮廓后处理配置。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OutlineConfig {
    /// 线性 RGBA 轮廓颜色。
    pub color: [f32; 4],
    /// 亮度边缘阈值。
    pub threshold: f32,
    /// 边缘采样距离（像素）。
    pub thickness: f32,
}

impl OutlineConfig {
    /// 返回钳位后的配置。
    pub fn normalized(self) -> Self {
        Self {
            color: [
                self.color[0].clamp(0.0, 1.0),
                self.color[1].clamp(0.0, 1.0),
                self.color[2].clamp(0.0, 1.0),
                self.color[3].clamp(0.0, 1.0),
            ],
            threshold: self.threshold.clamp(0.0, 1.0),
            thickness: self.thickness.clamp(0.0, 16.0),
        }
    }
}

impl Default for OutlineConfig {
    #[inline]
    fn default() -> Self {
        Self {
            color: [0.0, 0.0, 0.0, 1.0],
            threshold: 0.1,
            thickness: 1.0,
        }
    }
}

/// 相机运动模糊后处理配置。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MotionBlurConfig {
    /// 模糊强度。
    pub strength: f32,
    /// 概念采样数。v0.7 着色器将其映射为紧凑的固定模式。
    pub sample_count: u32,
}

impl MotionBlurConfig {
    /// 返回钳位后的配置。
    pub fn normalized(self) -> Self {
        Self {
            strength: self.strength.clamp(0.0, 1.0),
            sample_count: self.sample_count.clamp(1, 32),
        }
    }
}

impl Default for MotionBlurConfig {
    #[inline]
    fn default() -> Self {
        Self {
            strength: 0.08,
            sample_count: 8,
        }
    }
}

/// 按栈顺序排列的单个后处理效果。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PostEffect {
    /// 泛光通道。
    Bloom(BloomConfig),
    /// SSAO 近似通道。
    Ssao(SsaoConfig),
    /// 色调映射通道。
    Tonemap(ToneMapper),
    /// FXAA 通道。
    Fxaa(FxaaConfig),
    /// TAA 混合通道。
    Taa(TaaConfig),
    /// SMAA 通道。
    Smaa(SmaaConfig),
    /// 景深通道。
    Dof(DofConfig),
    /// 雾混合通道。
    Fog(FogPostConfig),
    /// 轮廓边缘通道。
    Outline(OutlineConfig),
    /// 运动模糊通道。
    MotionBlur(MotionBlurConfig),
}

impl PostEffect {
    /// 返回 WGSL 着色器使用的稳定数字类型 ID。
    #[inline]
    pub const fn kind_id(&self) -> u32 {
        match self {
            Self::Bloom(_) => 1,
            Self::Ssao(_) => 2,
            Self::Tonemap(_) => 3,
            Self::Fxaa(_) => 4,
            Self::Taa(_) => 5,
            Self::Smaa(_) => 6,
            Self::Dof(_) => 7,
            Self::Fog(_) => 8,
            Self::Outline(_) => 9,
            Self::MotionBlur(_) => 10,
        }
    }

    fn params(&self) -> [f32; 8] {
        match *self {
            Self::Bloom(config) => {
                let config = config.normalized();
                [
                    config.threshold,
                    config.intensity,
                    config.radius,
                    0.0,
                    self.kind_id() as f32,
                    0.0,
                    0.0,
                    0.0,
                ]
            }
            Self::Ssao(config) => {
                let config = config.normalized();
                [
                    config.radius,
                    config.intensity,
                    config.bias,
                    0.0,
                    self.kind_id() as f32,
                    0.0,
                    0.0,
                    0.0,
                ]
            }
            Self::Tonemap(mapper) => {
                let (mode, exposure) = match mapper {
                    ToneMapper::None => (0.0, 1.0),
                    ToneMapper::Reinhard => (1.0, 1.0),
                    ToneMapper::Aces => (2.0, 1.0),
                    ToneMapper::Exposure(exposure) => (3.0, exposure.max(0.0)),
                };
                [
                    mode,
                    exposure,
                    0.0,
                    0.0,
                    self.kind_id() as f32,
                    0.0,
                    0.0,
                    0.0,
                ]
            }
            Self::Fxaa(config) => {
                let config = config.normalized();
                [
                    config.contrast_threshold,
                    config.relative_threshold,
                    0.0,
                    0.0,
                    self.kind_id() as f32,
                    0.0,
                    0.0,
                    0.0,
                ]
            }
            Self::Taa(config) => {
                let config = config.normalized();
                [
                    config.feedback,
                    config.jitter,
                    0.0,
                    0.0,
                    self.kind_id() as f32,
                    0.0,
                    0.0,
                    0.0,
                ]
            }
            Self::Smaa(config) => {
                let quality = match config.quality {
                    SmaaQuality::Low => 0.25,
                    SmaaQuality::Medium => 0.5,
                    SmaaQuality::High => 0.75,
                    SmaaQuality::Ultra => 1.0,
                };
                [quality, 0.0, 0.0, 0.0, self.kind_id() as f32, 0.0, 0.0, 0.0]
            }
            Self::Dof(config) => {
                let config = config.normalized();
                [
                    config.focus_distance,
                    config.aperture,
                    config.max_blur_radius,
                    0.0,
                    self.kind_id() as f32,
                    0.0,
                    0.0,
                    0.0,
                ]
            }
            Self::Fog(config) => {
                let config = config.normalized();
                [
                    config.color[0],
                    config.color[1],
                    config.color[2],
                    config.density,
                    self.kind_id() as f32,
                    0.0,
                    0.0,
                    0.0,
                ]
            }
            Self::Outline(config) => {
                let config = config.normalized();
                [
                    config.color[0],
                    config.color[1],
                    config.threshold,
                    config.thickness,
                    self.kind_id() as f32,
                    config.color[2],
                    config.color[3],
                    0.0,
                ]
            }
            Self::MotionBlur(config) => {
                let config = config.normalized();
                [
                    config.strength,
                    config.sample_count as f32,
                    0.0,
                    0.0,
                    self.kind_id() as f32,
                    0.0,
                    0.0,
                    0.0,
                ]
            }
        }
    }
}

/// 渲染器持有的后处理纹理目标。
pub struct PostTarget {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
}

impl PostTarget {
    /// 分配适合后处理的纹理目标。
    pub fn new(
        device: &wgpu::Device,
        label: &str,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
    ) -> Result<Self, ScenixError> {
        if width == 0 || height == 0 {
            return Err(ScenixError::Validation(ValidationError::OutOfRange));
        }
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
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Ok(Self {
            texture,
            view,
            width,
            height,
            format,
        })
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

    /// 返回目标宽度。
    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// 返回目标高度。
    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// 返回纹理格式。
    #[inline]
    pub const fn format(&self) -> wgpu::TextureFormat {
        self.format
    }
}

/// 带有缓存 GPU 资源的有序后处理栈。
pub struct PostStack {
    effects: Vec<PostEffect>,
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    sampler: Option<wgpu::Sampler>,
    uniform_buffer: Option<wgpu::Buffer>,
    pipelines: Vec<(wgpu::TextureFormat, Arc<wgpu::RenderPipeline>)>,
    scratch: [Option<PostTarget>; 2],
}

impl PostStack {
    /// 创建空后处理栈。
    #[inline]
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
            bind_group_layout: None,
            sampler: None,
            uniform_buffer: None,
            pipelines: Vec::new(),
            scratch: [None, None],
        }
    }

    /// 返回有序效果列表。
    #[inline]
    pub fn effects(&self) -> &[PostEffect] {
        &self.effects
    }

    /// 返回栈中效果数量。
    #[inline]
    pub fn len(&self) -> usize {
        self.effects.len()
    }

    /// 返回栈是否为空。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }

    /// 添加泛光通道。
    pub fn with_bloom(mut self, config: BloomConfig) -> Self {
        self.effects.push(PostEffect::Bloom(config.normalized()));
        self
    }

    /// 添加 SSAO 通道。
    pub fn with_ssao(mut self, config: SsaoConfig) -> Self {
        self.effects.push(PostEffect::Ssao(config.normalized()));
        self
    }

    /// 添加色调映射通道。
    pub fn with_tonemap(mut self, mapper: ToneMapper) -> Self {
        self.effects.push(PostEffect::Tonemap(mapper));
        self
    }

    /// 添加 FXAA 通道。
    pub fn with_fxaa(mut self, config: FxaaConfig) -> Self {
        self.effects.push(PostEffect::Fxaa(config.normalized()));
        self
    }

    /// 添加 TAA 通道。
    pub fn with_taa(mut self, config: TaaConfig) -> Self {
        self.effects.push(PostEffect::Taa(config.normalized()));
        self
    }

    /// 添加 SMAA 通道。
    pub fn with_smaa(mut self, config: SmaaConfig) -> Self {
        self.effects.push(PostEffect::Smaa(config));
        self
    }

    /// 添加景深通道。
    pub fn with_dof(mut self, config: DofConfig) -> Self {
        self.effects.push(PostEffect::Dof(config.normalized()));
        self
    }

    /// 添加雾混合通道。
    pub fn with_fog(mut self, config: FogPostConfig) -> Self {
        self.effects.push(PostEffect::Fog(config.normalized()));
        self
    }

    /// 添加轮廓通道。
    pub fn with_outline(mut self, config: OutlineConfig) -> Self {
        self.effects.push(PostEffect::Outline(config.normalized()));
        self
    }

    /// 添加运动模糊通道。
    pub fn with_motion_blur(mut self, config: MotionBlurConfig) -> Self {
        self.effects
            .push(PostEffect::MotionBlur(config.normalized()));
        self
    }

    /// 按索引移除效果。
    pub fn remove(&mut self, index: usize) -> Option<PostEffect> {
        if index < self.effects.len() {
            Some(self.effects.remove(index))
        } else {
            None
        }
    }

    /// 清除所有效果，保留 GPU 资源以供复用。
    #[inline]
    pub fn clear(&mut self) {
        self.effects.clear();
    }

    /// 将栈从 `source` 应用到 `output` 并提交 GPU 命令缓冲区。
    pub fn apply_to_view(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        source: &wgpu::TextureView,
        output: &wgpu::TextureView,
        context: PostContext,
    ) -> Result<PostStats, ScenixError> {
        if self.effects.is_empty() {
            return Ok(PostStats::default());
        }
        if context.resolution.x <= 0.0 || context.resolution.y <= 0.0 {
            return Err(ScenixError::Validation(ValidationError::OutOfRange));
        }

        let width = context.resolution.x as u32;
        let height = context.resolution.y as u32;
        let resized_targets =
            self.ensure_scratch_targets(device, width, height, context.color_format)?;
        let pipeline = self.pipeline(device, context.color_format);
        self.ensure_common_resources(device);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("scenekit.post.encoder"),
        });
        let mut current_scratch: Option<usize> = None;

        for index in 0..self.effects.len() {
            let last = index + 1 == self.effects.len();
            let destination_scratch = if last { None } else { Some(index % 2) };
            let source_view = if let Some(scratch) = current_scratch {
                self.scratch[scratch].as_ref().unwrap().view()
            } else {
                source
            };
            let destination_view = if let Some(scratch) = destination_scratch {
                self.scratch[scratch].as_ref().unwrap().view()
            } else {
                output
            };

            let params = self.effects[index].params();
            self.render_effect(EffectPass {
                device,
                queue,
                pipeline: &pipeline,
                encoder: &mut encoder,
                source: source_view,
                destination: destination_view,
                params: &params,
            });
            current_scratch = destination_scratch;
        }

        queue.submit(Some(encoder.finish()));
        Ok(PostStats {
            passes: self.effects.len() as u32,
            resized_targets,
        })
    }

    fn ensure_scratch_targets(
        &mut self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
    ) -> Result<u32, ScenixError> {
        if self.effects.len() <= 1 {
            return Ok(0);
        }

        let mut resized = 0;
        for index in 0..2 {
            let replace = self.scratch[index].as_ref().is_none_or(|target| {
                target.width() != width || target.height() != height || target.format() != format
            });
            if replace {
                self.scratch[index] = Some(PostTarget::new(
                    device,
                    if index == 0 {
                        "scenekit.post.scratch.0"
                    } else {
                        "scenekit.post.scratch.1"
                    },
                    width,
                    height,
                    format,
                )?);
                resized += 1;
            }
        }
        Ok(resized)
    }

    fn ensure_common_resources(&mut self, device: &wgpu::Device) {
        if self.bind_group_layout.is_none() {
            self.bind_group_layout = Some(device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some("scenekit.post.bind_group_layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                },
            ));
        }
        if self.sampler.is_none() {
            self.sampler = Some(device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("scenekit.post.sampler"),
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::MipmapFilterMode::Linear,
                ..Default::default()
            }));
        }
        if self.uniform_buffer.is_none() {
            self.uniform_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("scenekit.post.uniforms"),
                size: std::mem::size_of::<PostUniform>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }
    }

    fn pipeline(
        &mut self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
    ) -> Arc<wgpu::RenderPipeline> {
        if let Some((_, pipeline)) = self
            .pipelines
            .iter()
            .find(|(pipeline_format, _)| *pipeline_format == format)
        {
            return Arc::clone(pipeline);
        }

        self.ensure_common_resources(device);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("scenekit.post.shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("scenekit.post.pipeline_layout"),
            bind_group_layouts: &[Some(self.bind_group_layout.as_ref().unwrap())],
            immediate_size: 0,
        });
        let pipeline = Arc::new(
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("scenekit.post.pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                multiview_mask: None,
                cache: None,
            }),
        );
        self.pipelines.push((format, Arc::clone(&pipeline)));
        pipeline
    }

    fn render_effect(&self, pass: EffectPass<'_>) {
        let uniform = PostUniform {
            values: *pass.params,
        };
        pass.queue.write_buffer(
            self.uniform_buffer.as_ref().unwrap(),
            0,
            bytemuck::bytes_of(&uniform),
        );
        let bind_group = pass.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scenekit.post.bind_group"),
            layout: self.bind_group_layout.as_ref().unwrap(),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(pass.source),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(self.sampler.as_ref().unwrap()),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: self.uniform_buffer.as_ref().unwrap().as_entire_binding(),
                },
            ],
        });
        let mut render_pass = pass.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("scenekit.post.pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: pass.destination,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        render_pass.set_pipeline(pass.pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}

impl Default for PostStack {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct PostUniform {
    values: [f32; 8],
}

struct EffectPass<'a> {
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    pipeline: &'a wgpu::RenderPipeline,
    encoder: &'a mut wgpu::CommandEncoder,
    source: &'a wgpu::TextureView,
    destination: &'a wgpu::TextureView,
    params: &'a [f32; 8],
}
