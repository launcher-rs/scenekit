use alloc::vec::Vec;

/// 顶点属性的语义含义。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VertexSemantic {
    /// 顶点位置。
    Position,
    /// 顶点法线。
    Normal,
    /// 主 UV 坐标。
    Uv0,
    /// 辅助 UV 坐标。
    Uv1,
    /// 顶点颜色。
    Color,
    /// 带手性的切线。
    Tangent,
    /// 每实例变换矩阵。
    InstanceMatrix,
}

/// 顶点数据格式。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VertexFormat {
    /// 两个 32 位浮点数。
    Float32x2,
    /// 三个 32 位浮点数。
    Float32x3,
    /// 四个 32 位浮点数。
    Float32x4,
    /// 一个 32 位无符号整数。
    Uint32,
}

/// 索引缓冲区整数格式。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IndexFormat {
    /// 16 位无符号整数索引。
    Uint16,
    /// 32 位无符号整数索引。
    Uint32,
}

/// 顶点缓冲区是按顶点还是按实例步进。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BufferStepMode {
    /// 每顶点步进一次。
    Vertex,
    /// 每实例步进一次。
    Instance,
}

/// 交错或紧凑缓冲区中的单个顶点属性。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VertexAttribute {
    /// 属性语义。
    pub semantic: VertexSemantic,
    /// 属性存储格式。
    pub format: VertexFormat,
    /// 从顶点起始位置的字节偏移。
    pub offset: u64,
    /// 渲染器后端的着色器位置。
    pub shader_location: u32,
}

impl VertexAttribute {
    /// 创建顶点属性描述符。
    #[inline]
    pub const fn new(
        semantic: VertexSemantic,
        format: VertexFormat,
        offset: u64,
        shader_location: u32,
    ) -> Self {
        Self {
            semantic,
            format,
            offset,
            shader_location,
        }
    }
}

/// 顶点缓冲区布局元数据。
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BufferLayout {
    /// 元素间的字节步长。
    pub array_stride: u64,
    /// 缓冲区的步进模式。
    pub step_mode: BufferStepMode,
    /// 缓冲区中存储的属性。
    pub attributes: Vec<VertexAttribute>,
}

impl BufferLayout {
    /// 创建缓冲区布局。
    #[inline]
    pub fn new(
        array_stride: u64,
        step_mode: BufferStepMode,
        attributes: Vec<VertexAttribute>,
    ) -> Self {
        Self {
            array_stride,
            step_mode,
            attributes,
        }
    }
}
