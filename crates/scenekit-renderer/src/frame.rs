use scenekit_math::{Mat4, Vec2, Vec3};

use crate::RenderTargetMode;

/// 每帧在渲染通道前上传的相机和目标数据。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameContext {
    /// 单调递增的帧索引。
    pub frame_index: u64,
    /// 渲染目标分辨率（像素）。
    pub resolution: Vec2,
    /// 相机视图矩阵。
    pub view: Mat4,
    /// 相机投影矩阵。
    pub projection: Mat4,
    /// 投影乘以视图。
    pub view_projection: Mat4,
    /// 相机在世界空间中的位置。
    pub camera_position: Vec3,
}

/// 渲染帧后报告的 CPU 端计数器。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameStats {
    /// 单调递增的帧索引。
    pub frame_index: u64,
    /// 遍历期间遇到的网格节点数。
    pub scene_meshes: u32,
    /// 剔除后提交的网格节点数。
    pub visible_meshes: u32,
    /// 被视锥体剔除拒绝的网格节点数。
    pub culled_meshes: u32,
    /// 不透明绘制提交数。
    pub opaque_draws: u32,
    /// 透明绘制提交数。
    pub transparent_draws: u32,
    /// 本帧可用的已注册光源数。
    pub lights: u32,
    /// 本帧使用的渲染目标后端。
    pub target_mode: Option<RenderTargetMode>,
}

/// 渲染器级别的资源和帧诊断信息。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RendererDiagnostics {
    /// 下次渲染调用将使用的单调递增帧索引。
    pub frame_index: u64,
    /// 已注册的网格数量。
    pub meshes: u32,
    /// 已注册的材质数量。
    pub materials: u32,
    /// 已注册的纹理数量。
    pub textures: u32,
    /// 已注册的光源数量。
    pub lights: u32,
    /// 已注册的渲染目标数量。
    pub render_targets: u32,
    /// 渲染器上传所拥有的近似 GPU 纹理字节数。
    pub texture_memory_bytes: u64,
    /// 近似的 GPU 顶点和索引缓冲区字节数。
    pub geometry_memory_bytes: u64,
    /// 为绘制资源分配的近似 uniform 缓冲区字节数。
    pub uniform_memory_bytes: u64,
    /// 缓存的渲染管线数量。
    pub pipeline_cache_entries: u32,
    /// 渲染器可用的阴影图集层数。
    pub shadow_slots: u32,
}

/// 渲染器拥有资源的内存快照。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResourceStats {
    /// 近似的网格缓冲区字节数。
    pub geometry_bytes: u64,
    /// 近似的纹理和渲染目标字节数。
    pub texture_bytes: u64,
    /// 近似的 uniform 缓冲区字节数。
    pub uniform_bytes: u64,
}

/// 管线缓存计数器。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PipelineCacheStats {
    /// 缓存的渲染管线数量。
    pub entries: u32,
}
