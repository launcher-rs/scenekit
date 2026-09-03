use scenekit_core::Color;
use scenekit_material::{
    LambertMaterial, Material, NormalMaterial, PbrMaterial, PhysicalMaterial, ToonMaterial,
    UnlitMaterial, WireframeMaterial,
};
use scenekit_math::Vec3;

use crate::TextureStore;

/// 内置 v0.6 材质路径共享的 GPU 就绪 uniform。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialUniform {
    /// 线性 RGBA 中的基础颜色或漫反射颜色。
    pub base_color: [f32; 4],
    /// 自发光颜色和 alpha 截断值。
    pub emissive_cutoff: [f32; 4],
    /// 金属度、粗糙度、着色器类型和标志位。
    pub params: [f32; 4],
}

impl MaterialUniform {
    /// 从通用参数创建材质 uniform。
    #[inline]
    pub fn new(
        base_color: Color,
        emissive: Vec3,
        metallic: f32,
        roughness: f32,
        alpha_cutoff: Option<f32>,
        shader_kind: f32,
        feature_bits: u64,
    ) -> Self {
        Self {
            base_color: base_color.to_array(),
            emissive_cutoff: [
                emissive.x,
                emissive.y,
                emissive.z,
                alpha_cutoff.unwrap_or(-1.0),
            ],
            params: [
                metallic.clamp(0.0, 1.0),
                roughness.clamp(0.0, 1.0),
                shader_kind,
                feature_bits as f32,
            ],
        }
    }
}

/// 将 CPU 端材质描述桥接到渲染器拥有的 GPU 资源。
pub trait GpuMaterial: Material {
    /// 返回此材质族使用的绑定组布局。
    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout
    where
        Self: Sized,
    {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("scenekit.material.empty_layout"),
            entries: &[],
        })
    }

    /// 将材质状态序列化为 uniform 字节。
    fn to_uniform_bytes(&self) -> Vec<u8>;

    /// 为此材质创建绑定组。
    fn create_bind_group(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        _textures: &TextureStore,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scenekit.material.bind_group"),
            layout,
            entries: &[],
        })
    }
}

impl GpuMaterial for PbrMaterial {
    fn to_uniform_bytes(&self) -> Vec<u8> {
        let uniform = MaterialUniform::new(
            self.albedo,
            self.emissive,
            self.metallic,
            self.roughness,
            self.alpha_cutoff(),
            0.0,
            self.pipeline_key().feature_bits,
        );
        bytemuck::bytes_of(&uniform).to_vec()
    }
}

impl GpuMaterial for UnlitMaterial {
    fn to_uniform_bytes(&self) -> Vec<u8> {
        let uniform = MaterialUniform::new(
            self.color,
            Vec3::ZERO,
            0.0,
            1.0,
            self.alpha_cutoff(),
            1.0,
            self.pipeline_key().feature_bits,
        );
        bytemuck::bytes_of(&uniform).to_vec()
    }
}

impl GpuMaterial for LambertMaterial {
    fn to_uniform_bytes(&self) -> Vec<u8> {
        let uniform = MaterialUniform::new(
            self.color,
            self.emissive,
            0.0,
            1.0,
            self.alpha_cutoff(),
            2.0,
            self.pipeline_key().feature_bits,
        );
        bytemuck::bytes_of(&uniform).to_vec()
    }
}

impl GpuMaterial for PhysicalMaterial {
    fn to_uniform_bytes(&self) -> Vec<u8> {
        let uniform = MaterialUniform::new(
            self.base.albedo,
            self.base.emissive,
            self.base.metallic,
            self.base.roughness,
            self.alpha_cutoff(),
            3.0,
            self.pipeline_key().feature_bits,
        );
        bytemuck::bytes_of(&uniform).to_vec()
    }
}

impl GpuMaterial for ToonMaterial {
    fn to_uniform_bytes(&self) -> Vec<u8> {
        let uniform = MaterialUniform::new(
            self.color,
            Vec3::ZERO,
            0.0,
            1.0,
            self.alpha_cutoff(),
            4.0,
            self.pipeline_key().feature_bits,
        );
        bytemuck::bytes_of(&uniform).to_vec()
    }
}

impl GpuMaterial for WireframeMaterial {
    fn to_uniform_bytes(&self) -> Vec<u8> {
        let uniform = MaterialUniform::new(
            Color {
                a: self.opacity,
                ..self.color
            },
            Vec3::ZERO,
            0.0,
            1.0,
            self.alpha_cutoff(),
            5.0,
            self.pipeline_key().feature_bits,
        );
        bytemuck::bytes_of(&uniform).to_vec()
    }
}

impl GpuMaterial for NormalMaterial {
    fn to_uniform_bytes(&self) -> Vec<u8> {
        let uniform = MaterialUniform::new(
            Color::WHITE,
            Vec3::ZERO,
            0.0,
            1.0,
            self.alpha_cutoff(),
            6.0,
            self.pipeline_key().feature_bits,
        );
        bytemuck::bytes_of(&uniform).to_vec()
    }
}
