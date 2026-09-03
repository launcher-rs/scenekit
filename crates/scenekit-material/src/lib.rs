#![cfg_attr(not(feature = "std"), no_std)]

//! scenekit 的无 GPU 材质描述。
//!
//! 本 crate 定义了渲染器无关的材质和紧凑的管线选择器。
//! 本 crate 不依赖 `wgpu`；GPU 上传和绑定组逻辑
//! 位于渲染器里程碑中。

extern crate alloc;

pub mod depth;
#[cfg(feature = "inspector")]
mod inspector;
pub mod lambert;
pub mod line;
pub mod normal;
pub mod pbr;
pub mod physical;
pub mod points;
pub mod shader;
pub mod toon;
mod traits;
pub mod unlit;
pub mod wireframe;

pub use depth::DepthMaterial;
pub use lambert::LambertMaterial;
pub use line::LineMaterial;
pub use normal::NormalMaterial;
pub use pbr::PbrMaterial;
pub use physical::PhysicalMaterial;
pub use points::PointsMaterial;
pub use shader::ShaderMaterial;
pub use toon::ToonMaterial;
pub use traits::{
    AlphaMode, FEATURE_ALBEDO_TEXTURE, FEATURE_CLEARCOAT, FEATURE_CUSTOM_TEXTURES, FEATURE_DASHED,
    FEATURE_DOUBLE_SIDED, FEATURE_EMISSIVE_TEXTURE, FEATURE_FLAT_SHADING, FEATURE_GRADIENT_TEXTURE,
    FEATURE_IRIDESCENCE, FEATURE_METALLIC_ROUGHNESS_TEXTURE, FEATURE_NORMAL_TEXTURE,
    FEATURE_OCCLUSION_TEXTURE, FEATURE_OUTLINE, FEATURE_SHEEN, FEATURE_SIZE_ATTENUATION,
    FEATURE_TRANSMISSION, FEATURE_VERTEX_COLORS, FEATURE_WIREFRAME, FEATURE_WORLD_SPACE, Material,
    PipelineAlphaMode, PipelineKey, ShaderKind,
};
pub use unlit::UnlitMaterial;
pub use wireframe::WireframeMaterial;
