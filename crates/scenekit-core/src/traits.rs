use scenekit_math::{Aabb, Vec3};

/// 能提供稳定渲染顺序的值。
pub trait Renderable {
    /// 值越小越先渲染。
    fn render_order(&self) -> u32;
}

/// 能报告保守包围盒的值。
pub trait Bounded {
    /// 返回轴对齐包围盒。
    fn aabb(&self) -> Aabb;

    /// 返回 `(center, radius)` 形式的保守包围球。
    fn bounding_sphere(&self) -> (Vec3, f32);
}

/// 将 CPU 端数据转换为适合 GPU 上传的简单表示。
#[cfg(feature = "gpu")]
pub trait GpuUpload {
    /// 适合 GPU 缓冲区的纯数据表示。
    type GpuData: bytemuck::Pod;

    /// 将值转换为 GPU 数据。
    fn to_gpu(&self) -> Self::GpuData;
}

/// 具有名称的对象。
#[cfg(feature = "std")]
pub trait Named {
    /// 返回当前名称。
    fn name(&self) -> &str;

    /// 设置当前名称。
    fn set_name(&mut self, name: impl Into<String>);
}
