use scenekit_core::Color;

/// 全场景雾效配置。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Fog {
    /// `near` 和 `far` 之间的线性雾。
    Linear {
        /// 雾效开始的距离。
        near: f32,
        /// 雾效达到最大浓度的距离。
        far: f32,
        /// 雾的颜色。
        color: Color,
    },
    /// 由密度控制的指数雾。
    Exponential {
        /// 雾的密度。
        density: f32,
        /// 雾的颜色。
        color: Color,
    },
}

impl Fog {
    /// 创建线性雾。
    #[inline]
    pub const fn linear(near: f32, far: f32, color: Color) -> Self {
        Self::Linear { near, far, color }
    }

    /// 创建指数雾。
    #[inline]
    pub const fn exponential(density: f32, color: Color) -> Self {
        Self::Exponential { density, color }
    }
}
