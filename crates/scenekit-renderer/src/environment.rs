use scenekit_core::{LightId, TextureId};

/// 基于图像的光照环境描述符。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnvironmentMap {
    /// 用作环境源的已注册立方体纹理。
    pub texture_id: TextureId,
    /// 渲染器光照应用的标量强度。
    pub intensity: f32,
    /// 可选的已注册光照探针，用于漫反射辐照度。
    pub light_probe: Option<LightId>,
}

impl EnvironmentMap {
    /// 创建强度为 `1.0` 的环境贴图描述符。
    #[inline]
    pub const fn new(texture_id: TextureId) -> Self {
        Self {
            texture_id,
            intensity: 1.0,
            light_probe: None,
        }
    }

    /// 返回带有强度的描述符副本。
    #[inline]
    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity.max(0.0);
        self
    }

    /// 返回带有光照探针的描述符副本。
    #[inline]
    pub const fn light_probe(mut self, light_probe: LightId) -> Self {
        self.light_probe = Some(light_probe);
        self
    }
}
