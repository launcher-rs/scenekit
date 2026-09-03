use scenekit_input::{
    GamepadAxis, GamepadButton, InputState, KeyCode, KeyboardState, PointerButton, PointerState,
};
use scenekit_math::{Quat, Spherical, Transform, Vec2, Vec3};

use crate::{PerspectiveCamera, clamp};

/// 轨道式相机控制器。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrbitController {
    /// 轨道目标。
    pub target: Vec3,
    /// 距离目标的距离。
    pub distance: f32,
    /// 方位角（弧度）。
    pub theta: f32,
    /// 极角（弧度）。
    pub phi: f32,
    /// 最小距离。
    pub min_distance: f32,
    /// 最大距离。
    pub max_distance: f32,
    /// 最小极角。
    pub min_polar_angle: f32,
    /// 最大极角。
    pub max_polar_angle: f32,
    /// 旋转灵敏度（弧度/像素）。
    pub rotate_sensitivity: f32,
    /// 每滚轮单位的缩放灵敏度。
    pub zoom_sensitivity: f32,
    /// 平移灵敏度（距离为 1 时的世界单位/像素）。
    pub pan_sensitivity: f32,
    /// 为渲染器循环保留的阻尼系数。
    pub damping: f32,
}

/// 飞行式相机控制器。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlyController {
    /// 相机位置。
    pub position: Vec3,
    /// 偏航角（弧度）。
    pub yaw: f32,
    /// 俯仰角（弧度）。
    pub pitch: f32,
    /// 移动速度（单位/秒）。
    pub speed: f32,
    /// 按下任一 Shift 键时的倍率。
    pub fast_multiplier: f32,
    /// 指针观察灵敏度（弧度/像素）。
    pub sensitivity: f32,
    /// 俯仰角绝对限制（弧度）。
    pub pitch_limit: f32,
}

impl OrbitController {
    /// 创建一个看向目标的轨道控制器。
    #[inline]
    pub fn new(target: Vec3, distance: f32) -> Self {
        Self {
            target,
            distance: distance.max(0.001),
            theta: 0.0,
            phi: core::f32::consts::FRAC_PI_2,
            min_distance: 0.001,
            max_distance: 1.0e9,
            min_polar_angle: 0.001,
            max_polar_angle: core::f32::consts::PI - 0.001,
            rotate_sensitivity: 0.005,
            zoom_sensitivity: 0.1,
            pan_sensitivity: 0.001,
            damping: 0.0,
        }
    }

    /// 应用拖拽旋转。
    pub fn on_drag(&mut self, delta: Vec2, _dt: f32) {
        self.theta -= delta.x * self.rotate_sensitivity;
        self.phi -= delta.y * self.rotate_sensitivity;
        self.clamp_state();
    }

    /// 应用滚轮缩放。
    pub fn on_scroll(&mut self, delta: f32, _dt: f32) {
        let scale = 1.0 + delta * self.zoom_sensitivity;
        self.distance *= scale.max(0.001);
        self.clamp_state();
    }

    /// 应用局部相机平面平移。
    pub fn on_pan(&mut self, delta: Vec2, _dt: f32) {
        let transform = self.camera_transform();
        let pan_scale = self.distance * self.pan_sensitivity;
        self.target += transform.right() * (-delta.x * pan_scale);
        self.target += transform.up() * (delta.y * pan_scale);
    }

    /// 消费指针状态。左键拖拽轨道旋转，右键/中键拖拽平移。
    pub fn update_from_pointer(&mut self, pointer: PointerState, scroll_delta: f32, dt: f32) {
        if pointer.is_pressed(PointerButton::Left) {
            self.on_drag(pointer.delta, dt);
        } else if pointer.is_pressed(PointerButton::Right)
            || pointer.is_pressed(PointerButton::Middle)
        {
            self.on_pan(pointer.delta, dt);
        }
        if scroll_delta != 0.0 {
            self.on_scroll(scroll_delta, dt);
        }
    }

    /// 消费聚合输入快照，包括触摸和游戏手柄数据。
    pub fn update_from_input(&mut self, input: &InputState, dt: f32) {
        let response = 1.0 / (1.0 + self.damping.max(0.0) * dt.max(0.0));
        self.update_from_pointer(input.pointer, input.scroll_delta, dt);
        match input.gesture.contact_count {
            1 => self.on_drag(input.gesture.pan_delta * response, dt),
            2.. => {
                self.on_pan(input.gesture.pan_delta * response, dt);
                if input.gesture.pinch_delta != 0.0 {
                    self.on_scroll(-input.gesture.pinch_delta, dt);
                }
            }
            _ => {}
        }
        if let Some((_, gamepad)) = input.gamepads.connected().next() {
            let look = Vec2::new(
                gamepad.axis(GamepadAxis::RightX),
                gamepad.axis(GamepadAxis::RightY),
            );
            if look.length_squared() > crate::EPSILON {
                self.on_drag(look * (200.0 * dt.max(0.0) * response), dt);
            }
            let zoom = gamepad.button_value(GamepadButton::RightTrigger)
                - gamepad.button_value(GamepadButton::LeftTrigger);
            if zoom.abs() > crate::EPSILON {
                self.on_scroll(-zoom * dt.max(0.0) * 5.0, dt);
            }
        }
    }

    /// 限制距离和极角。
    #[inline]
    pub fn update(&mut self, _dt: f32) {
        self.clamp_state();
    }

