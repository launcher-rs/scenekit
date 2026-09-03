use scenekit_input::{GamepadAxis, InputState, KeyCode, PointerButton};
use scenekit_math::{Quat, Transform, Vec2, Vec3};

use crate::{FlyController, OrthographicCamera, PerspectiveCamera, clamp};

fn input_look_delta(input: &InputState, dt: f32) -> Vec2 {
    let pointer = if input.pointer_lock.locked {
        input.pointer_lock.delta
    } else if input.pointer.is_pressed(PointerButton::Left) {
        input.pointer.delta
    } else {
        Vec2::ZERO
    };
    let touch = if input.gesture.contact_count == 1 {
        input.gesture.pan_delta
    } else {
        Vec2::ZERO
    };
    let gamepad = input
        .gamepads
        .connected()
        .next()
        .map_or(Vec2::ZERO, |(_, pad)| {
            Vec2::new(pad.axis(GamepadAxis::RightX), pad.axis(GamepadAxis::RightY))
        });
    pointer + touch + gamepad * (180.0 * dt.max(0.0))
}

fn zoom_factor(input: &InputState, sensitivity: f32) -> f32 {
    let pinch = if input.gesture.contact_count >= 2 {
        -input.gesture.pinch_delta
    } else {
        0.0
    };
    (1.0 + (input.scroll_delta + pinch) * sensitivity).max(0.01)
}

/// 基于四元数的轨道控制器，适用于模型查看器。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArcballController {
    /// 相机绕其旋转的点。
    pub target: Vec3,
    /// 相机距目标的距离。
    pub distance: f32,
    /// 当前相机朝向。
    pub rotation: Quat,
    /// 最小距离。
    pub min_distance: f32,
    /// 最大距离。
    pub max_distance: f32,
    /// 旋转灵敏度（弧度/逻辑像素）。
    pub rotate_sensitivity: f32,
    /// 缩放响应（每滚轮/捏合单位）。
    pub zoom_sensitivity: f32,
}

impl ArcballController {
    /// 创建一个从正 Z 方向看向 `target` 的弧球控制器。
    pub fn new(target: Vec3, distance: f32) -> Self {
        Self {
            target,
            distance: distance.max(0.001),
            rotation: Quat::IDENTITY,
            min_distance: 0.001,
            max_distance: 1.0e9,
            rotate_sensitivity: 0.005,
            zoom_sensitivity: 0.1,
        }
    }

    /// 从聚合输入快照更新旋转和缩放。
    pub fn update_from_input(&mut self, input: &InputState, dt: f32) -> Transform {
        let delta = input_look_delta(input, dt);
        if delta.length_squared() > crate::EPSILON {
            let yaw = Quat::from_axis_angle(Vec3::Y, -delta.x * self.rotate_sensitivity);
            let right = self.rotation.mul_vec3(Vec3::X).normalize();
            let pitch = Quat::from_axis_angle(right, -delta.y * self.rotate_sensitivity);
            self.rotation = (yaw * pitch * self.rotation).normalize();
        }
        self.distance *= zoom_factor(input, self.zoom_sensitivity);
        self.distance = clamp(
            self.distance,
            self.min_distance.max(0.001),
            self.max_distance.max(self.min_distance.max(0.001)),
        );
        self.camera_transform()
    }

    /// 返回相机世界变换。
    pub fn camera_transform(&self) -> Transform {
        let position = self.target + self.rotation.mul_vec3(Vec3::Z) * self.distance;
        Transform::looking_at(position, self.target, self.rotation.mul_vec3(Vec3::Y))
    }

    /// 将当前姿态应用到透视相机。
    pub fn apply_to_perspective(&self, camera: &mut PerspectiveCamera) {
        let transform = self.camera_transform();
        camera.position = transform.translation;
        camera.target = self.target;
        camera.up = transform.up();
    }
}

impl Default for ArcballController {
    fn default() -> Self {
        Self::new(Vec3::ZERO, 5.0)
    }
}

/// 无约束的轨道、旋转、平移和缩放控制。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TrackballController {
    /// 底层四元数轨道状态。
    pub arcball: ArcballController,
    /// 距离为 1 时每逻辑像素的平移单位。
    pub pan_sensitivity: f32,
    /// 双指旋转的旋转灵敏度。
    pub roll_sensitivity: f32,
}

