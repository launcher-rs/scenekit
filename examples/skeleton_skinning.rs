//! CPU 蒙皮 + GPU 钩子注册的冒烟测试。
//!
//! 演示构建蒙皮数据模型、从 `SkeletonPose` 计算最终关节
//! 矩阵、对小型几何体进行 CPU 蒙皮，以及
//! 将结果注册到渲染器 GPU 蒙皮注册表中。
//!
//! 运行方式：
//!   cargo run -p scenekit --example skeleton_skinning --features mesh,animato

use scenekit::{
    Geometry, GpuSkinningRegistry, Mat4, MeshId, MorphTarget, SkeletonPose, SkinningAttributes,
    Transform, Vec3, apply_morph, cpu_skin, final_joint_matrices,
};

fn main() {
    // 构建一个包含 2 个顶点的小型几何体和一个 1 关节的蒙皮。
    let mut geometry = Geometry::new();
    geometry.positions = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];
    geometry.normals = vec![Vec3::Z, Vec3::Z];

    let skin = SkinningAttributes {
        joints: vec![[0, 0, 0, 0], [0, 0, 0, 0]],
        weights: vec![[1.0, 0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]],
    };

    // 一个将单个关节向上平移 2 个单位的姿态。
    let pose = SkeletonPose::new(vec![Transform {
        translation: Vec3::new(0.0, 2.0, 0.0),
        ..Default::default()
    }]);
    let bone_world: Vec<Mat4> = pose
        .bones
        .iter()
        .map(|t| Mat4::from_translation(t.translation))
        .collect();

    let inverse_binds = vec![Mat4::IDENTITY];
    let final_mats = final_joint_matrices(&bone_world, &inverse_binds);
    let skinned = cpu_skin(&geometry, &skin, &final_mats);

    println!(
        "skeleton_skinning: original[1].y={:.2} skinned[1].y={:.2}",
        geometry.positions[1].y, skinned.positions[1].y
    );
    assert!((skinned.positions[1].y - 3.0).abs() < 1e-3);

    // 注册到 GPU 蒙皮注册表（渲染器拥有的上传钩子）。
    let mut registry = GpuSkinningRegistry::new();
    let mesh_id = MeshId::new(1);
    registry.register_skin(mesh_id, final_mats.clone());
    registry.register_morph_targets(mesh_id, vec![0.0]);
    assert!(registry.has_skin(mesh_id));
    assert!(registry.has_morph(mesh_id));

    // 将变形目标应用到克隆的几何体上。
    let mut target = MorphTarget::new("smile".to_string());
    target.positions_delta = vec![Vec3::new(0.5, 0.0, 0.0), Vec3::new(0.0, 0.5, 0.0)];
    let morphed = apply_morph(&geometry, &[target], &[1.0]);
    println!(
        "skeleton_skinning: morphed[0].x={:.2} (expected 0.5)",
        morphed.positions[0].x
    );

    println!("skeleton_skinning: done (CPU skin + GPU registry + morph apply)");
}
