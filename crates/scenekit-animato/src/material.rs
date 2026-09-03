use alloc::collections::BTreeMap;

use scenekit_core::{Color, MaterialId, ValidationError};
use scenekit_material::{AlphaMode, PbrMaterial};

use crate::{ColorTrack, ScalarTrack, Vec3Track};

/// 材质动画器使用的可变 PBR 材质查找。
pub trait PbrMaterialStoreMut {
    /// 返回 `id` 的可变 PBR 材质。
    fn pbr_material_mut(&mut self, id: MaterialId) -> Option<&mut PbrMaterial>;
}

impl PbrMaterialStoreMut for BTreeMap<MaterialId, PbrMaterial> {
    #[inline]
    fn pbr_material_mut(&mut self, id: MaterialId) -> Option<&mut PbrMaterial> {
        self.get_mut(&id)
    }
}

/// 可被动画化的 PBR 材质字段。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MaterialAnimationTarget {
    /// 基础颜色（反照率）。
    Albedo(ColorTrack),
    /// 基础颜色 Alpha 通道。
    Opacity(ScalarTrack),
    /// 自发光 RGB 颜色。
    Emissive(Vec3Track),
    /// 粗糙度系数。
    Roughness(ScalarTrack),
    /// 金属度系数。
    Metallic(ScalarTrack),
}

impl MaterialAnimationTarget {
    /// 推进包含的轨道。
    pub fn update(&mut self, dt: f32) -> bool {
        match self {
            Self::Albedo(track) => track.update(dt),
            Self::Opacity(track) | Self::Roughness(track) | Self::Metallic(track) => {
                track.update(dt)
            }
            Self::Emissive(track) => track.update(dt),
        }
    }

    /// 返回包含的轨道是否已完成。
    pub fn is_complete(&self) -> bool {
        match self {
            Self::Albedo(track) => track.is_complete(),
            Self::Opacity(track) | Self::Roughness(track) | Self::Metallic(track) => {
                track.is_complete()
            }
            Self::Emissive(track) => track.is_complete(),
        }
    }
}

/// 将动画轨道应用到 PBR 材质。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MaterialAnimator {
    /// 目标材质 ID。
    pub material_id: MaterialId,
    /// 被动画化的字段。
    pub target: MaterialAnimationTarget,
}

impl MaterialAnimator {
    /// 创建材质动画器。
    #[inline]
    pub const fn new(material_id: MaterialId, target: MaterialAnimationTarget) -> Self {
        Self {
            material_id,
            target,
        }
    }

    /// 推进动画器，应用当前值，并返回完成状态。
    pub fn update(
        &mut self,
        dt: f32,
        materials: &mut impl PbrMaterialStoreMut,
    ) -> Result<bool, ValidationError> {
        self.target.update(dt);
        let material = materials
            .pbr_material_mut(self.material_id)
            .ok_or(ValidationError::InvalidId)?;

        match &self.target {
            MaterialAnimationTarget::Albedo(track) => {
                material.albedo = track.value();
            }
            MaterialAnimationTarget::Opacity(track) => {
                let opacity = track.value().clamp(0.0, 1.0);
                material.albedo = Color::rgba(
                    material.albedo.r,
                    material.albedo.g,
                    material.albedo.b,
                    opacity,
                );
                if opacity < 1.0 {
                    material.alpha_mode = AlphaMode::Blend;
                } else if material.alpha_mode == AlphaMode::Blend {
                    material.alpha_mode = AlphaMode::Opaque;
                }
            }
            MaterialAnimationTarget::Emissive(track) => {
                material.emissive = track.value();
            }
            MaterialAnimationTarget::Roughness(track) => {
                material.roughness = track.value().clamp(0.0, 1.0);
            }
            MaterialAnimationTarget::Metallic(track) => {
                material.metallic = track.value().clamp(0.0, 1.0);
            }
        }

        Ok(self.target.is_complete())
    }
}
