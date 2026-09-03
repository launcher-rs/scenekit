//! 通过 AnimationMixer 处理标记 + 循环 + 完成事件。
//!
//! 运行方式：
//!   cargo run -p scenekit --example animation_events --features animato,scene,light

use std::collections::BTreeMap;

use scenekit::{
    AnimationClip, AnimationEvent, AnimationMarker, AnimationMixer, CameraId, CameraStores,
    ClipChannel, ClipTrack, Color, DirectionalLight, KeyframeInterpolation, KeyframeVec3, LightId,
    LightStores, LoopMode, MaterialId, MeshId, NodeProperty, OrthographicCamera, PbrMaterial,
    PerspectiveCamera, PointLight, PropertyBinding, SceneGraph, SceneNode, SpotLight, Vec3,
};

fn main() {
    let mut scene = SceneGraph::new();
    let node = scene.add(SceneNode::new("runner"));

    // 一个包含两个标记和位移的 2 秒片段。
    let clip = AnimationClip::empty("sprint")
        .with_channel(ClipChannel {
            binding: PropertyBinding::Node {
                node_id: node,
                property: NodeProperty::Translation,
            },
            track: ClipTrack::Vec3(KeyframeVec3::new(
                vec![0.0, 1.0, 2.0],
                vec![
                    Vec3::ZERO,
                    Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(2.0, 0.0, 0.0),
                ],
                KeyframeInterpolation::Linear,
            )),
        })
        .with_marker(AnimationMarker::new("start", 0.0))
        .with_marker(AnimationMarker::new("midpoint", 1.0))
        .with_marker(AnimationMarker::new("finish", 2.0));

    let mut mixer = AnimationMixer::new();
    let ci = mixer.add_clip(clip);
    let action = mixer.add_action(ci);
    // 重复两次后停止，这样我们就能看到循环和完成事件。
    mixer
        .action_mut(action)
        .unwrap()
        .set_loop_mode(LoopMode::Repeat { max: 2 });
    mixer.action_mut(action).unwrap().play(0.0);

    let mut perspective: BTreeMap<CameraId, PerspectiveCamera> = BTreeMap::new();
    let mut orthographic: BTreeMap<CameraId, OrthographicCamera> = BTreeMap::new();
    let mut materials: BTreeMap<MaterialId, PbrMaterial> = BTreeMap::new();
    let mut point_lights: BTreeMap<LightId, PointLight> =
        BTreeMap::from([(LightId::new(1), PointLight::new(Color::WHITE, 1.0, 10.0))]);
    let mut spot_lights: BTreeMap<LightId, SpotLight> = BTreeMap::new();
    let mut directional_lights: BTreeMap<LightId, DirectionalLight> = BTreeMap::new();
    let mut morphs: BTreeMap<MeshId, Vec<f32>> = BTreeMap::new();

    let dt = 1.0 / 30.0;
    let mut total_events = 0usize;
    for frame in 0..150 {
        let mut camera_stores = CameraStores {
            perspective: &mut perspective,
            orthographic: &mut orthographic,
        };
        let mut light_stores = LightStores {
            point: &mut point_lights,
            spot: &mut spot_lights,
            directional: &mut directional_lights,
        };
        let result = mixer
            .tick(
                dt,
                &mut scene,
                &mut camera_stores,
                &mut materials,
                &mut light_stores,
                &mut [],
                &mut morphs,
            )
            .expect("mixer tick");

        for event in &result.events {
            total_events += 1;
            match event {
                AnimationEvent::Loop { action, iteration } => {
                    println!("frame={frame:>3} Loop action={action} iteration={iteration}");
                }
                AnimationEvent::Marker { action, name } => {
                    println!("frame={frame:>3} Marker action={action} name={name}");
                }
                AnimationEvent::Finished { action } => {
                    println!("frame={frame:>3} Finished action={action}");
                }
            }
        }
    }

    println!("animation_events: done (total events fired: {total_events})");
}
