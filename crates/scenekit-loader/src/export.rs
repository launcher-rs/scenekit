use scenekit_core::{LoadError, ScenixError};
use scenekit_math::Vec3;
use scenekit_mesh::Geometry;

use crate::AssetPackage;

/// 将包元数据和网格数量导出为紧凑的 glTF 2.0 JSON 文档。
pub fn gltf_json_string(package: &AssetPackage) -> String {
    format!(
        "{{\"asset\":{{\"version\":\"2.0\",\"generator\":\"scenekit-loader 1.3\"}},\"extras\":{{\"label\":\"{}\",\"meshes\":{},\"materials\":{},\"textures\":{},\"animations\":{},\"skins\":{}}}}}",
        escape_json(&package.label),
        package.meshes.len(),
        package.materials.len(),
        package.textures.len() + package.texture_cubes.len(),
        package.animations.len(),
        package.skins.len()
    )
}

/// 导出简单的场景摘要 JSON 文档。
pub fn scene_json_string(package: &AssetPackage) -> String {
    format!(
        "{{\"label\":\"{}\",\"meshes\":{},\"materials\":{},\"lights\":{},\"cameras\":{},\"diagnostics\":{}}}",
        escape_json(&package.label),
        package.meshes.len(),
        package.materials.len(),
        package.lights.len(),
        package.cameras.len(),
        package.diagnostics.len()
    )
}

/// 将包网格导出为 Wavefront OBJ 字符串。
pub fn obj_string(package: &AssetPackage) -> String {
    let mut out = String::from("# scenekit-loader OBJ export\n");
    let mut vertex_offset = 1_u32;
    for (mesh_id, geometry) in &package.meshes {
        out.push_str(&format!("o mesh_{}\n", mesh_id.get()));
        for position in &geometry.positions {
            out.push_str(&format!("v {} {} {}\n", position.x, position.y, position.z));
        }
        for uv in &geometry.uvs {
            out.push_str(&format!("vt {} {}\n", uv.x, uv.y));
        }
        for normal in &geometry.normals {
            out.push_str(&format!("vn {} {} {}\n", normal.x, normal.y, normal.z));
        }
        let indices = effective_indices(geometry);
        for triangle in indices.as_chunks::<3>().0 {
            out.push_str(&format!(
                "f {} {} {}\n",
                triangle[0] + vertex_offset,
                triangle[1] + vertex_offset,
                triangle[2] + vertex_offset
            ));
        }
        vertex_offset += geometry.positions.len() as u32;
    }
    out
}

/// 将包网格导出为 ASCII STL 字符串。
pub fn stl_ascii_string(package: &AssetPackage) -> String {
    let mut out = format!("solid {}\n", sanitize_label(&package.label));
    for geometry in package.meshes.values() {
        write_stl_geometry(&mut out, geometry);
    }
    out.push_str(&format!("endsolid {}\n", sanitize_label(&package.label)));
    out
}

/// 将第一个网格导出为简单的 ASCII PLY 文档。
pub fn ply_ascii_string(package: &AssetPackage) -> Result<String, ScenixError> {
    let geometry = package
        .meshes
        .values()
        .next()
        .ok_or(ScenixError::Load(LoadError::NotFound))?;
    let indices = effective_indices(geometry);
    let mut out = String::new();
    out.push_str("ply\nformat ascii 1.0\n");
    out.push_str(&format!("element vertex {}\n", geometry.positions.len()));
    out.push_str("property float x\nproperty float y\nproperty float z\n");
    out.push_str(&format!("element face {}\n", indices.len() / 3));
    out.push_str("property list uchar uint vertex_indices\nend_header\n");
    for position in &geometry.positions {
        out.push_str(&format!("{} {} {}\n", position.x, position.y, position.z));
    }
    for triangle in indices.as_chunks::<3>().0 {
        out.push_str(&format!(
            "3 {} {} {}\n",
            triangle[0], triangle[1], triangle[2]
        ));
    }
    Ok(out)
}

/// 将紧凑的 glTF 摘要导出为类似 GLB 的字节，用于工具链冒烟测试。
pub fn glb_summary_bytes(package: &AssetPackage) -> Vec<u8> {
    let mut json = gltf_json_string(package).into_bytes();
    while !json.len().is_multiple_of(4) {
        json.push(b' ');
    }
    let total_len = 12 + 8 + json.len();
    let mut bytes = Vec::with_capacity(total_len);
    bytes.extend_from_slice(b"glTF");
    bytes.extend_from_slice(&2_u32.to_le_bytes());
    bytes.extend_from_slice(&(total_len as u32).to_le_bytes());
    bytes.extend_from_slice(&(json.len() as u32).to_le_bytes());
    bytes.extend_from_slice(b"JSON");
    bytes.extend_from_slice(&json);
    bytes
}

fn write_stl_geometry(out: &mut String, geometry: &Geometry) {
    let indices = effective_indices(geometry);
    for triangle in indices.as_chunks::<3>().0 {
        let a = geometry.positions[triangle[0] as usize];
        let b = geometry.positions[triangle[1] as usize];
        let c = geometry.positions[triangle[2] as usize];
        let normal = (b - a).cross(c - a).normalize();
        out.push_str(&format!(
            "  facet normal {} {} {}\n    outer loop\n",
            normal.x, normal.y, normal.z
        ));
        write_stl_vertex(out, a);
        write_stl_vertex(out, b);
        write_stl_vertex(out, c);
        out.push_str("    endloop\n  endfacet\n");
    }
}

fn write_stl_vertex(out: &mut String, position: Vec3) {
    out.push_str(&format!(
        "      vertex {} {} {}\n",
        position.x, position.y, position.z
    ));
}

fn effective_indices(geometry: &Geometry) -> Vec<u32> {
    if geometry.indices.is_empty() {
        (0..geometry.positions.len())
            .map(|index| index as u32)
            .collect()
    } else {
        geometry.indices.clone()
    }
}

fn escape_json(value: &str) -> String {
    value
        .chars()
        .flat_map(|ch| match ch {
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            _ => vec![ch],
        })
        .collect()
}

fn sanitize_label(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}
