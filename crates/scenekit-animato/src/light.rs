//! 过程式驱动器和混合器的灯光动画目标。

use alloc::collections::BTreeMap;

use scenekit_core::{LightId, ValidationError};
use scenekit_light::{DirectionalLight, PointLight, SpotLight};

use crate::{ColorTrack, ScalarTrack};

/// 灯光动画器使用的可变灯光查找。
pub trait LightStoreMut {
    /// 返回可变的点光源（如果存在）。
    fn point_mut(&mut self, _id: LightId) -> Option<&mut PointLight> {
        None
    }
    /// 返回可变的聚光灯（如果存在）。
    fn spot_mut(&mut self, _id: LightId) -> Option<&mut SpotLight> {
        None
    }
    /// 返回可变的方向光（如果存在）。
    fn directional_mut(&mut self, _id: LightId) -> Option<&mut DirectionalLight> {
        None
    }
}

impl LightStoreMut for BTreeMap<LightId, PointLight> {
    #[inline]
    fn point_mut(&mut self, id: LightId) -> Option<&mut PointLight> {
        self.get_mut(&id)
    }
}
impl LightStoreMut for BTreeMap<LightId, SpotLight> {
    #[inline]
    fn spot_mut(&mut self, id: LightId) -> Option<&mut SpotLight> {
        self.get_mut(&id)
    }
}
impl LightStoreMut for BTreeMap<LightId, DirectionalLight> {
    #[inline]
    fn directional_mut(&mut self, id: LightId) -> Option<&mut DirectionalLight> {
        self.get_mut(&id)
    }
}

/// 借用的点光/聚光/方向光映射。
pub struct LightStores<'a> {
    /// 按 ID 索引的点光源。
    pub point: &'a mut BTreeMap<LightId, PointLight>,
    /// 按 ID 索引的聚光灯。
    pub spot: &'a mut BTreeMap<LightId, SpotLight>,
    /// 按 ID 索引的方向光。
    pub directional: &'a mut BTreeMap<LightId, DirectionalLight>,
}

impl LightStoreMut for LightStores<'_> {
    #[inline]
    fn point_mut(&mut self, id: LightId) -> Option<&mut PointLight> {
        self.point.get_mut(&id)
    }
    #[inline]
    fn spot_mut(&mut self, id: LightId) -> Option<&mut SpotLight> {
        self.spot.get_mut(&id)
    }
    #[inline]
    fn directional_mut(&mut self, id: LightId) -> Option<&mut DirectionalLight> {
        self.directional.get_mut(&id)
    }
}

/// 可被动画化的灯光字段。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LightAnimationTarget {
    /// 灯光颜色。
    Color(ColorTrack),
    /// 标量强度。
    Intensity(ScalarTrack),
    /// 最大范围（仅点光/聚光）。
    Range(ScalarTrack),
    /// 聚光灯外锥半角，单位为弧度（仅聚光）。
    SpotAngle(ScalarTrack),
}

impl LightAnimationTarget {
    /// 推进包含的轨道并返回是否仍在运行。
    pub fn update(&mut self, dt: f32) -> bool {
        match self {
            Self::Color(t) => t.update(dt),
            Self::Intensity(t) | Self::Range(t) | Self::SpotAngle(t) => t.update(dt),
        }
    }
    /// 返回包含的轨道是否已完成。
    pub fn is_complete(&self) -> bool {
        match self {
            Self::Color(t) => t.is_complete(),
            Self::Intensity(t) | Self::Range(t) | Self::SpotAngle(t) => t.is_complete(),
        }
    }
}

/// 将过程式轨道应用到灯光存储条目。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LightAnimator {
    /// 目标灯光 ID。
    pub light_id: LightId,
    /// 被动画化的字段。
    pub target: LightAnimationTarget,
}

impl LightAnimator {
    /// 创建灯光动画器。
    #[inline]
    pub const fn new(light_id: LightId, target: LightAnimationTarget) -> Self {
        Self { light_id, target }
    }

    /// 推进、应用并返回完成状态。
    pub fn update(
        &mut self,
        dt: f32,
        lights: &mut impl LightStoreMut,
    ) -> Result<bool, ValidationError> {
        self.target.update(dt);
        match &self.target {
            LightAnimationTarget::Color(track) => {
                let v = track.value();
                if let Some(l) = lights.point_mut(self.light_id) {
                    l.color = v;
                }
                if let Some(l) = lights.spot_mut(self.light_id) {
                    l.color = v;
                }
                if let Some(l) = lights.directional_mut(self.light_id) {
                    l.color = v;
                }
            }
            LightAnimationTarget::Intensity(track) => {
                let v = track.value().max(0.0);
                if let Some(l) = lights.point_mut(self.light_id) {
                    l.intensity = v;
                }
                if let Some(l) = lights.spot_mut(self.light_id) {
                    l.intensity = v;
                }
                if let Some(l) = lights.directional_mut(self.light_id) {
                    l.intensity = v;
                }
            }
            LightAnimationTarget::Range(track) => {
                let v = track.value().max(0.0);
                if let Some(l) = lights.point_mut(self.light_id) {
                    l.range = v;
                }
                if let Some(l) = lights.spot_mut(self.light_id) {
                    l.range = v;
                }
            }
            LightAnimationTarget::SpotAngle(track) => {
                let v = track.value().clamp(0.0, core::f32::consts::FRAC_PI_2);
                if let Some(l) = lights.spot_mut(self.light_id) {
                    l.angle = v;
                }
            }
        }
        Ok(self.target.is_complete())
    }
}
