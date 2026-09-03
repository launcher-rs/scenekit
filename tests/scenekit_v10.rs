#[cfg(all(feature = "helpers", feature = "camera", feature = "raycaster"))]
#[test]
fn stable_default_facade_exports_raycasting_and_helpers() {
    let grid = scenekit::GridHelper::new(2.0, 2).to_geometry();
    assert!(grid.validate().is_ok());

    let axes = scenekit::AxesHelper::new(1.0).to_geometry();
    assert_eq!(axes.positions.len(), 6);

    let camera = scenekit::PerspectiveCamera::new(60.0, 1.0, 0.1, 100.0)
        .position(scenekit::Vec3::new(0.0, 0.0, 4.0))
        .target(scenekit::Vec3::ZERO);
    let ray = scenekit::Raycaster::from_camera_ndc(&camera, scenekit::Vec2::ZERO);
    assert!(ray.direction.z < -0.9);
}

#[cfg(all(feature = "renderer", feature = "material"))]
#[test]
fn stable_facade_exports_renderer_material_registration_api() {
    let mut gpu_scene = scenekit::GpuScene::new();
    gpu_scene
        .register_physical_material(
            scenekit::MaterialId::new(1),
            &scenekit::PhysicalMaterial::new(),
        )
        .unwrap();
    gpu_scene
        .register_toon_material(scenekit::MaterialId::new(2), &scenekit::ToonMaterial::new())
        .unwrap();
    gpu_scene
        .register_wireframe_material(
            scenekit::MaterialId::new(3),
            &scenekit::WireframeMaterial::new(),
        )
        .unwrap();
    gpu_scene
        .register_normal_material(
            scenekit::MaterialId::new(4),
            &scenekit::NormalMaterial::new(),
        )
        .unwrap();

    assert_eq!(gpu_scene.material_count(), 4);
}

#[cfg(all(feature = "loader", feature = "renderer"))]
#[test]
fn stable_facade_exports_asset_upload_bridge_types() {
    fn assert_upload_bridge<T: scenekit::RendererAssetExt>() {}
    assert_upload_bridge::<scenekit::Renderer>();

    let stats = scenekit::UploadedAssetStats::default();
    assert_eq!(stats.meshes, 0);
}

#[cfg(feature = "wasm")]
#[test]
fn stable_facade_exports_wasm_demo_helpers() {
    assert_eq!(scenekit::clamp_canvas_size(0, 0), (1, 1));
    assert_eq!(
        scenekit::pointer_button_from_dom(0),
        Some(scenekit::PointerButton::Left)
    );
    assert_eq!(
        scenekit::BrowserBackendPreference::Auto,
        scenekit::BrowserBackendPreference::Auto
    );
    assert_eq!(
        scenekit::BrowserBackendKind::WebGl,
        scenekit::BrowserBackendKind::WebGl
    );
}
