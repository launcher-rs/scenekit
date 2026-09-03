use std::path::{Path, PathBuf};

use scenekit_core::{LoadError, ScenixError, TextureId};
use scenekit_math::{Vec2, Vec3};
use scenekit_mesh::Geometry;

/// 与 scenekit CPU 资产相关的 OBJ 材质元数据。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjMaterial {
    /// 源材质名称。
    pub name: String,
    /// 可选的漫反射/基础颜色纹理路径。
    pub diffuse_texture: Option<PathBuf>,
    /// 从材质顺序分配的稳定纹理标识符。
    pub texture_id: Option<TextureId>,
}

/// 将 OBJ 文件解码为每个源模型一个几何体。
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjAsset {
    /// 按源顺序排列的模型几何体。
    pub geometries: Vec<Geometry>,
    /// 从 MTL 文件加载的材质元数据（如果有）。
    pub materials: Vec<ObjMaterial>,
    /// 按源顺序排列的几何体材质索引。
    pub geometry_materials: Vec<Option<usize>>,
}

/// 加载 OBJ 几何体，对多边形面进行三角化。
pub fn load(path: impl AsRef<Path>) -> Result<Vec<Geometry>, ScenixError> {
    Ok(load_with_materials(path)?.geometries)
}

/// 加载 OBJ 几何体以及 MTL 材质/纹理元数据。
pub fn load_with_materials(path: impl AsRef<Path>) -> Result<ObjAsset, ScenixError> {
    let path = path.as_ref();
    let options = tobj::LoadOptions {
        triangulate: true,
        single_index: true,
        ..Default::default()
    };
    let (models, materials) = tobj::load_obj(path, &options).map_err(|err| match err {
        tobj::LoadError::OpenFileFailed => LoadError::NotFound,
        _ => LoadError::Parse,
    })?;

    let mut asset = ObjAsset::default();
    asset.geometries.reserve(models.len());
    asset.geometry_materials.reserve(models.len());
    for model in models {
        asset.geometry_materials.push(model.mesh.material_id);
        asset.geometries.push(geometry_from_obj_mesh(&model.mesh)?);
    }

    if let Ok(materials) = materials {
        asset.materials.reserve(materials.len());
        for (index, material) in materials.into_iter().enumerate() {
            let diffuse_texture = material.diffuse_texture.and_then(|texture| {
                if texture.is_empty() {
                    None
                } else {
                    Some(resolve_relative(path, &texture))
                }
            });
            asset.materials.push(ObjMaterial {
                name: material.name,
                diffuse_texture: diffuse_texture.clone(),
                texture_id: diffuse_texture.map(|_| TextureId::new(index as u64 + 1)),
            });
        }
    }

    Ok(asset)
}

fn geometry_from_obj_mesh(mesh: &tobj::Mesh) -> Result<Geometry, ScenixError> {
    let mut geometry = Geometry::new();
    geometry.positions.reserve(mesh.positions.len() / 3);
    geometry.normals.reserve(mesh.normals.len() / 3);
    geometry.uvs.reserve(mesh.texcoords.len() / 2);
    geometry.indices.extend(mesh.indices.iter().copied());

    for position in mesh.positions.chunks_exact(3) {
        geometry
            .positions
            .push(Vec3::new(position[0], position[1], position[2]));
    }
    for normal in mesh.normals.chunks_exact(3) {
        geometry
            .normals
            .push(Vec3::new(normal[0], normal[1], normal[2]).normalize());
    }
    for uv in mesh.texcoords.chunks_exact(2) {
        geometry.uvs.push(Vec2::new(uv[0], uv[1]));
    }

    if geometry.normals.is_empty() {
        geometry.compute_normals();
    }
    geometry.validate()?;
    Ok(geometry)
}

fn resolve_relative(source: &Path, value: &str) -> PathBuf {
    let path = PathBuf::from(value);
    if path.is_absolute() {
        path
    } else {
        source.parent().unwrap_or_else(|| Path::new("")).join(path)
    }
}
