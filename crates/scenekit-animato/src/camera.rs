use alloc::collections::BTreeMap;

use scenekit_camera::{OrthographicCamera, PerspectiveCamera};
use scenekit_core::{CameraId, ValidationError};
use scenekit_math::Vec3;

use crate::{ScalarTrack, Vec3Track};

/// 相机动画器使用的可变相机查找。
pub trait CameraStoreMut {
    /// 返回可变的透视相机（当存储包含一个时）。
    fn perspective_mut(&mut self, _id: CameraId) -> Option<&mut PerspectiveCamera> {
        None
    }

    /// 返回可变的正交相机（当存储包含一个时）。
    fn orthographic_mut(&mut self, _id: CameraId) -> Option<&mut OrthographicCamera> {
        None
    }
}

impl CameraStoreMut for BTreeMap<CameraId, PerspectiveCamera> {
    #[inline]
    fn perspective_mut(&mut self, id: CameraId) -> Option<&mut PerspectiveCamera> {
        self.get_mut(&id)
    }
}

impl CameraStoreMut for BTreeMap<CameraId, OrthographicCamera> {
    #[inline]
    fn orthographic_mut(&mut self, id: CameraId) -> Option<&mut OrthographicCamera> {
        self.get_mut(&id)
    }
}

/// 借用的透视和正交相机映射。
pub struct CameraStores<'a> {
    /// 按 ID 索引的透视相机。
    pub perspective: &'a mut BTreeMap<CameraId, PerspectiveCamera>,
    /// 按 ID 索引的正交相机。
    pub orthographic: &'a mut BTreeMap<CameraId, OrthographicCamera>,
}

impl CameraStoreMut for CameraStores<'_> {
    #[inline]
    fn perspective_mut(&mut self, id: CameraId) -> Option<&mut PerspectiveCamera> {
        self.perspective.get_mut(&id)
    }

    #[inline]
    fn orthographic_mut(&mut self, id: CameraId) -> Option<&mut OrthographicCamera> {
        self.orthographic.get_mut(&id)
    }
}

/// 正交投影包围盒。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrthographicBounds {
    /// 左投影边界。
    pub left: f32,
    /// 右投影边界。
    pub right: f32,
    /// 下投影边界。
    pub bottom: f32,
    /// 上投影边界。
    pub top: f32,
}

impl OrthographicBounds {
    /// 从相机平面创建包围盒。
    #[inline]
    pub const fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        Self {
            left,
            right,
            bottom,
            top,
        }
    }
}

/// 正交投影包围盒的四个标量轨道。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrthographicBoundsTrack {
    /// 左边界轨道。
    pub left: ScalarTrack,
    /// 右边界轨道。
    pub right: ScalarTrack,
    /// 下边界轨道。
    pub bottom: ScalarTrack,
    /// 上边界轨道。
    pub top: ScalarTrack,
}

impl OrthographicBoundsTrack {
    /// 创建线性包围盒补间。
    pub fn tween(start: OrthographicBounds, end: OrthographicBounds, duration: f32) -> Self {
        Self {
            left: ScalarTrack::tween(start.left, end.left, duration),
            right: ScalarTrack::tween(start.right, end.right, duration),
            bottom: ScalarTrack::tween(start.bottom, end.bottom, duration),
            top: ScalarTrack::tween(start.top, end.top, duration),
        }
    }

    /// 推进所有轨道并返回是否有任何轨道仍在运行。
    #[inline]
    pub fn update(&mut self, dt: f32) -> bool {
        self.left.update(dt) | self.right.update(dt) | self.bottom.update(dt) | self.top.update(dt)
    }

    /// 返回当前包围盒。
    #[inline]
    pub fn value(&self) -> OrthographicBounds {
        OrthographicBounds::new(
            self.left.value(),
            self.right.value(),
            self.bottom.value(),
            self.top.value(),
        )
    }

    /// 返回所有轨道是否已完成。
    #[inline]
    pub fn is_complete(&self) -> bool {
        self.left.is_complete()
            && self.right.is_complete()
            && self.bottom.is_complete()
            && self.top.is_complete()
    }
}

