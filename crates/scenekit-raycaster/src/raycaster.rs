use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use scenekit_camera::PerspectiveCamera;
use scenekit_core::{MaterialId, MeshId, NodeId, ValidationError};
use scenekit_math::{Mat3, Mat4, Ray3, Vec2};
use scenekit_mesh::Geometry;
use scenekit_scene::{NodeKind, SceneGraph};

use crate::{Bvh, BvhEntry, Intersection};

/// 通过 `MeshId` 提供网格几何数据的 trait。
pub trait GeometryProvider {
    /// 返回 `mesh_id` 对应的几何数据（如果存在）。
    fn geometry(&self, mesh_id: MeshId) -> Option<&Geometry>;
}

impl GeometryProvider for BTreeMap<MeshId, Geometry> {
    #[inline]
    fn geometry(&self, mesh_id: MeshId) -> Option<&Geometry> {
        self.get(&mesh_id)
    }
}

impl GeometryProvider for [(MeshId, Geometry)] {
    fn geometry(&self, mesh_id: MeshId) -> Option<&Geometry> {
        self.iter()
            .find_map(|(id, geometry)| (*id == mesh_id).then_some(geometry))
    }
}

impl<const N: usize> GeometryProvider for [(MeshId, Geometry); N] {
    #[inline]
    fn geometry(&self, mesh_id: MeshId) -> Option<&Geometry> {
        self.as_slice().geometry(mesh_id)
    }
}

/// BVH 加速的场景光线投射器。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Raycaster {
    bvh: Option<Bvh>,
    layers: u32,
}

impl Raycaster {
    /// 创建测试所有图层的光线投射器。
    #[inline]
    pub const fn new() -> Self {
        Self {
            bvh: None,
            layers: u32::MAX,
        }
    }

    /// 创建带图层掩码的光线投射器。
    #[inline]
    pub const fn with_layers(layers: u32) -> Self {
        Self { bvh: None, layers }
    }

    /// 返回当前活动的图层掩码。
    #[inline]
    pub const fn layers(&self) -> u32 {
        self.layers
    }

    /// 设置当前活动的图层掩码。已有的 BVH 数据仍然有效，
    /// 但可能包含现在被过滤掉的条目。
    #[inline]
    pub fn set_layers(&mut self, layers: u32) {
        self.layers = layers;
    }

    /// 返回已构建的 BVH（如果有的话）。
    #[inline]
    pub const fn bvh(&self) -> Option<&Bvh> {
        self.bvh.as_ref()
    }

    /// 清除缓存的 BVH 数据。
    #[inline]
    pub fn clear_bvh(&mut self) {
        self.bvh = None;
    }

    /// 从 `scene` 中的可见网格节点构建节点级 BVH。
    pub fn build_bvh<G: GeometryProvider + ?Sized>(
        &mut self,
        scene: &SceneGraph,
        geometries: &G,
    ) -> Result<(), ValidationError> {
        let mut entries = Vec::new();
        for node_id in scene.iter_depth_first() {
            let Some((mesh_id, _material_id)) = mesh_node(scene, node_id, self.layers) else {
                continue;
            };
            let geometry = geometries
                .geometry(mesh_id)
                .ok_or(ValidationError::InvalidId)?;
            if geometry.is_empty() {
                continue;
            }
            let world = scene.world_matrix(node_id).unwrap_or(Mat4::IDENTITY);
            entries.push(BvhEntry::new(node_id, geometry.aabb().transform(world)));
        }
        self.bvh = Some(Bvh::build(&entries));
        Ok(())
    }

    /// 返回最近的交点（如果有的话）。
    pub fn cast_ray<G: GeometryProvider + ?Sized>(
        &self,
        ray: Ray3,
        scene: &SceneGraph,
        geometries: &G,
    ) -> Option<Intersection> {
        self.cast_ray_all(ray, scene, geometries).into_iter().next()
    }

    /// 返回按光线距离升序排列的所有交点。
    pub fn cast_ray_all<G: GeometryProvider + ?Sized>(
        &self,
        ray: Ray3,
        scene: &SceneGraph,
        geometries: &G,
    ) -> Vec<Intersection> {
        let mut hits = Vec::new();
        self.cast_ray_all_into(ray, scene, geometries, &mut hits);
        hits
    }

    /// 将所有排序后的交点写入调用者拥有的可重用存储中。
    pub fn cast_ray_all_into<G: GeometryProvider + ?Sized>(
        &self,
        ray: Ray3,
        scene: &SceneGraph,
        geometries: &G,
        hits: &mut Vec<Intersection>,
    ) {
        hits.clear();
        if let Some(bvh) = &self.bvh {
            bvh.visit_ray(ray, |node_id| {
                self.cast_candidate(ray, scene, geometries, node_id, hits)
            });
        } else {
            for node_id in scene.iter_depth_first() {
                self.cast_candidate(ray, scene, geometries, node_id, hits);
            }
        }
        sort_hits(hits);
    }

