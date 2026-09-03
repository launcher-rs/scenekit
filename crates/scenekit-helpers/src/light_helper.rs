use scenekit_core::Color;
use scenekit_light::{DirectionalLight, PointLight, SpotLight};
use scenekit_math::{Quat, Vec3};

use crate::arrow::{ArrowHelper, normalize_direction, perpendicular_basis};
use crate::{EPSILON, LineGeometry};

/// 线框点光源辅助工具。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointLightHelper {
    /// 要可视化的点光源。
    pub light: PointLight,
    /// 世界空间中的灯光位置。
    pub position: Vec3,
    /// 辅助工具颜色。
    pub color: Color,
    /// 圆形的线段数量。
    pub segments: u32,
}

impl PointLightHelper {
    /// 创建点光源辅助工具。
    #[inline]
    pub const fn new(light: PointLight, position: Vec3, color: Color) -> Self {
        Self {
            light,
            position,
            color,
            segments: 32,
        }
    }

    /// 生成三个正交的范围圆。
    pub fn to_geometry(&self) -> LineGeometry {
        let radius = if self.light.range > EPSILON {
            self.light.range
        } else {
            1.0
        };
        let mut geometry = LineGeometry::new();
        append_circle(
            &mut geometry,
            self.position,
            Vec3::X,
            Vec3::Y,
            radius,
            self.segments,
            self.color,
        );
        append_circle(
            &mut geometry,
            self.position,
            Vec3::X,
            Vec3::Z,
            radius,
            self.segments,
            self.color,
        );
        append_circle(
            &mut geometry,
            self.position,
            Vec3::Y,
            Vec3::Z,
            radius,
            self.segments,
            self.color,
        );
        geometry
    }
}

/// 线框聚光灯锥体辅助工具。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpotLightHelper {
    /// 要可视化的聚光灯。
    pub light: SpotLight,
    /// 世界空间中的灯光位置。
    pub position: Vec3,
    /// 世界空间中的锥体方向。
    pub direction: Vec3,
    /// 辅助工具颜色。
    pub color: Color,
    /// 圆形的线段数量。
    pub segments: u32,
}

impl SpotLightHelper {
    /// 创建聚光灯辅助工具。
    #[inline]
    pub fn new(light: SpotLight, position: Vec3, direction: Vec3, color: Color) -> Self {
        Self {
            light,
            position,
            direction: normalize_direction(direction),
            color,
            segments: 32,
        }
    }

    /// 生成锥体边缘和外锥圆环。
    pub fn to_geometry(&self) -> LineGeometry {
        let direction = normalize_direction(self.direction);
        let length = if self.light.range > EPSILON {
            self.light.range
        } else {
            1.0
        };
        let radius = length * tan_approx(self.light.angle.clamp(0.0, 1.55));
        let center = self.position + direction * length;
        let (right, up) = perpendicular_basis(direction);
        let mut geometry = LineGeometry::new();
        append_circle(
            &mut geometry,
            center,
            right,
            up,
            radius,
            self.segments,
            self.color,
        );
        geometry.push_segment(self.position, center + right * radius, self.color);
        geometry.push_segment(self.position, center - right * radius, self.color);
        geometry.push_segment(self.position, center + up * radius, self.color);
        geometry.push_segment(self.position, center - up * radius, self.color);
        geometry
    }
}

/// 方向光箭头辅助工具。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirectionalLightHelper {
    /// 要可视化的方向光。
    pub light: DirectionalLight,
    /// 世界空间中的箭头原点。
    pub origin: Vec3,
    /// 箭头长度。
    pub length: f32,
    /// 辅助工具颜色。
    pub color: Color,
}

impl DirectionalLightHelper {
    /// 创建方向光辅助工具。
    #[inline]
    pub const fn new(light: DirectionalLight, origin: Vec3, length: f32, color: Color) -> Self {
        Self {
            light,
            origin,
            length,
            color,
        }
    }

    /// 生成一个沿灯光方向的箭头。
    #[inline]
    pub fn to_geometry(&self) -> LineGeometry {
        ArrowHelper::new(self.origin, self.light.direction, self.length, self.color).to_geometry()
    }
}

pub(crate) fn append_circle(
    geometry: &mut LineGeometry,
    center: Vec3,
    axis_a: Vec3,
    axis_b: Vec3,
    radius: f32,
    segments: u32,
    color: Color,
) {
    let radius = radius.abs().max(EPSILON);
    let segments = segments.max(3);
    let axis_a = axis_a.normalize();
    let axis_b = axis_b.normalize();
    let step = core::f32::consts::TAU / segments as f32;
    let mut previous = center + axis_a * radius;
    for index in 1..=segments {
        let angle = step * index as f32;
        let point = center + rotate_in_plane(axis_a, axis_b, angle) * radius;
        geometry.push_segment(previous, point, color);
        previous = point;
    }
}

fn rotate_in_plane(axis_a: Vec3, axis_b: Vec3, angle: f32) -> Vec3 {
    let rotation = Quat::from_axis_angle(axis_a.cross(axis_b).normalize(), angle);
    rotation.mul_vec3(axis_a).normalize()
}

fn tan_approx(value: f32) -> f32 {
    let x2 = value * value;
    value + value * x2 / 3.0 + 2.0 * value * x2 * x2 / 15.0
}