/// 可被动画化的相机字段。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CameraAnimationTarget {
    /// 透视垂直视野角，单位为弧度。
    FovY(ScalarTrack),
    /// 相机位置。
    Position(Vec3Track),
    /// 相机观察目标。
    Target(Vec3Track),
    /// 相机上方向向量。
    Up(Vec3Track),
    /// 正交投影包围盒。
    OrthographicBounds(OrthographicBoundsTrack),
}

impl CameraAnimationTarget {
    /// 推进包含的轨道。
    pub fn update(&mut self, dt: f32) -> bool {
        match self {
            Self::FovY(track) => track.update(dt),
            Self::Position(track) | Self::Target(track) | Self::Up(track) => track.update(dt),
            Self::OrthographicBounds(track) => track.update(dt),
        }
    }

    /// 返回包含的轨道是否已完成。
    pub fn is_complete(&self) -> bool {
        match self {
            Self::FovY(track) => track.is_complete(),
            Self::Position(track) | Self::Target(track) | Self::Up(track) => track.is_complete(),
            Self::OrthographicBounds(track) => track.is_complete(),
        }
    }
}

/// 将动画轨道应用到相机存储条目。
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CameraAnimator {
    /// 目标相机 ID。
    pub camera_id: CameraId,
    /// 被动画化的字段。
    pub target: CameraAnimationTarget,
}

impl CameraAnimator {
    /// 创建相机动画器。
    #[inline]
    pub const fn new(camera_id: CameraId, target: CameraAnimationTarget) -> Self {
        Self { camera_id, target }
    }

    /// 推进动画器，应用当前值，并返回完成状态。
    pub fn update(
        &mut self,
        dt: f32,
        cameras: &mut impl CameraStoreMut,
    ) -> Result<bool, ValidationError> {
        self.target.update(dt);
        match &self.target {
            CameraAnimationTarget::FovY(track) => {
                let camera = cameras
                    .perspective_mut(self.camera_id)
                    .ok_or(ValidationError::InvalidId)?;
                camera.fov_y = track.value().clamp(
                    core::f32::consts::PI / 180.0,
                    179.0 * core::f32::consts::PI / 180.0,
                );
            }
            CameraAnimationTarget::Position(track) => {
                apply_position(cameras, self.camera_id, track.value())?;
            }
            CameraAnimationTarget::Target(track) => {
                apply_target(cameras, self.camera_id, track.value())?;
            }
            CameraAnimationTarget::Up(track) => {
                let up = track.value().normalize();
                let up = if up == Vec3::ZERO { Vec3::Y } else { up };
                apply_up(cameras, self.camera_id, up)?;
            }
            CameraAnimationTarget::OrthographicBounds(track) => {
                let camera = cameras
                    .orthographic_mut(self.camera_id)
                    .ok_or(ValidationError::InvalidId)?;
                let bounds = track.value();
                camera.left = bounds.left;
                camera.right = bounds.right;
                camera.bottom = bounds.bottom;
                camera.top = bounds.top;
            }
        }
        Ok(self.target.is_complete())
    }
}

fn apply_position(
    cameras: &mut impl CameraStoreMut,
    id: CameraId,
    value: Vec3,
) -> Result<(), ValidationError> {
    if let Some(camera) = cameras.perspective_mut(id) {
        camera.position = value;
        return Ok(());
    }
    if let Some(camera) = cameras.orthographic_mut(id) {
        camera.position = value;
        return Ok(());
    }
    Err(ValidationError::InvalidId)
}

fn apply_target(
    cameras: &mut impl CameraStoreMut,
    id: CameraId,
    value: Vec3,
) -> Result<(), ValidationError> {
    if let Some(camera) = cameras.perspective_mut(id) {
        camera.target = value;
        return Ok(());
    }
    if let Some(camera) = cameras.orthographic_mut(id) {
        camera.target = value;
        return Ok(());
    }
    Err(ValidationError::InvalidId)
}

fn apply_up(
    cameras: &mut impl CameraStoreMut,
    id: CameraId,
    value: Vec3,
) -> Result<(), ValidationError> {
    if let Some(camera) = cameras.perspective_mut(id) {
        camera.up = value;
        return Ok(());
    }
    if let Some(camera) = cameras.orthographic_mut(id) {
        camera.up = value;
        return Ok(());
    }
    Err(ValidationError::InvalidId)
}