    /// 用于验证 BVH 结果的暴力全命中路径。
    pub fn cast_ray_all_bruteforce<G: GeometryProvider + ?Sized>(
        &self,
        ray: Ray3,
        scene: &SceneGraph,
        geometries: &G,
    ) -> Vec<Intersection> {
        let mut hits = Vec::new();
        for node_id in scene.iter_depth_first() {
            self.cast_candidate(ray, scene, geometries, node_id, &mut hits);
        }
        sort_hits(&mut hits);
        hits
    }

    /// 从透视相机的归一化设备坐标构建光线。
    #[inline]
    pub fn from_camera_ndc(camera: &PerspectiveCamera, ndc: Vec2) -> Ray3 {
        camera.screen_to_ray(ndc)
    }

    /// 测试单个候选节点与光线的交点。
    fn cast_candidate<G: GeometryProvider + ?Sized>(
        &self,
        ray: Ray3,
        scene: &SceneGraph,
        geometries: &G,
        node_id: NodeId,
        hits: &mut Vec<Intersection>,
    ) {
        let Some((mesh_id, material_id)) = mesh_node(scene, node_id, self.layers) else {
            return;
        };
        let Some(geometry) = geometries.geometry(mesh_id) else {
            return;
        };
        if geometry.is_empty() {
            return;
        }
        let world = scene.world_matrix(node_id).unwrap_or(Mat4::IDENTITY);
        let world_aabb = geometry.aabb().transform(world);
        if ray.intersect_aabb(world_aabb).is_none() {
            return;
        }
        intersect_geometry(ray, node_id, mesh_id, material_id, world, geometry, hits);
    }
}

impl Default for Raycaster {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// 从场景节点获取网格和材质 ID。
fn mesh_node(scene: &SceneGraph, node_id: NodeId, layers: u32) -> Option<(MeshId, MaterialId)> {
    let node = scene.get(node_id)?;
    if !node.visible || node.layer & layers == 0 {
        return None;
    }
    match node.kind {
        NodeKind::Mesh {
            mesh_id,
            material_id,
        } => Some((mesh_id, material_id)),
        _ => None,
    }
}

/// 按距离升序排序交点。
fn sort_hits(hits: &mut [Intersection]) {
    hits.sort_by(|a, b| {
        a.distance
            .total_cmp(&b.distance)
            .then_with(|| a.node_id.get().cmp(&b.node_id.get()))
    });
}

/// 计算几何体与光线的交点。
fn intersect_geometry(
    ray: Ray3,
    node_id: NodeId,
    mesh_id: MeshId,
    material_id: MaterialId,
    world: Mat4,
    geometry: &Geometry,
    hits: &mut Vec<Intersection>,
) {
    let normal_matrix = Mat3::from_mat4(world)
        .inverse()
        .map(Mat3::transpose)
        .unwrap_or_else(|| Mat3::from_mat4(world));

    if geometry.indices.is_empty() {
        for triangle in (0..geometry.positions.len()).step_by(3) {
            if triangle + 2 >= geometry.positions.len() {
                break;
            }
            intersect_triangle(
                ray,
                node_id,
                mesh_id,
                material_id,
                world,
                normal_matrix,
                geometry,
                triangle,
                triangle + 1,
                triangle + 2,
                hits,
            );
        }
    } else {
        for triangle in geometry.indices.chunks_exact(3) {
            let a = triangle[0] as usize;
            let b = triangle[1] as usize;
            let c = triangle[2] as usize;
            if a >= geometry.positions.len()
                || b >= geometry.positions.len()
                || c >= geometry.positions.len()
            {
                continue;
            }
            intersect_triangle(
                ray,
                node_id,
                mesh_id,
                material_id,
                world,
                normal_matrix,
                geometry,
                a,
                b,
                c,
                hits,
            );
        }
    }
}

/// 计算单个三角形与光线的交点。
#[allow(clippy::too_many_arguments)]
fn intersect_triangle(
    ray: Ray3,
    node_id: NodeId,
    mesh_id: MeshId,
    material_id: MaterialId,
    world: Mat4,
    normal_matrix: Mat3,
    geometry: &Geometry,
    a: usize,
    b: usize,
    c: usize,
    hits: &mut Vec<Intersection>,
) {
    let wa = world.mul_vec3(geometry.positions[a]);
    let wb = world.mul_vec3(geometry.positions[b]);
    let wc = world.mul_vec3(geometry.positions[c]);
    let Some((distance, bary_uv)) = ray.intersect_triangle(wa, wb, wc) else {
        return;
    };
    let u = bary_uv.x;
    let v = bary_uv.y;
    let w = 1.0 - u - v;
    let point = ray.at(distance);
    let normal = if geometry.normals.len() == geometry.positions.len() {
        normal_matrix
            .mul_vec3(geometry.normals[a] * w + geometry.normals[b] * u + geometry.normals[c] * v)
            .normalize()
    } else {
        (wb - wa).cross(wc - wa).normalize()
    };
    let uv = if geometry.uvs.len() == geometry.positions.len() {
        geometry.uvs[a] * w + geometry.uvs[b] * u + geometry.uvs[c] * v
    } else {
        Vec2::ZERO
    };

    hits.push(Intersection {
        node_id,
        mesh_id,
        material_id,
        distance,
        point,
        normal,
        uv,
    });
}
