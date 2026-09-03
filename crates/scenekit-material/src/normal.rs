use crate::traits::double_sided_bit;
use crate::{
    AlphaMode, FEATURE_FLAT_SHADING, FEATURE_WORLD_SPACE, Material, PipelineAlphaMode, PipelineKey,
    ShaderKind,
};

/// 将表面法线渲染为 RGB 颜色的调试材质。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalMaterial {
    /// 输出不透明度。
    pub opacity: f32,
    /// 是否使用平面面法线。
    pub flat_shading: bool,
    /// 法线是否在世界空间而非视图空间中显示。
    pub world_space: bool,
    /// 材质是否为双面渲染。
    pub double_sided: bool,
}

impl NormalMaterial {
    /// 创建默认的法线调试材质。
    #[inline]
    pub const fn new() -> Self {
        Self {
            opacity: 1.0,
            flat_shading: false,
            world_space: false,
            double_sided: false,
        }
    }

    /// 返回设置不透明度的此材质。
    #[inline]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
}

impl Default for NormalMaterial {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Material for NormalMaterial {
    #[inline]
    fn pipeline_key(&self) -> PipelineKey {
        let mut bits = double_sided_bit(self.double_sided);
        if self.flat_shading {
            bits |= FEATURE_FLAT_SHADING;
        }
        if self.world_space {
            bits |= FEATURE_WORLD_SPACE;
        }
        let alpha = if self.opacity < 1.0 {
            PipelineAlphaMode::Blend
        } else {
            PipelineAlphaMode::Opaque
        };
        PipelineKey::new(ShaderKind::Normal, alpha, bits)
    }

    #[inline]
    fn is_transparent(&self) -> bool {
        self.opacity < 1.0
    }

    #[inline]
    fn double_sided(&self) -> bool {
        self.double_sided
    }

    #[inline]
    fn alpha_cutoff(&self) -> Option<f32> {
        AlphaMode::Opaque.cutoff()
    }
}