impl TrackballController {
    /// 创建一个围绕目标的轨迹球控制器。
    pub fn new(target: Vec3, distance: f32) -> Self {
        Self {
            arcball: ArcballController::new(target, distance),
            pan_sensitivity: 0.001,
            roll_sensitivity: 1.0,
        }
    }

    /// 从指针、触摸和游戏手柄输入更新轨迹球。
    pub fn update_from_input(&mut self, input: &InputState, dt: f32) -> Transform {
        if input.gesture.contact_count >= 2 {
            let transform = self.arcball.camera_transform();
            let scale = self.arcball.distance * self.pan_sensitivity;
            self.arcball.target += transform.right() * (-input.gesture.pan_delta.x * scale);
            self.arcball.target += transform.up() * (input.gesture.pan_delta.y * scale);
            if input.gesture.rotation_delta != 0.0 {
                let roll = Quat::from_axis_angle(
                    transform.forward(),
                    input.gesture.rotation_delta * self.roll_sensitivity,
                );
                self.arcball.rotation = (roll * self.arcball.rotation).normalize();
            }
        }
        self.arcball.update_from_input(input, dt)
    }

    /// 返回当前相机变换。
    pub fn camera_transform(&self) -> Transform {
        self.arcball.camera_transform()
    }

    /// 将当前姿态应用到透视相机。
    pub fn apply_to_perspective(&self, camera: &mut PerspectiveCamera) {
        self.arcball.apply_to_perspective(camera);
    }
}

impl Default for TrackballController {
    fn default() -> Self {
        Self::new(Vec3::ZERO, 5.0)
    }
}

/// 世界朝上的地图/导航控制器，支持轨道、平移和缩放。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MapController {
    /// 地平面焦点。
    pub target: Vec3,
    /// 距离目标的距离。
    pub distance: f32,
    /// 绕世界 Y 轴的航向角。
    pub heading: f32,
    /// 向下倾斜角（弧度）。
    pub tilt: f32,
    /// 最小倾斜角。
    pub min_tilt: f32,
    /// 最大倾斜角。
    pub max_tilt: f32,
    /// 最小距离。
    pub min_distance: f32,
    /// 最大距离。
    pub max_distance: f32,
    /// 旋转灵敏度。
    pub rotate_sensitivity: f32,
    /// 平移灵敏度。
    pub pan_sensitivity: f32,
    /// 缩放灵敏度。
    pub zoom_sensitivity: f32,
}

impl MapController {
    /// 创建一个位于目标上方的地图控制器。
    pub fn new(target: Vec3, distance: f32) -> Self {
        Self {
            target,
            distance: distance.max(0.001),
            heading: 0.0,
            tilt: 0.8,
            min_tilt: 0.05,
            max_tilt: core::f32::consts::FRAC_PI_2 - 0.01,
            min_distance: 0.01,
            max_distance: 1.0e9,
            rotate_sensitivity: 0.005,
            pan_sensitivity: 0.001,
            zoom_sensitivity: 0.1,
        }
    }

    /// 更新地图导航。
    pub fn update_from_input(&mut self, input: &InputState, _dt: f32) -> Transform {
        if input.pointer.is_pressed(PointerButton::Left) && input.gesture.contact_count == 0 {
            self.heading -= input.pointer.delta.x * self.rotate_sensitivity;
            self.tilt -= input.pointer.delta.y * self.rotate_sensitivity;
        }
        let pan = if input.gesture.contact_count >= 2
            || input.pointer.is_pressed(PointerButton::Right)
            || input.pointer.is_pressed(PointerButton::Middle)
        {
            if input.gesture.contact_count >= 2 {
                input.gesture.pan_delta
            } else {
                input.pointer.delta
            }
        } else {
            Vec2::ZERO
        };
        if pan.length_squared() > crate::EPSILON {
            let rotation = Quat::from_axis_angle(Vec3::Y, self.heading);
            let right = rotation.mul_vec3(Vec3::X);
            let forward = rotation.mul_vec3(Vec3::NEG_Z);
            let scale = self.distance * self.pan_sensitivity;
            self.target += right * (-pan.x * scale) + forward * (pan.y * scale);
        }
        self.distance *= zoom_factor(input, self.zoom_sensitivity);
        self.tilt = clamp(self.tilt, self.min_tilt, self.max_tilt);
        self.distance = clamp(
            self.distance,
            self.min_distance.max(0.001),
            self.max_distance,
        );
        self.camera_transform()
    }

