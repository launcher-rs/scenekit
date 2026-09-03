//! scenekit 的 CPU 端资产加载。
//!
//! 本 crate 将常见的资产文件解码为渲染器无关的 scenekit 数据。
//! 它不创建 GPU 缓冲区、绑定组或渲染器资源。

pub mod asset;
pub mod asset_cache;
pub mod asset_manager;
pub mod export;
pub mod gltf;
pub mod hdr;
pub mod image;
pub mod ktx2;
pub mod obj;
pub mod stl;

pub use asset::{
    ASSET_FORMATS, AssetDependency, AssetDependencyGraph, AssetDiagnostic, AssetDiagnosticSeverity,
    AssetFormatInfo, AssetFormatSupport, AssetLoadHandle, AssetLoadStatus, AssetPackage,
    AssetRequest, AssetSource, LoadedAnimationChannel, LoadedAnimationClip,
    LoadedAnimationInterpolation, LoadedAnimationProperty, LoadedMaterial,
    LoadedMeshSkinAttributes, LoadedSkin, MaterialVariant, SharedAssetPackage, TextureTransform,
    support_for_extension,
};
pub use asset_cache::AssetCache;
pub use asset_manager::AssetManager;
pub use gltf::{GltfAsset, GltfLoader, LoadedCamera, LoadedLight, LoaderOptions};
