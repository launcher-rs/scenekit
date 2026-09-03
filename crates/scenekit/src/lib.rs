#![cfg_attr(not(feature = "std"), no_std)]

//! scenekit 渲染器 API 的门面 crate。
//!
//! 此版本重新导出了基础 crate、无 GPU 场景图、
//! CPU 端几何体、材质、灯光、纹理、相机、可选加载器、
//! 射线投射、调试辅助几何体、可选 Animato 和 WASM 集成、
//! 可选后处理以及可选渲染器 API。

extern crate alloc;

pub use scenekit_core::*;
pub use scenekit_input::*;
pub use scenekit_math::*;

#[cfg(feature = "scene")]
pub use scenekit_scene::*;

#[cfg(feature = "camera")]
pub use scenekit_camera::*;

#[cfg(feature = "mesh")]
pub use scenekit_mesh::*;

#[cfg(feature = "material")]
pub use scenekit_material::*;

#[cfg(feature = "light")]
pub use scenekit_light::*;

#[cfg(feature = "texture")]
pub use scenekit_texture::*;

#[cfg(feature = "raycaster")]
pub use scenekit_raycaster::*;

#[cfg(feature = "helpers")]
pub use scenekit_helpers::*;

#[cfg(feature = "animato")]
pub use scenekit_animato::*;

#[cfg(feature = "wasm")]
pub use scenekit_wasm::*;

#[cfg(feature = "loader")]
pub use scenekit_loader::*;

#[cfg(feature = "post")]
pub use scenekit_post::*;

#[cfg(feature = "renderer")]
pub use scenekit_renderer::*;

#[cfg(all(feature = "loader", feature = "renderer"))]
/// 上传资产包到渲染器后返回的计数。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UploadedAssetStats {
    /// 已注册的网格数。
    pub meshes: usize,
    /// 已注册的材质数。
    pub materials: usize,
    /// 已注册的纹理数。
    pub textures: usize,
    /// 已注册的灯光数。
    pub lights: usize,
}

#[cfg(all(feature = "loader", feature = "renderer"))]
/// 从 CPU 资产包到渲染器拥有的资源的便捷上传桥接。
pub trait RendererAssetExt {
    /// 从资产包注册网格、材质、纹理和灯光。
    fn register_asset_package(
        &mut self,
        package: &scenekit_loader::AssetPackage,
    ) -> Result<UploadedAssetStats, scenekit_core::ScenixError>;
}

#[cfg(all(feature = "loader", feature = "renderer"))]
impl RendererAssetExt for scenekit_renderer::Renderer {
    fn register_asset_package(
        &mut self,
        package: &scenekit_loader::AssetPackage,
    ) -> Result<UploadedAssetStats, scenekit_core::ScenixError> {
        let mut stats = UploadedAssetStats::default();

        for (texture_id, texture) in &package.textures {
            let sampler = package
                .samplers
                .get(texture_id)
                .copied()
                .unwrap_or_default();
            self.register_texture2d(*texture_id, texture, sampler)?;
            stats.textures += 1;
        }
        for (texture_id, texture) in &package.texture_cubes {
            let sampler = package
                .samplers
                .get(texture_id)
                .copied()
                .unwrap_or_default();
            self.register_texture_cube(*texture_id, texture, sampler)?;
            stats.textures += 1;
        }

        if package.loaded_materials.is_empty() {
            for (material_id, material) in &package.materials {
                self.register_pbr_material(*material_id, material)?;
                stats.materials += 1;
            }
        } else {
            for (material_id, material) in &package.loaded_materials {
                match material {
                    scenekit_loader::LoadedMaterial::Pbr(material) => {
                        self.register_pbr_material(*material_id, material)?;
                    }
                    scenekit_loader::LoadedMaterial::Physical(material) => {
                        self.register_physical_material(*material_id, material)?;
                    }
                    scenekit_loader::LoadedMaterial::Unlit(material) => {
                        self.register_unlit_material(*material_id, material)?;
                    }
                }
                stats.materials += 1;
            }
        }

        for (mesh_id, geometry) in &package.meshes {
            self.register_mesh(*mesh_id, geometry)?;
            stats.meshes += 1;
        }

        for (light_id, light) in &package.lights {
            match light {
                scenekit_loader::LoadedLight::Directional(light) => {
                    self.register_directional_light(*light_id, *light)?;
                }
                scenekit_loader::LoadedLight::Point(light) => {
                    self.register_point_light(*light_id, *light)?;
                }
                scenekit_loader::LoadedLight::Spot(light) => {
                    self.register_spot_light(*light_id, *light)?;
                }
            }
            stats.lights += 1;
        }

        Ok(stats)
    }
}

