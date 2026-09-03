//! scenekit 的浏览器/WASM 集成辅助函数。

mod input;

pub use input::{
    gamepad_axis_from_standard, gamepad_button_from_standard, key_code_from_dom,
    pointer_button_from_dom, touch_phase_from_dom,
};

/// 设备像素比感知的浏览器画布测量。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CanvasMetrics {
    /// CSS 逻辑像素宽度。
    pub logical_width: u32,
    /// CSS 逻辑像素高度。
    pub logical_height: u32,
    /// 后备缓冲区物理像素宽度。
    pub physical_width: u32,
    /// 后备缓冲区物理像素高度。
    pub physical_height: u32,
    /// 每逻辑像素的物理像素数。
    pub device_pixel_ratio: f32,
}

impl CanvasMetrics {
    /// 从 CSS 尺寸和 DPR 创建规范化的测量值。
    pub fn new(logical_width: u32, logical_height: u32, device_pixel_ratio: f32) -> Self {
        let (logical_width, logical_height) = clamp_canvas_size(logical_width, logical_height);
        let device_pixel_ratio = if device_pixel_ratio.is_finite() && device_pixel_ratio > 0.0 {
            device_pixel_ratio
        } else {
            1.0
        };
        Self {
            logical_width,
            logical_height,
            physical_width: (logical_width as f32 * device_pixel_ratio).round().max(1.0) as u32,
            physical_height: (logical_height as f32 * device_pixel_ratio)
                .round()
                .max(1.0) as u32,
            device_pixel_ratio,
        }
    }
}

/// 安装将 Rust panic 转发到浏览器控制台的 panic 钩子。
#[inline]
pub fn set_panic_hook() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}

/// 将画布/渲染目标大小钳位到渲染器有效尺寸。
#[inline]
pub const fn clamp_canvas_size(width: u32, height: u32) -> (u32, u32) {
    (
        if width == 0 { 1 } else { width },
        if height == 0 { 1 } else { height },
    )
}

/// 浏览器回退渲染器使用的 WebGL 能力级别。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WebGlCapabilityLevel {
    /// WebGL 1 精简回退路径。
    WebGl1,
    /// WebGL 2 完整浏览器回退路径，用于生成的渲染器场景。
    WebGl2,
}

impl WebGlCapabilityLevel {
    /// 返回诊断中使用的紧凑标签。
    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::WebGl1 => "webgl1",
            Self::WebGl2 => "webgl2",
        }
    }

    /// 返回此浏览器回退的渲染器对等级别。
    #[inline]
    pub const fn parity_label(self) -> &'static str {
        match self {
            Self::WebGl1 => "reduced-fallback",
            Self::WebGl2 => "full-fallback",
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
pub use web::{
    BrowserBackendKind, BrowserBackendPreference, BrowserRenderer, WebGlRenderer, WebRenderer,
    canvas_metrics, canvas_size,
};

#[cfg(not(target_arch = "wasm32"))]
/// 浏览器渲染器包装器。
///
/// 具体实现可在编译为
/// `wasm32-unknown-unknown` 时使用。
#[derive(Debug)]
pub struct WebRenderer;

#[cfg(not(target_arch = "wasm32"))]
/// 具有自动 WebGPU/WebGL 后端选择的浏览器渲染器。
#[derive(Debug)]
pub struct BrowserRenderer;

#[cfg(not(target_arch = "wasm32"))]
/// 浏览器 WebGL 回退渲染器。
#[derive(Debug)]
pub struct WebGlRenderer;

#[cfg(not(target_arch = "wasm32"))]
/// 首选浏览器渲染后端。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BrowserBackendPreference {
    /// 选择最佳可用浏览器后端。
    Auto,
    /// 强制 WebGPU。
    WebGpu,
    /// 强制 WebGL。
    WebGl,
}

#[cfg(not(target_arch = "wasm32"))]
/// 活动浏览器渲染后端。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BrowserBackendKind {
    /// WebGPU 后端。
    WebGpu,
    /// WebGL 后端。
    WebGl,
    /// 应用级 Canvas2D 回退。
    CanvasFallback,
}
