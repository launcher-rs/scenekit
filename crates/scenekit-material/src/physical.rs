use scenekit_core::{Color, TextureId};

use crate::{
    FEATURE_CLEARCOAT, FEATURE_IRIDESCENCE, FEATURE_NORMAL_TEXTURE, FEATURE_SHEEN,
    FEATURE_TRANSMISSION, Material, PbrMaterial, PipelineAlphaMode, PipelineKey, ShaderKind,
};

/// 具有高级物理表面效果的 PBR 材质。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhysicalMaterial {
    /// 基础金属度粗糙度材质参数。
    pub base: PbrMaterial,
    /// 清漆层强度，范围 `0.0..=1.0`。
    pub clearcoat: f32,
    /// 清漆层粗糙度，范围 `0.0..=1.0`。
    pub clearcoat_roughness: f32,
    /// 可选的清漆法线纹理。
    pub clearcoat_normal_texture: Option<TextureId>,
    /// 织物般的光泽强度，范围 `0.0..=1.0`。
    pub sheen: f32,
    /// 光泽颜色。
    pub sheen_color: Color,
    /// 光泽粗糙度，范围 `0.0..=1.0`。
    pub sheen_roughness: f32,
    /// 玻璃般的透射强度，范围 `0.0..=1.0`。
    pub transmission: f32,
    /// 用于透射的体积厚度。
    pub thickness: f32,
    /// 折射率。
    pub ior: f32,
    /// 薄膜虹彩强度，范围 `0.0..=1.0`。
    pub iridescence: f32,
    /// 薄膜虹彩折射率。
    pub iridescence_ior: f32,
}

impl PhysicalMaterial {
    /// 创建默认的物理材质。
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 返回设置基础 PBR 参数的此材质。
    #[inline]
    pub fn base(mut self, base: PbrMaterial) -> Self {
        self.base = base;
        self
    }

    /// 返回设置清漆参数的此材质。
    #[inline]
    pub fn clearcoat(mut self, strength: f32, roughness: f32) -> Self {
        self.clearcoat = strength.clamp(0.0, 1.0);
        self.clearcoat_roughness = roughness.clamp(0.0, 1.0);
        self
    }

    /// 返回设置光泽参数的此材质。
    #[inline]
    pub fn sheen(mut self, strength: f32, color: Color, roughness: f32) -> Self {
        self.sheen = strength.clamp(0.0, 1.0);
        self.sheen_color = color;
        self.sheen_roughness = roughness.clamp(0.0, 1.0);
        self
    }

    /// 返回设置透射参数的此材质。
    #[inline]
    pub fn transmission(mut self, strength: f32, thickness: f32) -> Self {
        self.transmission = strength.clamp(0.0, 1.0);
        self.thickness = thickness.max(0.0);
        self
    }

    /// 返回设置虹彩参数的此材质。
    #[inline]
    pub fn iridescence(mut self, strength: f32, ior: f32) -> Self {
        self.iridescence = strength.clamp(0.0, 1.0);
        self.iridescence_ior = ior.max(1.0);
        self
    }

    fn feature_bits(&self) -> u64 {
        let mut bits = self.base.feature_bits();
        if self.clearcoat > 0.0 {
            bits |= FEATURE_CLEARCOAT;
        }
        if self.clearcoat_normal_texture.is_some() {
            bits |= FEATURE_NORMAL_TEXTURE;
        }
        if self.sheen > 0.0 {
            bits |= FEATURE_SHEEN;
        }
        if self.transmission > 0.0 {
            bits |= FEATURE_TRANSMISSION;
        }
        if self.iridescence > 0.0 {
            bits |= FEATURE_IRIDESCENCE;
        }
        bits
    }
}

impl Default for PhysicalMaterial {
    #[inline]
    fn default() -> Self {
        Self {
            base: PbrMaterial::default(),
            clearcoat: 0.0,
            clearcoat_roughness: 0.0,
            clearcoat_normal_texture: None,
            sheen: 0.0,
            sheen_color: Color::WHITE,
            sheen_roughness: 1.0,
            transmission: 0.0,
            thickness: 0.0,
            ior: 1.5,
            iridescence: 0.0,
            iridescence_ior: 1.3,
        }
    }
}

impl Material for PhysicalMaterial {
    #[inline]
    fn pipeline_key(&self) -> PipelineKey {
        let alpha = if self.transmission > 0.0 {
            PipelineAlphaMode::Blend
        } else {
            self.base.alpha_mode.pipeline_alpha()
        };
        PipelineKey::new(ShaderKind::Physical, alpha, self.feature_bits())
    }

    #[inline]
    fn is_transparent(&self) -> bool {
        self.base.alpha_mode.is_transparent() || self.transmission > 0.0
    }

    #[inline]
    fn double_sided(&self) -> bool {
        self.base.double_sided
    }

    #[inline]
    fn alpha_cutoff(&self) -> Option<f32> {
        if self.transmission > 0.0 {
            None
        } else {
            self.base.alpha_mode.cutoff()
        }
    }
}
