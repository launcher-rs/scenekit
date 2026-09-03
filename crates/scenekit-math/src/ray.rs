use crate::{Aabb, EPSILON, Sphere, Vec2, Vec3, sqrt};

/// 归一化的三维射线。
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ray3 {
    /// 射线原点。
    pub origin: Vec3,
    /// 归一化的射线方向。
    pub direction: Vec3,
}

impl Ray3 {
    /// 创建射线并归一化方向。
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// 返回参数距离 `t` 处的点。
    #[inline]
    pub fn at(self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    /// 射线与 AABB 求交，返回最近的非负 `t`。
    pub fn intersect_aabb(self, aabb: Aabb) -> Option<f32> {
        let mut t_min = 0.0_f32;
        let mut t_max = f32::INFINITY;

        for axis in 0..3 {
            let origin = self.origin[axis];
            let direction = self.direction[axis];
            let min = aabb.min[axis];
            let max = aabb.max[axis];

            if direction.abs() <= EPSILON {
                if origin < min || origin > max {
                    return None;
                }
                continue;
            }

            let inv = 1.0 / direction;
            let mut t1 = (min - origin) * inv;
            let mut t2 = (max - origin) * inv;
            if t1 > t2 {
                core::mem::swap(&mut t1, &mut t2);
            }
            t_min = t_min.max(t1);
            t_max = t_max.min(t2);
            if t_min > t_max {
                return None;
            }
        }

        Some(t_min)
    }

    /// 射线与球体求交，返回最近的非负 `t`。
    pub fn intersect_sphere(self, center: Vec3, radius: f32) -> Option<f32> {
        self.intersect_bounding_sphere(Sphere::new(center, radius))
    }

    /// 射线与球体求交，返回最近的非负 `t`。
    pub fn intersect_bounding_sphere(self, sphere: Sphere) -> Option<f32> {
        let oc = self.origin - sphere.center;
        let a = self.direction.dot(self.direction);
        let b = 2.0 * oc.dot(self.direction);
        let c = oc.dot(oc) - sphere.radius * sphere.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 || a.abs() <= EPSILON {
            return None;
        }
        let root = sqrt(discriminant);
        let t0 = (-b - root) / (2.0 * a);
        let t1 = (-b + root) / (2.0 * a);
        if t0 >= 0.0 {
            Some(t0)
        } else if t1 >= 0.0 {
            Some(t1)
        } else {
            None
        }
    }

    /// 使用 Moller-Trumbore 算法与三角形求交。
    ///
    /// 返回 `(t, Vec2::new(u, v))`，其中 `u` 和 `v` 是重心坐标，`w = 1 - u - v`。
    pub fn intersect_triangle(self, a: Vec3, b: Vec3, c: Vec3) -> Option<(f32, Vec2)> {
        let edge1 = b - a;
        let edge2 = c - a;
        let pvec = self.direction.cross(edge2);
        let det = edge1.dot(pvec);
        if det.abs() <= EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let tvec = self.origin - a;
        let u = tvec.dot(pvec) * inv_det;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let qvec = tvec.cross(edge1);
        let v = self.direction.dot(qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = edge2.dot(qvec) * inv_det;
        if t >= 0.0 {
            Some((t, Vec2::new(u, v)))
        } else {
            None
        }
    }
}

impl Default for Ray3 {
    #[inline]
    fn default() -> Self {
        Self::new(Vec3::ZERO, Vec3::NEG_Z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_close;

    #[test]
    fn ray_intersects_aabb() {
        let ray = Ray3::new(Vec3::new(0.0, 0.0, 5.0), Vec3::NEG_Z);
        let aabb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        assert_close(ray.intersect_aabb(aabb).unwrap(), 4.0);
    }

    #[test]
    fn ray_intersects_sphere() {
        let ray = Ray3::new(Vec3::new(0.0, 0.0, 5.0), Vec3::NEG_Z);
        assert_close(ray.intersect_sphere(Vec3::ZERO, 1.0).unwrap(), 4.0);
    }

    #[test]
    fn ray_intersects_triangle_with_barycentric_coordinates() {
        let ray = Ray3::new(Vec3::new(0.25, 0.25, 1.0), Vec3::NEG_Z);
        let (t, uv) = ray
            .intersect_triangle(Vec3::ZERO, Vec3::X, Vec3::Y)
            .unwrap();
        assert_close(t, 1.0);
        assert_close(uv.x, 0.25);
        assert_close(uv.y, 0.25);
    }
}