/// 将加载器导入的 [`scenekit_loader::LoadedAnimationClip`] 转换为
/// 运行时 [`scenekit_animato::AnimationClip`]，通过 `node_index_to_id` 映射 glTF 节点索引到
/// 场景 [`scenekit_core::NodeId`]。
///
/// 需要同时启用 `loader` 和 `animato` 功能。剪辑通道使用
/// 类型化的 [`scenekit_animato::PropertyBinding`]；未映射的节点索引将被跳过。
/// 三次样条通道在 v1.4 中回退到线性采样。
#[cfg(all(feature = "loader", feature = "animato"))]
pub fn clip_from_loaded(
    loaded: &scenekit_loader::LoadedAnimationClip,
    node_index_to_id: &[scenekit_core::NodeId],
) -> scenekit_animato::AnimationClip {
    use alloc::vec::Vec;
    use scenekit_animato::{
        AnimationClip, ClipChannel, ClipTrack, KeyframeInterpolation, KeyframeQuat, KeyframeScalar,
        KeyframeVec3, NodeProperty, PropertyBinding,
    };
    use scenekit_loader::{LoadedAnimationInterpolation, LoadedAnimationProperty};

    let interp = loaded
        .channels
        .first()
        .map(|c| match c.interpolation {
            LoadedAnimationInterpolation::Linear => KeyframeInterpolation::Linear,
            LoadedAnimationInterpolation::Step => KeyframeInterpolation::Step,
            LoadedAnimationInterpolation::CubicSpline => KeyframeInterpolation::CubicSpline,
        })
        .unwrap_or(KeyframeInterpolation::Linear);

    let mut channels = Vec::new();
    for ch in &loaded.channels {
        let Some(&node_id) = node_index_to_id.get(ch.node_index) else {
            continue;
        };

        // 将打包的 `output` 字节解码为关键帧值。加载器为每个关键帧存储
        // `output_components`（1 标量、3 vec3、4 四元数/颜色）。
        let key_count = ch.times.len();
        let comps = ch.output_components.max(1);

        let track = match ch.property {
            LoadedAnimationProperty::Translation | LoadedAnimationProperty::Scale => {
                let mut values = Vec::with_capacity(key_count);
                for k in 0..key_count {
                    let base = k * comps;
                    let x = ch.output.get(base).copied().unwrap_or(0.0);
                    let y = ch.output.get(base + 1).copied().unwrap_or(0.0);
                    let z = ch.output.get(base + 2).copied().unwrap_or(0.0);
                    values.push(scenekit_math::Vec3::new(x, y, z));
                }
                ClipTrack::Vec3(KeyframeVec3::new(ch.times.clone(), values, interp))
            }
            LoadedAnimationProperty::Rotation => {
                let mut values = Vec::with_capacity(key_count);
                for k in 0..key_count {
                    let base = k * comps;
                    let x = ch.output.get(base).copied().unwrap_or(0.0);
                    let y = ch.output.get(base + 1).copied().unwrap_or(0.0);
                    let z = ch.output.get(base + 2).copied().unwrap_or(0.0);
                    let w = ch.output.get(base + 3).copied().unwrap_or(1.0);
                    values.push(scenekit_math::Quat::new(x, y, z, w));
                }
                ClipTrack::Quat(KeyframeQuat::new(ch.times.clone(), values, interp))
            }
            LoadedAnimationProperty::MorphTargetWeights => {
                let mut values = Vec::with_capacity(key_count);
                for k in 0..key_count {
                    // 每个关键帧的第一个变形目标权重（多目标变形剪辑超出
                    // v1.4 范围；使用第一个权重）。
                    values.push(ch.output.get(k * comps).copied().unwrap_or(0.0));
                }
                ClipTrack::Scalar(KeyframeScalar::new(ch.times.clone(), values, interp))
            }
        };

        let property = match ch.property {
            LoadedAnimationProperty::Translation => NodeProperty::Translation,
            LoadedAnimationProperty::Rotation => NodeProperty::Rotation,
            LoadedAnimationProperty::Scale => NodeProperty::Scale,
            LoadedAnimationProperty::MorphTargetWeights => NodeProperty::Scale,
        };

        channels.push(ClipChannel {
            binding: PropertyBinding::Node { node_id, property },
            track,
        });
    }

    let mut clip = AnimationClip {
        name: loaded.name.clone(),
        duration: loaded.duration,
        channels,
        markers: Vec::new(),
    };
    clip.recompute_duration();
    clip
}