    /// 返回相机变换。
    pub fn camera_transform(&self) -> Transform {
        let offset = Spherical::new(self.distance, self.phi, self.theta).to_vec3();
        Transform::looking_at(self.target + offset, self.target, Vec3::Y)
    }

    /// 将控制器姿态应用到透视相机。
    pub fn apply_to_perspective(&self, camera: &mut PerspectiveCamera) {
        let transform = self.camera_transform();
        camera.position = transform.translation;
        camera.target = self.target;
        camera.up = Vec3::Y;
    }

    fn clamp_state(&mut self) {
        self.min_distance = self.min_distance.max(0.001);
        self.max_distance = self.max_distance.max(self.min_distance);
        self.distance = clamp(self.distance, self.min_distance, self.max_distance);
        self.min_polar_angle = clamp(self.min_polar_angle, 0.0, core::f32::consts::PI);
        self.max_polar_angle = clamp(
            self.max_polar_angle,
            self.min_polar_angle,
            core::f32::consts::PI,
        );
        self.phi = clamp(self.phi, self.min_polar_angle, self.max_polar_angle);
    }
}

impl Default for OrbitController {
    #[inline]
    fn default() -> Self {
        Self::new(Vec3::ZERO, 5.0)
    }
}

impl FlyController {
    /// 创建一个位于指定位置的飞行控制器。
    #[inline]
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            yaw: 0.0,
            pitch: 0.0,
            speed: 5.0,
            fast_multiplier: 4.0,
            sensitivity: 0.003,
            pitch_limit: core::f32::consts::FRAC_PI_2 - 0.001,
        }
    }

    /// 消费键盘和指针状态，然后返回相机变换。
    pub fn update(&mut self, keyboard: KeyboardState, pointer: PointerState, dt: f32) -> Transform {
        self.yaw -= pointer.delta.x * self.sensitivity;
        self.pitch -= pointer.delta.y * self.sensitivity;
        self.pitch = clamp(self.pitch, -self.pitch_limit, self.pitch_limit);

        let rotation = self.rotation();
        let forward = rotation.mul_vec3(Vec3::NEG_Z).normalize();
        let right = rotation.mul_vec3(Vec3::X).normalize();
        let up = Vec3::Y;
        let mut movement = Vec3::ZERO;

        if keyboard.is_pressed(KeyCode::KeyW) || keyboard.is_pressed(KeyCode::ArrowUp) {
            movement += forward;
        }
        if keyboard.is_pressed(KeyCode::KeyS) || keyboard.is_pressed(KeyCode::ArrowDown) {
            movement -= forward;
        }
        if keyboard.is_pressed(KeyCode::KeyD) || keyboard.is_pressed(KeyCode::ArrowRight) {
            movement += right;
        }
        if keyboard.is_pressed(KeyCode::KeyA) || keyboard.is_pressed(KeyCode::ArrowLeft) {
            movement -= right;
        }
        if keyboard.is_pressed(KeyCode::KeyE) || keyboard.is_pressed(KeyCode::Space) {
            movement += up;
        }
        if keyboard.is_pressed(KeyCode::KeyQ) {
            movement -= up;
        }

        if movement.length_squared() > crate::EPSILON {
            let mut speed = self.speed;
            if keyboard.modifiers().shift {
                speed *= self.fast_multiplier.max(1.0);
            }
            self.position += movement.normalize() * speed * dt.max(0.0);
        }

        Transform::new(self.position, rotation, Vec3::ONE)
    }

    /// 消费聚合输入，在指针锁定激活时使用相对运动。
    pub fn update_from_input(&mut self, input: &InputState, dt: f32) -> Transform {
        let mut pointer = input.pointer;
        if input.pointer_lock.locked {
            pointer.delta = input.pointer_lock.delta;
        }
        let mut transform = self.update(input.keyboard, pointer, dt);
        if let Some((_, gamepad)) = input.gamepads.connected().next() {
            self.yaw -= gamepad.axis(GamepadAxis::RightX) * self.sensitivity * 300.0 * dt.max(0.0);
            self.pitch -=
                gamepad.axis(GamepadAxis::RightY) * self.sensitivity * 300.0 * dt.max(0.0);
            self.pitch = clamp(self.pitch, -self.pitch_limit, self.pitch_limit);
            let rotation = self.rotation();
            let forward = rotation.mul_vec3(Vec3::NEG_Z).normalize();
            let right = rotation.mul_vec3(Vec3::X).normalize();
            let movement = right * gamepad.axis(GamepadAxis::LeftX)
                - forward * gamepad.axis(GamepadAxis::LeftY);
            if movement.length_squared() > crate::EPSILON {
                self.position += movement.normalize() * self.speed * dt.max(0.0);
            }
            transform = Transform::new(self.position, rotation, Vec3::ONE);
        }
        transform
    }

    /// 返回当前旋转。
    #[inline]
    pub fn rotation(&self) -> Quat {
        Quat::from_axis_angle(Vec3::Y, self.yaw) * Quat::from_axis_angle(Vec3::X, self.pitch)
    }

    /// 将控制器姿态应用到透视相机。
    pub fn apply_to_perspective(&self, camera: &mut PerspectiveCamera) {
        let rotation = self.rotation();
        let forward = rotation.mul_vec3(Vec3::NEG_Z).normalize();
        camera.position = self.position;
        camera.target = self.position + forward;
        camera.up = rotation.mul_vec3(Vec3::Y).normalize();
    }
}

impl Default for FlyController {
    #[inline]
    fn default() -> Self {
        Self::new(Vec3::ZERO)
    }
}
