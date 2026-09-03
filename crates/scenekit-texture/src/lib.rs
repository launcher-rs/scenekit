#![cfg_attr(not(feature = "std"), no_std)]

//! scenekit 的 CPU 端纹理数据和采样元数据。
//!
//! 本 crate 存储原始纹理字节、采样器设置、简单的图集打包
//! 以及 CPU mipmap 生成。它不解码图像文件，也不分配 GPU 资源。

extern crate alloc;

pub mod atlas;
pub mod format;
#[cfg(feature = "inspector")]
mod inspector;
pub mod mipmap;
pub mod sampler;
pub mod texture;
pub mod video;

pub use atlas::{AtlasEntry, AtlasRect, TextureAtlas, UvRect};
pub use format::TextureFormat;
pub use sampler::{AddressMode, CompareFunction, FilterMode, Sampler};
pub use texture::{Texture2D, Texture3D, TextureCube};
pub use video::VideoTexture;
