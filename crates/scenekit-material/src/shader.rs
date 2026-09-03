use alloc::{string::String, vec::Vec};

use scenekit_core::TextureId;

use crate::traits::{double_sided_bit, stable_shader_id};
use crate::{FEATURE_CUSTOM_TEXTURES, Material, PipelineAlphaMode, PipelineKey, ShaderKind};

/// 自定义 WGSL 材质。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShaderMaterial {
    /// 人类可读的材质名称。
    pub name: String,
    /// 顶点着色器 WGSL 源码。
    pub vertex_wgsl: String,
    /// 片段着色器 WGSL 源码。
    pub fragment_wgsl: String,
    /// 应用程序持有的原始 uniform 缓冲区字节。
    pub uniforms: Vec<u8>,
    /// 自定义着色器引用的纹理 ID。
    pub textures: Vec<TextureId>,
    /// 着色器是否需要透明排序和混合。
    pub transparent: bool,
    /// 材质是否为双面渲染。
    pub double_sided: bool,
    /// 可选的 alpha 测试裁剪值。
    pub alpha_cutoff: Option<f32>,
}

impl ShaderMaterial {
    /// 从 WGSL 源码创建着色器材质。
    #[inline]
    pub fn new(vertex_wgsl: impl Into<String>, fragment_wgsl: impl Into<String>) -> Self {
        Self {
            name: String::new(),
            vertex_wgsl: vertex_wgsl.into(),
            fragment_wgsl: fragment_wgsl.into(),
            uniforms: Vec::new(),
            textures: Vec::new(),
            transparent: false,
            double_sided: false,
            alpha_cutoff: None,
        }
    }

    /// 返回 `PipelineKey` 使用的基于源码的稳定着色器 ID。
    #[inline]
    pub fn shader_id(&self) -> u64 {
        stable_shader_id(&self.vertex_wgsl, &self.fragment_wgsl)
    }

    /// 返回启用或禁用透明渲染的此材质。
    #[inline]
    pub const fn transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }
}

impl Default for ShaderMaterial {
    #[inline]
    fn default() -> Self {
        Self::new("", "")
    }
}

impl Material for ShaderMaterial {
    #[inline]
    fn pipeline_key(&self) -> PipelineKey {
        let alpha = if self.transparent {
            PipelineAlphaMode::Blend
        } else if self.alpha_cutoff.is_some() {
            PipelineAlphaMode::Mask
        } else {
            PipelineAlphaMode::Opaque
        };
        let mut bits = double_sided_bit(self.double_sided);
        if !self.textures.is_empty() {
            bits |= FEATURE_CUSTOM_TEXTURES;
        }
        PipelineKey::new(ShaderKind::Custom(self.shader_id()), alpha, bits)
    }

    #[inline]
    fn is_transparent(&self) -> bool {
        self.transparent
    }

    #[inline]
    fn double_sided(&self) -> bool {
        self.double_sided
    }

    #[inline]
    fn alpha_cutoff(&self) -> Option<f32> {
        self.alpha_cutoff
    }
}
