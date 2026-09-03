//! wgpu 渲染器，GPU 场景上传、渲染通道与帧协调，用于 scenekit。
//!
//! 本 crate 是 scenekit 中第一个依赖 GPU 的层。CPU 端 crate 继续持有创作数据；
//! 本 crate 负责上传、渲染目标分配、渲染通道调度和管线缓存。

pub mod config;
pub mod editor_picking;
pub mod environment;
pub mod frame;
pub mod gbuffer;
pub mod gpu_scene;
#[cfg(feature = "inspector")]
mod inspector;
pub mod material;
pub mod pass;
pub mod pipeline_cache;
pub mod renderer;
mod shadow;
pub mod skinning;

pub use config::{RenderTargetMode, RendererConfig};
pub use editor_picking::{EditorBufferStats, EditorBuffers, EditorPickRequest, EditorPickResult};
pub use environment::EnvironmentMap;
pub use frame::{FrameContext, FrameStats, PipelineCacheStats, RendererDiagnostics, ResourceStats};
pub use gbuffer::{GBuffer, RenderTargetDescriptor, RenderTargetKind};
pub use gpu_scene::{
    DrawSubmission, GpuIndexFormat, GpuMesh, GpuScene, GpuTexture, PackedGeometry, PackedVertex,
    RendererLight, RendererMaterial, TextureStore, to_wgpu_address_mode, to_wgpu_compare,
    to_wgpu_filter_mode, to_wgpu_texture_format,
};
pub use material::{GpuMaterial, MaterialUniform};
pub use pass::culling::{CullingStats, collect_visible_draws};
pub use pass::sort::{sort_opaque_front_to_back, sort_transparent_back_to_front};
pub use pipeline_cache::{PipelineCache, RenderPassKind, RendererPipelineKey};
pub use renderer::Renderer;
pub use shadow::ShadowMapAtlas;
pub use skinning::{GpuSkinningRegistry, SKINNING_WGSL};

pub use wgpu;