    /// 返回地图相机变换。
    pub fn camera_transform(&self) -> Transform {
        let rotation = Quat::from_axis_angle(Vec3::Y, self.heading)
            * Quat::from_axis_angle(Vec3::X, -self.tilt);
        let position = self.target + rotation.mul_vec3(Vec3::Z) * self.distance;
        Transform::looking_at(position, self.target, Vec3::Y)
    }

    /// 将姿态应用到透视相机。
    pub fn apply_to_perspective(&self, camera: &mut PerspectiveCamera) {
        camera.position = self.camera_transform().translation;
        camera.target = self.target;
        camera.up = Vec3::Y;
    }

    /// 将姿态和缩放应用到正交相机。
    pub fn apply_to_orthographic(&self, camera: &mut OrthographicCamera) {
        let half_height = self.distance.max(0.001);
        let aspect = ((camera.right - camera.left) / (camera.top - camera.bottom).max(0.001)).abs();
        camera.left = -half_height * aspect;
        camera.right = half_height * aspect;
        camera.bottom = -half_height;
        camera.top = half_height;
        camera.position = self.camera_transform().translation;
        camera.target = self.target;
        camera.up = Vec3::Y;
    }
}

impl Default for MapController {
    fn default() -> Self {
        Self::new(Vec3::ZERO, 10.0)
    }
}

/// 地平面第一人称控制器，可选垂直移动。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FirstPersonController {
    /// 相机位置。
    pub position: Vec3,
    /// 绕世界 Y 轴的偏航角。
    pub yaw: f32,
    /// 绕局部 X 轴的俯仰角。
    pub pitch: f32,
    /// 移动速度（单位/秒）。
    pub speed: f32,
    /// 观察灵敏度（弧度/逻辑像素）。
    pub sensitivity: f32,
    /// 俯仰角绝对限制。
    pub pitch_limit: f32,
    /// 是否允许 Q/E 和扳机键进行垂直移动。
    pub allow_vertical: bool,
}

impl FirstPersonController {
    /// 创建一个位于世界位置的控制器。
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            yaw: 0.0,
            pitch: 0.0,
            speed: 5.0,
            sensitivity: 0.003,
            pitch_limit: core::f32::consts::FRAC_PI_2 - 0.001,
            allow_vertical: false,
        }
    }

    /// 从聚合输入更新观察和移动。
    pub fn update_from_input(&mut self, input: &InputState, dt: f32) -> Transform {
        let delta = input_look_delta(input, dt);
        self.yaw -= delta.x * self.sensitivity;
        self.pitch = clamp(
            self.pitch - delta.y * self.sensitivity,
            -self.pitch_limit,
            self.pitch_limit,
        );
        let rotation = self.rotation();
        let full_forward = rotation.mul_vec3(Vec3::NEG_Z);
        let forward = Vec3::new(full_forward.x, 0.0, full_forward.z).normalize();
        let right = Vec3::new(forward.z, 0.0, -forward.x);
        let mut movement = Vec3::ZERO;
        if input.keyboard.is_pressed(KeyCode::KeyW) || input.keyboard.is_pressed(KeyCode::ArrowUp) {
            movement += forward;
        }
        if input.keyboard.is_pressed(KeyCode::KeyS) || input.keyboard.is_pressed(KeyCode::ArrowDown)
        {
            movement -= forward;
        }
        if input.keyboard.is_pressed(KeyCode::KeyD)
            || input.keyboard.is_pressed(KeyCode::ArrowRight)
        {
            movement += right;
        }
        if input.keyboard.is_pressed(KeyCode::KeyA) || input.keyboard.is_pressed(KeyCode::ArrowLeft)
        {
            movement -= right;
        }
        if self.allow_vertical {
            if input.keyboard.is_pressed(KeyCode::KeyE) || input.keyboard.is_pressed(KeyCode::Space)
            {
                movement += Vec3::Y;
            }
            if input.keyboard.is_pressed(KeyCode::KeyQ) {
                movement -= Vec3::Y;
            }
        }
        if let Some((_, pad)) = input.gamepads.connected().next() {
            movement +=
                right * pad.axis(GamepadAxis::LeftX) - forward * pad.axis(GamepadAxis::LeftY);
        }
        if movement.length_squared() > crate::EPSILON {
            self.position += movement.normalize() * self.speed * dt.max(0.0);
        }
        self.camera_transform()
    }

    /// 返回当前朝向。
    pub fn rotation(&self) -> Quat {
        Quat::from_axis_angle(Vec3::Y, self.yaw) * Quat::from_axis_angle(Vec3::X, self.pitch)
    }

    /// 返回当前相机变换。
    pub fn camera_transform(&self) -> Transform {
        Transform::new(self.position, self.rotation(), Vec3::ONE)
    }

    /// 将姿态应用到透视相机。
    pub fn apply_to_perspective(&self, camera: &mut PerspectiveCamera) {
        let rotation = self.rotation();
        camera.position = self.position;
        camera.target = self.position + rotation.mul_vec3(Vec3::NEG_Z);
        camera.up = rotation.mul_vec3(Vec3::Y);
    }
}

