use scenekit_core::TextureId;

/// 精灵应如何面向相机。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BillboardMode {
    /// 精灵保持其节点旋转。
    #[default]
    None,
    /// 精灵面向活动相机。
    FaceCamera,
    /// 精灵绕世界 Y 轴旋转以面向活动相机。
    AxisAlignedY,
}

/// CPU 端的精灵附件数据。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sprite {
    /// 精灵宽度（局部单位）。
    pub width: f32,
    /// 精灵高度（局部单位）。
    pub height: f32,
    /// 精灵显示的纹理。
    pub texture_id: TextureId,
    /// 公告牌朝向行为。
    pub billboard: BillboardMode,
}

impl Sprite {
    /// 创建一个无公告牌旋转的精灵。
    #[inline]
    pub const fn new(width: f32, height: f32, texture_id: TextureId) -> Self {
        Self {
            width,
            height,
            texture_id,
            billboard: BillboardMode::None,
        }
    }

    /// 返回设置了公告牌模式的精灵。
    #[inline]
    pub const fn billboard(mut self, billboard: BillboardMode) -> Self {
        self.billboard = billboard;
        self
    }
}