impl Default for FirstPersonController {
    fn default() -> Self {
        Self::new(Vec3::ZERO)
    }
}

/// 指针锁定控制器适配器，仅在锁定时响应相对观察输入。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointerLockController {
    /// 复用的飞行移动和相机状态。
    pub fly: FlyController,
    /// 为 true 时，未锁定的调用保留姿态但不消费输入。
    pub require_lock: bool,
}

impl PointerLockController {
    /// 创建一个位于指定位置的指针锁定控制器。
    pub fn new(position: Vec3) -> Self {
        Self {
            fly: FlyController::new(position),
            require_lock: true,
        }
    }

    /// 仅在满足指针锁定要求时更新。
    pub fn update_from_input(&mut self, input: &InputState, dt: f32) -> Transform {
        if self.require_lock && !input.pointer_lock.locked {
            return self.camera_transform();
        }
        self.fly.update_from_input(input, dt)
    }

    /// 返回当前相机变换。
    pub fn camera_transform(&self) -> Transform {
        Transform::new(self.fly.position, self.fly.rotation(), Vec3::ONE)
    }

    /// 将姿态应用到透视相机。
    pub fn apply_to_perspective(&self, camera: &mut PerspectiveCamera) {
        self.fly.apply_to_perspective(camera);
    }
}

impl Default for PointerLockController {
    fn default() -> Self {
        Self::new(Vec3::ZERO)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scenekit_input::{GamepadId, ViewportMetrics};

    #[test]
    fn arcball_zoom_is_clamped_and_finite() {
        let mut input = InputState::new(ViewportMetrics::default());
        input.on_scroll(-1000.0);
        let mut control = ArcballController::default();
        control.update_from_input(&input, 1.0 / 60.0);
        assert!(control.distance >= control.min_distance);
        assert!(control.camera_transform().translation.x.is_finite());
    }

    #[test]
    fn first_person_uses_keyboard_and_gamepad() {
        let mut input = InputState::default();
        input.on_key_down(KeyCode::KeyW);
        input.set_gamepad_connected(GamepadId(0), true);
        input.set_gamepad_axis(GamepadId(0), GamepadAxis::LeftX, 1.0);
        let mut control = FirstPersonController::default();
        control.update_from_input(&input, 1.0);
        assert!(control.position.length_squared() > 1.0);
    }

    #[test]
    fn pointer_lock_requirement_gates_motion() {
        let mut input = InputState::default();
        input.on_key_down(KeyCode::KeyW);
        let mut control = PointerLockController::default();
        control.update_from_input(&input, 1.0);
        assert_eq!(control.fly.position, Vec3::ZERO);
        input.set_pointer_locked(true);
        control.update_from_input(&input, 1.0);
        assert_ne!(control.fly.position, Vec3::ZERO);
    }
}
