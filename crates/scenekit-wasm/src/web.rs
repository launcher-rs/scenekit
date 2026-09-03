use std::collections::BTreeMap;

use js_sys::{Float32Array, Reflect, Uint16Array};
use scenekit_animato::ScalarTrack;
use scenekit_camera::{OrbitController, PerspectiveCamera};
use scenekit_core::{Color, Inspectable, LightId, MaterialId, MeshId, NodeId, ScenixError};
use scenekit_input::{GamepadId, InputState, TouchId, ViewportMetrics};
use scenekit_light::{DirectionalLight, PointLight};
use scenekit_material::{
    LambertMaterial, PbrMaterial, PhysicalMaterial, ToonMaterial, UnlitMaterial, WireframeMaterial,
};
use scenekit_math::{Transform, Vec2, Vec3};
use scenekit_mesh::{Geometry, box_geometry, plane_geometry, sphere_geometry, torus_geometry};
use scenekit_raycaster::Raycaster;
use scenekit_renderer::{Renderer, RendererConfig, wgpu};
use scenekit_scene::{NodeKind, SceneGraph, SceneNode, SelectionMode, TransformMode};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlRenderingContext,
    WebGlShader, WebGlTexture, WebGlUniformLocation, window,
};

use crate::{
    CanvasMetrics, WebGlCapabilityLevel, clamp_canvas_size, gamepad_axis_from_standard,
    gamepad_button_from_standard, key_code_from_dom, pointer_button_from_dom, touch_phase_from_dom,
};

const OBJECT_LAYER: u32 = 1;
const HELPER_LAYER: u32 = 2;

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct DemoObject {
    node_id: NodeId,
    material_id: MaterialId,
    name: &'static str,
    material_name: &'static str,
}

struct LabRuntime {
    scene: SceneGraph,
    camera: PerspectiveCamera,
    orbit: OrbitController,
    input: InputState,
    geometries: BTreeMap<MeshId, Geometry>,
    raycaster: Raycaster,
    objects: Vec<DemoObject>,
    helper_node: NodeId,
    animated_node: NodeId,
    pulse_track: ScalarTrack,
    last_timestamp_ms: Option<f64>,
    fps: f32,
    paused: bool,
    helpers_visible: bool,
    wireframe_enabled: bool,
    bloom_enabled: bool,
    ssao_enabled: bool,
    selected_node: Option<NodeId>,
    selected_name: String,
    selected_distance: f32,
    active_material: String,
    transform_mode: TransformMode,
}

struct WebGlMesh {
    vertex_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    line_index_buffer: WebGlBuffer,
    index_count: i32,
    line_index_count: i32,
}

#[derive(Clone, Copy, Debug)]
enum WebGlMaterialModel {
    Pbr,
    Physical,
    Toon,
    Lambert,
    Unlit,
}

struct WebGlMaterial {
    color: Color,
    texture: Option<WebGlTexture>,
    model: WebGlMaterialModel,
    metallic: f32,
    roughness: f32,
    clearcoat: f32,
    unlit: bool,
    wireframe: bool,
}

struct WebGlProgramState {
    program: WebGlProgram,
    position_attrib: u32,
    normal_attrib: u32,
    uv_attrib: u32,
    color_attrib: u32,
    view_projection_uniform: WebGlUniformLocation,
    model_uniform: WebGlUniformLocation,
    material_uniform: WebGlUniformLocation,
    light_direction_uniform: WebGlUniformLocation,
    point_position_range_uniform: WebGlUniformLocation,
    point_color_uniform: WebGlUniformLocation,
    texture_uniform: WebGlUniformLocation,
    use_texture_uniform: WebGlUniformLocation,
    material_model_uniform: WebGlUniformLocation,
    metallic_roughness_uniform: WebGlUniformLocation,
    unlit_uniform: WebGlUniformLocation,
    bloom_uniform: WebGlUniformLocation,
    ssao_uniform: WebGlUniformLocation,
}

enum WebGlBackendContext {
    WebGl2(WebGl2RenderingContext),
    WebGl1(WebGlRenderingContext),
}

impl WebGlBackendContext {
    fn capability(&self) -> WebGlCapabilityLevel {
        match self {
            Self::WebGl2(_) => WebGlCapabilityLevel::WebGl2,
            Self::WebGl1(_) => WebGlCapabilityLevel::WebGl1,
        }
    }

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        match self {
            Self::WebGl2(gl) => gl.viewport(x, y, width, height),
            Self::WebGl1(gl) => gl.viewport(x, y, width, height),
        }
    }

    fn enable(&self, cap: u32) {
        match self {
            Self::WebGl2(gl) => gl.enable(cap),
            Self::WebGl1(gl) => gl.enable(cap),
        }
    }

    fn disable(&self, cap: u32) {
        match self {
            Self::WebGl2(gl) => gl.disable(cap),
            Self::WebGl1(gl) => gl.disable(cap),
        }
    }

    fn depth_func(&self, func: u32) {
        match self {
            Self::WebGl2(gl) => gl.depth_func(func),
            Self::WebGl1(gl) => gl.depth_func(func),
        }
    }

    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        match self {
            Self::WebGl2(gl) => gl.clear_color(r, g, b, a),
            Self::WebGl1(gl) => gl.clear_color(r, g, b, a),
        }
    }

    fn clear(&self, mask: u32) {
        match self {
            Self::WebGl2(gl) => gl.clear(mask),
            Self::WebGl1(gl) => gl.clear(mask),
        }
    }

    fn create_buffer(&self) -> Option<WebGlBuffer> {
        match self {
            Self::WebGl2(gl) => gl.create_buffer(),
            Self::WebGl1(gl) => gl.create_buffer(),
        }
    }

    fn bind_buffer(&self, target: u32, buffer: Option<&WebGlBuffer>) {
        match self {
            Self::WebGl2(gl) => gl.bind_buffer(target, buffer),
            Self::WebGl1(gl) => gl.bind_buffer(target, buffer),
        }
    }

    fn buffer_data_with_array_buffer_view(&self, target: u32, data: &js_sys::Object, usage: u32) {
        match self {
            Self::WebGl2(gl) => gl.buffer_data_with_array_buffer_view(target, data, usage),
            Self::WebGl1(gl) => gl.buffer_data_with_array_buffer_view(target, data, usage),
        }
    }

    fn create_texture(&self) -> Option<WebGlTexture> {
        match self {
            Self::WebGl2(gl) => gl.create_texture(),
            Self::WebGl1(gl) => gl.create_texture(),
        }
    }

    fn active_texture(&self, texture: u32) {
        match self {
            Self::WebGl2(gl) => gl.active_texture(texture),
            Self::WebGl1(gl) => gl.active_texture(texture),
        }
    }

    fn bind_texture(&self, target: u32, texture: Option<&WebGlTexture>) {
        match self {
            Self::WebGl2(gl) => gl.bind_texture(target, texture),
            Self::WebGl1(gl) => gl.bind_texture(target, texture),
        }
    }

    fn tex_parameteri(&self, target: u32, pname: u32, param: i32) {
        match self {
            Self::WebGl2(gl) => gl.tex_parameteri(target, pname, param),
            Self::WebGl1(gl) => gl.tex_parameteri(target, pname, param),
        }
    }

    fn use_program(&self, program: Option<&WebGlProgram>) {
        match self {
            Self::WebGl2(gl) => gl.use_program(program),
            Self::WebGl1(gl) => gl.use_program(program),
        }
    }

    fn uniform1i(&self, location: Option<&WebGlUniformLocation>, v0: i32) {
        match self {
            Self::WebGl2(gl) => gl.uniform1i(location, v0),
            Self::WebGl1(gl) => gl.uniform1i(location, v0),
        }
    }

    fn uniform1f(&self, location: Option<&WebGlUniformLocation>, v0: f32) {
        match self {
            Self::WebGl2(gl) => gl.uniform1f(location, v0),
            Self::WebGl1(gl) => gl.uniform1f(location, v0),
        }
    }

    fn uniform3f(&self, location: Option<&WebGlUniformLocation>, v0: f32, v1: f32, v2: f32) {
        match self {
            Self::WebGl2(gl) => gl.uniform3f(location, v0, v1, v2),
            Self::WebGl1(gl) => gl.uniform3f(location, v0, v1, v2),
        }
    }

    fn uniform4f(
        &self,
        location: Option<&WebGlUniformLocation>,
        v0: f32,
        v1: f32,
        v2: f32,
        v3: f32,
    ) {
        match self {
            Self::WebGl2(gl) => gl.uniform4f(location, v0, v1, v2, v3),
            Self::WebGl1(gl) => gl.uniform4f(location, v0, v1, v2, v3),
        }
    }

    fn uniform4fv_with_f32_array(&self, location: Option<&WebGlUniformLocation>, data: &[f32]) {
        match self {
            Self::WebGl2(gl) => gl.uniform4fv_with_f32_array(location, data),
            Self::WebGl1(gl) => gl.uniform4fv_with_f32_array(location, data),
        }
    }

    fn uniform_matrix4fv_with_f32_array(
        &self,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    ) {
        match self {
            Self::WebGl2(gl) => gl.uniform_matrix4fv_with_f32_array(location, transpose, data),
            Self::WebGl1(gl) => gl.uniform_matrix4fv_with_f32_array(location, transpose, data),
        }
    }

    fn enable_vertex_attrib_array(&self, index: u32) {
        match self {
            Self::WebGl2(gl) => gl.enable_vertex_attrib_array(index),
            Self::WebGl1(gl) => gl.enable_vertex_attrib_array(index),
        }
    }

    fn vertex_attrib_pointer_with_i32(
        &self,
        index: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        match self {
            Self::WebGl2(gl) => {
                gl.vertex_attrib_pointer_with_i32(index, size, type_, normalized, stride, offset)
            }
            Self::WebGl1(gl) => {
                gl.vertex_attrib_pointer_with_i32(index, size, type_, normalized, stride, offset)
            }
        }
    }

    fn draw_elements_with_i32(&self, mode: u32, count: i32, type_: u32, offset: i32) {
        match self {
            Self::WebGl2(gl) => gl.draw_elements_with_i32(mode, count, type_, offset),
            Self::WebGl1(gl) => gl.draw_elements_with_i32(mode, count, type_, offset),
        }
    }

    fn create_shader(&self, shader_type: u32) -> Option<WebGlShader> {
        match self {
            Self::WebGl2(gl) => gl.create_shader(shader_type),
            Self::WebGl1(gl) => gl.create_shader(shader_type),
        }
    }

    fn shader_source(&self, shader: &WebGlShader, source: &str) {
        match self {
            Self::WebGl2(gl) => gl.shader_source(shader, source),
            Self::WebGl1(gl) => gl.shader_source(shader, source),
        }
    }

    fn compile_shader(&self, shader: &WebGlShader) {
        match self {
            Self::WebGl2(gl) => gl.compile_shader(shader),
            Self::WebGl1(gl) => gl.compile_shader(shader),
        }
    }

    fn get_shader_parameter(&self, shader: &WebGlShader, pname: u32) -> JsValue {
        match self {
            Self::WebGl2(gl) => gl.get_shader_parameter(shader, pname),
            Self::WebGl1(gl) => gl.get_shader_parameter(shader, pname),
        }
    }

    fn get_shader_info_log(&self, shader: &WebGlShader) -> Option<String> {
        match self {
            Self::WebGl2(gl) => gl.get_shader_info_log(shader),
            Self::WebGl1(gl) => gl.get_shader_info_log(shader),
        }
    }

    fn create_program(&self) -> Option<WebGlProgram> {
        match self {
            Self::WebGl2(gl) => gl.create_program(),
            Self::WebGl1(gl) => gl.create_program(),
        }
    }

    fn attach_shader(&self, program: &WebGlProgram, shader: &WebGlShader) {
        match self {
            Self::WebGl2(gl) => gl.attach_shader(program, shader),
            Self::WebGl1(gl) => gl.attach_shader(program, shader),
        }
    }

    fn link_program(&self, program: &WebGlProgram) {
        match self {
            Self::WebGl2(gl) => gl.link_program(program),
            Self::WebGl1(gl) => gl.link_program(program),
        }
    }

    fn get_program_parameter(&self, program: &WebGlProgram, pname: u32) -> JsValue {
        match self {
            Self::WebGl2(gl) => gl.get_program_parameter(program, pname),
            Self::WebGl1(gl) => gl.get_program_parameter(program, pname),
        }
    }

    fn get_program_info_log(&self, program: &WebGlProgram) -> Option<String> {
        match self {
            Self::WebGl2(gl) => gl.get_program_info_log(program),
            Self::WebGl1(gl) => gl.get_program_info_log(program),
        }
    }

    fn get_attrib_location(&self, program: &WebGlProgram, name: &str) -> i32 {
        match self {
            Self::WebGl2(gl) => gl.get_attrib_location(program, name),
            Self::WebGl1(gl) => gl.get_attrib_location(program, name),
        }
    }

    fn get_uniform_location(
        &self,
        program: &WebGlProgram,
        name: &str,
    ) -> Option<WebGlUniformLocation> {
        match self {
            Self::WebGl2(gl) => gl.get_uniform_location(program, name),
            Self::WebGl1(gl) => gl.get_uniform_location(program, name),
        }
    }

    fn tex_image_2d_with_u8(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pixels: Option<&[u8]>,
    ) -> Result<(), JsValue> {
        match self {
            Self::WebGl2(gl) => gl
                .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    target,
                    level,
                    internal_format,
                    width,
                    height,
                    border,
                    format,
                    type_,
                    pixels,
                ),
            Self::WebGl1(gl) => gl
                .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    target,
                    level,
                    internal_format,
                    width,
                    height,
                    border,
                    format,
                    type_,
                    pixels,
                ),
        }
    }
}

/// 首选浏览器渲染后端。
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BrowserBackendPreference {
    /// 当浏览器已知安全支持 WebGPU 时选择 WebGPU，否则选择 WebGL。
    Auto,
    /// 强制使用现有 WebGPU/wgpu 渲染器。
    WebGpu,
    /// 强制使用 WebGL 回退渲染器。
    WebGl,
}

/// 活动浏览器渲染后端。
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BrowserBackendKind {
    /// 现有 WebGPU/wgpu 渲染器处于活动状态。
    WebGpu,
    /// WebGL 回退渲染器处于活动状态。
    WebGl,
    /// 调用方正在使用应用级 Canvas2D 回退。
    CanvasFallback,
}

/// 返回画布的有效渲染器尺寸。
pub fn canvas_size(canvas: &HtmlCanvasElement) -> (u32, u32) {
    let css_width = canvas.client_width().max(0) as u32;
    let css_height = canvas.client_height().max(0) as u32;
    let width = if css_width == 0 {
        canvas.width()
    } else {
        css_width
    };
    let height = if css_height == 0 {
        canvas.height()
    } else {
        css_height
    };
    clamp_canvas_size(width, height)
}

/// 使用浏览器 DPR 返回逻辑和物理画布测量值。
pub fn canvas_metrics(canvas: &HtmlCanvasElement) -> CanvasMetrics {
    let (logical_width, logical_height) = canvas_size(canvas);
    let dpr = window().map_or(1.0, |window| window.device_pixel_ratio()) as f32;
    CanvasMetrics::new(logical_width, logical_height, dpr)
}

/// 带有生成场景和 DOM 输入状态的浏览器渲染器包装器。
#[wasm_bindgen]
pub struct WebRenderer {
    renderer: Renderer,
    lab: LabRuntime,
}

impl LabRuntime {
    fn new(width: u32, height: u32) -> Self {
        let cube_mesh = MeshId::new(1);
        let sphere_mesh = MeshId::new(2);
        let torus_mesh = MeshId::new(3);
        let floor_mesh = MeshId::new(4);
        let helper_mesh = MeshId::new(5);

        let pbr_id = MaterialId::new(1);
        let toon_id = MaterialId::new(2);
        let physical_id = MaterialId::new(3);
        let floor_id = MaterialId::new(4);
        let helper_id = MaterialId::new(5);

        let mut geometries = BTreeMap::new();
        geometries.insert(
            cube_mesh,
            with_color(
                box_geometry(0.9, 0.9, 0.9, 1, 1, 1),
                Color::from_hex(0x4EA1FF),
            ),
        );
        geometries.insert(
            sphere_mesh,
            with_color(sphere_geometry(0.52, 32, 16), Color::from_hex(0xFFCC66)),
        );
        geometries.insert(
            torus_mesh,
            with_color(
                torus_geometry(0.48, 0.14, 32, 12),
                Color::from_hex(0xD970FF),
            ),
        );
        geometries.insert(
            floor_mesh,
            with_color(plane_geometry(7.0, 7.0, 1, 1), Color::from_hex(0x2D3446)),
        );
        geometries.insert(helper_mesh, helper_geometry());

        let mut scene = SceneGraph::new();
        let cube = scene.add(
            SceneNode::mesh("Cube", cube_mesh, pbr_id)
                .transform(Transform::from_translation(Vec3::new(-1.25, 0.55, 0.0)))
                .layer(OBJECT_LAYER),
        );
        let sphere = scene.add(
            SceneNode::mesh("Sphere", sphere_mesh, toon_id)
                .transform(Transform::from_translation(Vec3::new(0.0, 0.85, -0.25)))
                .layer(OBJECT_LAYER),
        );
        let torus = scene.add(
            SceneNode::mesh("Torus", torus_mesh, physical_id)
                .transform(Transform::from_translation(Vec3::new(1.25, 0.75, 0.1)))
                .layer(OBJECT_LAYER),
        );
        scene.add(
            SceneNode::mesh("Floor", floor_mesh, floor_id)
                .transform(Transform::from_translation(Vec3::new(0.0, -0.03, 0.0)))
                .layer(OBJECT_LAYER),
        );

        let helper_group = SceneNode::group("Helpers").layer(HELPER_LAYER);
        let helper_node = scene.add(helper_group);
        let _axes = scene
            .add_child(
                helper_node,
                SceneNode::mesh("Axes", helper_mesh, helper_id)
                    .transform(Transform::from_scale(Vec3::new(0.35, 0.35, 0.35))),
            )
            .unwrap();
        let _grid = scene
            .add_child(
                helper_node,
                SceneNode::mesh("Grid", helper_mesh, helper_id)
                    .transform(Transform::from_translation(Vec3::new(0.0, -0.02, 0.0))),
            )
            .unwrap();
        let _bbox = scene
            .add_child(
                helper_node,
                SceneNode::mesh("BoundingBox", helper_mesh, helper_id),
            )
            .unwrap();

        let objects = vec![
            DemoObject {
                node_id: cube,
                material_id: pbr_id,
                name: "Cube",
                material_name: "lab blue PBR",
            },
            DemoObject {
                node_id: sphere,
                material_id: toon_id,
                name: "Sphere",
                material_name: "Toon",
            },
            DemoObject {
                node_id: torus,
                material_id: physical_id,
                name: "Torus",
                material_name: "Physical",
            },
        ];

        let orbit = default_orbit();
        let mut camera = PerspectiveCamera::default();
        camera.aspect = width as f32 / height.max(1) as f32;
        orbit.apply_to_perspective(&mut camera);

        let input = InputState::new(ViewportMetrics::new(
            Vec2::new(width as f32, height as f32),
            1.0,
        ));

        let pulse_track = ScalarTrack::tween(0.0, 1.0, 1.8);

        Self {
            scene,
            camera,
            orbit,
            input,
            geometries,
            raycaster: Raycaster::new(),
            objects,
            helper_node,
            animated_node: torus,
            pulse_track,
            last_timestamp_ms: None,
            fps: 0.0,
            paused: false,
            helpers_visible: true,
            wireframe_enabled: false,
            bloom_enabled: false,
            ssao_enabled: false,
            selected_node: None,
            selected_name: String::from("None"),
            selected_distance: 0.0,
            active_material: String::from("None"),
            transform_mode: TransformMode::Translate,
        }
    }

    fn tick(&mut self, timestamp_ms: f64) {
        let dt = if let Some(prev) = self.last_timestamp_ms {
            ((timestamp_ms - prev) as f32 / 1000.0).clamp(0.0, 0.1)
        } else {
            0.0
        };
        self.last_timestamp_ms = Some(timestamp_ms);

        if !self.paused {
            self.animate_lab(dt);
            self.orbit.apply_to_perspective(&mut self.camera);
            let frames = (dt * 1000.0).max(1.0);
            self.fps = self.fps * 0.95 + (1000.0 / frames) * 0.05;
        }
    }

    fn animate_lab(&mut self, dt: f32) {
        self.pulse_track.update(dt);
        let s = self.pulse_track.value();
        let scale = 0.9 + 0.1 * s;
        if let Some(node) = self.scene.get_mut(self.animated_node) {
            node.transform = node.transform.scale_by(Vec3::new(scale, scale, scale));
        }
        if self.pulse_track.is_complete() {
            self.pulse_track = ScalarTrack::tween(0.0, 1.0, 1.8);
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.camera.aspect = width as f32 / height.max(1) as f32;
        self.input.viewport = ViewportMetrics::new(Vec2::new(width as f32, height as f32), 1.0);
    }

    fn on_pointer_move(&mut self, x: f32, y: f32) {
        self.input.on_pointer_move(Vec2::new(x, y));
    }

    fn on_pointer_down(&mut self, button: i16, x: f32, y: f32) {
        self.input.on_pointer_move(Vec2::new(x, y));
        if let Some(button) = pointer_button_from_dom(button) {
            self.input.on_pointer_down(button);
        }
    }

    fn on_pointer_up(&mut self, button: i16, x: f32, y: f32, width: f32, height: f32) {
        self.input.on_pointer_move(Vec2::new(x, y));
        if let Some(button) = pointer_button_from_dom(button) {
            self.input.on_pointer_up(button);
        }
        self.pick_at(x, y, width, height);
    }

    fn on_wheel(&mut self, delta_y: f32) {
        self.input.on_scroll(delta_y.signum() * 0.12);
    }

    fn on_key_down(&mut self, code: &str) {
        if let Some(code) = key_code_from_dom(code) {
            self.input.on_key_down(code);
        }
    }

    fn on_key_up(&mut self, code: &str) {
        if let Some(code) = key_code_from_dom(code) {
            self.input.on_key_up(code);
        }
    }

    fn on_touch(&mut self, id: u64, phase: u8, x: f32, y: f32, pressure: f32) {
        if let Some(phase) = touch_phase_from_dom(phase) {
            let _ = self
                .input
                .on_touch(TouchId(id), phase, Vec2::new(x, y), pressure);
        }
    }

    fn set_pointer_locked(&mut self, locked: bool) {
        self.input.set_pointer_locked(locked);
    }

    fn on_pointer_motion(&mut self, delta_x: f32, delta_y: f32) {
        self.input.on_pointer_motion(Vec2::new(delta_x, delta_y));
    }

    fn set_gamepad_connected(&mut self, index: u8, connected: bool) {
        let _ = self
            .input
            .set_gamepad_connected(GamepadId(index), connected);
    }

    fn set_gamepad_axis(&mut self, index: u8, axis: u8, value: f32) {
        if let Some(axis) = gamepad_axis_from_standard(axis) {
            let _ = self.input.set_gamepad_axis(GamepadId(index), axis, value);
        }
    }

    fn set_gamepad_button(&mut self, index: u8, button: u8, value: f32) {
        if let Some(button) = gamepad_button_from_standard(button) {
            let _ = self
                .input
                .set_gamepad_button(GamepadId(index), button, value);
        }
    }

    fn set_transform_mode(&mut self, mode: &str) {
        self.transform_mode = match mode {
            "rotate" => TransformMode::Rotate,
            "scale" => TransformMode::Scale,
            _ => TransformMode::Translate,
        };
    }

    fn transform_mode_label(&self) -> &'static str {
        match self.transform_mode {
            TransformMode::Translate => "translate",
            TransformMode::Rotate => "rotate",
            TransformMode::Scale => "scale",
        }
    }

    fn inspector_snapshot_json(&self) -> String {
        serde_json::to_string(&self.scene.inspector_snapshot())
            .unwrap_or_else(|_| String::from("{\"roots\":[]}"))
    }

    fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    fn paused(&self) -> bool {
        self.paused
    }

    fn set_helpers_visible(&mut self, visible: bool) {
        self.helpers_visible = visible;
        if let Some(node) = self.scene.get_mut(self.helper_node) {
            node.visible = visible;
        }
    }

    fn helpers_visible(&self) -> bool {
        self.helpers_visible
    }

    fn set_wireframe_enabled(&mut self, enabled: bool) {
        self.wireframe_enabled = enabled;
        let wireframe_id = MaterialId::new(6);
        for object in &self.objects {
            if let Some(node) = self.scene.get_mut(object.node_id)
                && let NodeKind::Mesh { material_id, .. } = &mut node.kind
            {
                *material_id = if enabled {
                    wireframe_id
                } else {
                    object.material_id
                };
            }
        }
    }

    fn wireframe_enabled(&self) -> bool {
        self.wireframe_enabled
    }

    fn set_bloom_enabled(&mut self, enabled: bool) {
        self.bloom_enabled = enabled;
    }

    fn bloom_enabled(&self) -> bool {
        self.bloom_enabled
    }

    fn set_ssao_enabled(&mut self, enabled: bool) {
        self.ssao_enabled = enabled;
    }

    fn ssao_enabled(&self) -> bool {
        self.ssao_enabled
    }

    fn reset_camera(&mut self) {
        self.orbit = default_orbit();
        self.orbit.apply_to_perspective(&mut self.camera);
    }

    fn fps(&self) -> f32 {
        self.fps
    }

    fn selected_node_name(&self) -> String {
        self.selected_name.clone()
    }

    fn selected_node_id(&self) -> u64 {
        self.selected_node.map_or(0, NodeId::get)
    }

    fn raycast_distance(&self) -> f32 {
        self.selected_distance
    }

    fn active_material(&self) -> String {
        self.active_material.clone()
    }

    fn pick_at(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let ndc = Vec2::new((x / width) * 2.0 - 1.0, 1.0 - (y / height) * 2.0);
        self.scene.update_world_transforms();
        if self
            .raycaster
            .build_bvh(&self.scene, &self.geometries)
            .is_err()
        {
            self.clear_selection();
            return;
        }
        let ray = Raycaster::from_camera_ndc(&self.camera, ndc);
        let Some(hit) = self.raycaster.cast_ray(ray, &self.scene, &self.geometries) else {
            self.clear_selection();
            return;
        };
        self.selected_node = Some(hit.node_id);
        let _ = self.scene.select(hit.node_id, SelectionMode::Replace);
        self.selected_distance = hit.distance;
        self.selected_name = self
            .scene
            .get(hit.node_id)
            .map_or_else(|| String::from("Unknown"), |node| node.name.clone());
        self.active_material = self
            .objects
            .iter()
            .find(|object| object.node_id == hit.node_id)
            .map_or_else(
                || String::from("Unknown"),
                |object| String::from(object.material_name),
            );
    }

    fn clear_selection(&mut self) {
        self.scene.clear_selection();
        self.selected_node = None;
        self.selected_distance = 0.0;
        self.selected_name = String::from("None");
        self.active_material = String::from("None");
    }

    fn active_feature_flags(&self) -> String {
        format!(
            "backend=wgpu, helpers={}, wireframe={}, bloom={}, ssao={}, textures=true, materials=true, lights=true, raycaster=true, animato=true",
            self.helpers_visible, self.wireframe_enabled, self.bloom_enabled, self.ssao_enabled,
        )
    }
}

fn generated_lab(renderer: &mut Renderer, width: u32, height: u32) -> Result<LabRuntime, JsValue> {
    let lab = LabRuntime::new(width, height);
    register_lab_assets_wgpu(renderer, &lab.geometries)?;
    Ok(lab)
}

fn register_lab_assets_wgpu(
    renderer: &mut Renderer,
    geometries: &BTreeMap<MeshId, Geometry>,
) -> Result<(), JsValue> {
    let pbr_id = MaterialId::new(1);
    let toon_id = MaterialId::new(2);
    let physical_id = MaterialId::new(3);
    let floor_id = MaterialId::new(4);
    let helper_id = MaterialId::new(5);
    let wireframe_id = MaterialId::new(6);

    for (mesh_id, geometry) in geometries {
        renderer
            .register_mesh(*mesh_id, geometry)
            .map_err(js_error)?;
    }

    renderer
        .register_pbr_material(
            pbr_id,
            &PbrMaterial::new()
                .named("lab blue PBR")
                .albedo(Color::from_hex(0x4EA1FF))
                .metallic_roughness(0.18, 0.38),
        )
        .map_err(js_error)?;
    let mut toon = ToonMaterial::new().steps(4).outline(0.025, Color::BLACK);
    toon.color = Color::from_hex(0xFFCC66);
    renderer
        .register_toon_material(toon_id, &toon)
        .map_err(js_error)?;
    renderer
        .register_physical_material(
            physical_id,
            &PhysicalMaterial::new()
                .base(
                    PbrMaterial::new()
                        .albedo(Color::from_hex(0xD970FF))
                        .metallic_roughness(0.55, 0.25),
                )
                .clearcoat(0.65, 0.16),
        )
        .map_err(js_error)?;
    renderer
        .register_lambert_material(
            floor_id,
            &LambertMaterial::new().color(Color::from_hex(0x2D3446)),
        )
        .map_err(js_error)?;
    renderer
        .register_unlit_material(
            helper_id,
            &UnlitMaterial::new().color(Color::from_hex(0xA7F3D0)),
        )
        .map_err(js_error)?;
    renderer
        .register_wireframe_material(
            wireframe_id,
            &WireframeMaterial {
                color: Color::from_hex(0xE8F0FF),
                opacity: 0.85,
                line_width: 1.0,
                double_sided: true,
            },
        )
        .map_err(js_error)?;

    renderer
        .register_directional_light(
            LightId::new(1),
            DirectionalLight::new(Vec3::new(-0.45, -0.85, -0.25), Color::WHITE, 3.2),
        )
        .map_err(js_error)?;
    renderer
        .register_point_light(
            LightId::new(2),
            PointLight::new(Color::from_hex(0x66CCFF), 1.6, 5.0),
        )
        .map_err(js_error)?;
    Ok(())
}

fn should_try_webgpu() -> bool {
    let Some(window) = window() else {
        return false;
    };
    let user_agent = window
        .navigator()
        .user_agent()
        .unwrap_or_default()
        .to_lowercase();
    if user_agent.contains("firefox") {
        return false;
    }
    let is_safari = user_agent.contains("safari")
        && !user_agent.contains("chrome")
        && !user_agent.contains("chromium")
        && !user_agent.contains("edg/");
    if is_safari {
        return false;
    }
    let navigator = JsValue::from(window.navigator());
    Reflect::has(&navigator, &JsValue::from_str("gpu")).unwrap_or(false)
}

fn webgl_context(canvas: &HtmlCanvasElement) -> Result<WebGlBackendContext, JsValue> {
    if let Some(context) = canvas.get_context("webgl2")? {
        return context
            .dyn_into::<WebGl2RenderingContext>()
            .map(WebGlBackendContext::WebGl2)
            .map_err(|_| JsValue::from_str("canvas context is not a WebGl2RenderingContext"));
    }

    canvas
        .get_context("webgl")?
        .or_else(|| canvas.get_context("experimental-webgl").ok().flatten())
        .ok_or_else(|| JsValue::from_str("WebGL is not available for this canvas"))?
        .dyn_into::<WebGlRenderingContext>()
        .map(WebGlBackendContext::WebGl1)
        .map_err(|_| JsValue::from_str("canvas context is not a WebGLRenderingContext"))
}

fn create_webgl_program(gl: &WebGlBackendContext) -> Result<WebGlProgramState, JsValue> {
    let vertex = compile_shader(
        gl,
        WebGlRenderingContext::VERTEX_SHADER,
        WEBGL_VERTEX_SHADER,
    )?;
    let fragment = compile_shader(
        gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        WEBGL_FRAGMENT_SHADER,
    )?;
    let program = link_program(gl, &vertex, &fragment)?;
    gl.use_program(Some(&program));
    let position_attrib = attrib_location(gl, &program, "a_position")?;
    let normal_attrib = attrib_location(gl, &program, "a_normal")?;
    let uv_attrib = attrib_location(gl, &program, "a_uv")?;
    let color_attrib = attrib_location(gl, &program, "a_color")?;
    Ok(WebGlProgramState {
        view_projection_uniform: uniform_location(gl, &program, "u_view_projection")?,
        model_uniform: uniform_location(gl, &program, "u_model")?,
        material_uniform: uniform_location(gl, &program, "u_material")?,
        light_direction_uniform: uniform_location(gl, &program, "u_light_direction")?,
        point_position_range_uniform: uniform_location(gl, &program, "u_point_position_range")?,
        point_color_uniform: uniform_location(gl, &program, "u_point_color")?,
        texture_uniform: uniform_location(gl, &program, "u_texture")?,
        use_texture_uniform: uniform_location(gl, &program, "u_use_texture")?,
        material_model_uniform: uniform_location(gl, &program, "u_material_model")?,
        metallic_roughness_uniform: uniform_location(gl, &program, "u_metallic_roughness")?,
        unlit_uniform: uniform_location(gl, &program, "u_unlit")?,
        bloom_uniform: uniform_location(gl, &program, "u_bloom")?,
        ssao_uniform: uniform_location(gl, &program, "u_ssao")?,
        program,
        position_attrib,
        normal_attrib,
        uv_attrib,
        color_attrib,
    })
}

fn compile_shader(
    gl: &WebGlBackendContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, JsValue> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| JsValue::from_str("failed to create WebGL shader"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(JsValue::from_str(
            &gl.get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("WebGL shader compilation failed")),
        ))
    }
}

fn link_program(
    gl: &WebGlBackendContext,
    vertex: &WebGlShader,
    fragment: &WebGlShader,
) -> Result<WebGlProgram, JsValue> {
    let program = gl
        .create_program()
        .ok_or_else(|| JsValue::from_str("failed to create WebGL program"))?;
    gl.attach_shader(&program, vertex);
    gl.attach_shader(&program, fragment);
    gl.link_program(&program);
    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(JsValue::from_str(
            &gl.get_program_info_log(&program)
                .unwrap_or_else(|| String::from("WebGL program link failed")),
        ))
    }
}

fn attrib_location(
    gl: &WebGlBackendContext,
    program: &WebGlProgram,
    name: &str,
) -> Result<u32, JsValue> {
    let location = gl.get_attrib_location(program, name);
    if location < 0 {
        Err(JsValue::from_str(&format!(
            "WebGL attribute `{name}` was not found"
        )))
    } else {
        Ok(location as u32)
    }
}

fn uniform_location(
    gl: &WebGlBackendContext,
    program: &WebGlProgram,
    name: &str,
) -> Result<WebGlUniformLocation, JsValue> {
    gl.get_uniform_location(program, name)
        .ok_or_else(|| JsValue::from_str(&format!("WebGL uniform `{name}` was not found")))
}

const WEBGL_VERTEX_SHADER: &str = r#"
attribute vec3 a_position;
attribute vec3 a_normal;
attribute vec2 a_uv;
attribute vec4 a_color;

uniform mat4 u_view_projection;
uniform mat4 u_model;

varying vec3 v_normal;
varying vec3 v_world_position;
varying vec2 v_uv;
varying vec4 v_color;

void main() {
    vec4 world = u_model * vec4(a_position, 1.0);
    gl_Position = u_view_projection * world;
    v_normal = mat3(u_model) * a_normal;
    v_world_position = world.xyz;
    v_uv = a_uv;
    v_color = a_color;
}
"#;

const WEBGL_FRAGMENT_SHADER: &str = r#"
precision highp float;

varying vec3 v_normal;
varying vec3 v_world_position;
varying vec2 v_uv;
varying vec4 v_color;

uniform vec4 u_material;
uniform float u_material_model;
uniform vec4 u_metallic_roughness;
uniform float u_unlit;
uniform vec3 u_light_direction;
uniform vec4 u_point_position_range;
uniform vec4 u_point_color;
uniform sampler2D u_texture;
uniform float u_use_texture;
uniform float u_bloom;
uniform float u_ssao;

void main() {
    vec3 N = normalize(v_normal);
    vec3 L = normalize(-u_light_direction);

    vec4 base_color = u_use_texture > 0.5
        ? texture2D(u_texture, v_uv) * u_material
        : u_material;

    vec3 albedo = base_color.rgb;
    float metallic = u_metallic_roughness.x;
    float roughness = u_metallic_roughness.y;
    float clearcoat = u_metallic_roughness.z;

    if (u_unlit > 0.5) {
        gl_FragColor = vec4(albedo, base_color.a);
        return;
    }

    float NdotL = max(dot(N, L), 0.0);

    vec3 view_dir = normalize(-v_world_position);
    vec3 half_dir = normalize(L + view_dir);
    float NdotH = max(dot(N, half_dir), 0.0);

    float diffuse = NdotL;
    float specular = pow(NdotH, mix(16.0, 128.0, 1.0 - roughness));
    float fresnel = pow(1.0 - max(dot(N, view_dir), 0.0), 3.0);

    vec3 dielectric_specular = vec3(specular * 0.5);
    vec3 metal_specular = albedo * specular;

    vec3 spec = mix(dielectric_specular, metal_specular, metallic);
    vec3 diffuse_color = albedo * (1.0 - metallic) * diffuse;

    vec3 point_dir = u_point_position_range.xyz - v_world_position;
    float point_dist = length(point_dir);
    float point_attenuation = 1.0 - smoothstep(0.0, u_point_position_range.w, point_dist);
    float point_NdotL = max(dot(N, normalize(point_dir)), 0.0);

    vec3 color = diffuse_color + spec + fresnel * 0.04;
    color += point_point_color.rgb * point_NdotL * point_attenuation * u_point_color.a;

    color += color * clearcoat * 0.12;

    if (u_bloom > 0.5) {
        float brightness = dot(color, vec3(0.2126, 0.7152, 0.0722));
        color += color * smoothstep(0.8, 1.6, brightness) * 0.35;
    }
    if (u_ssao > 0.5) {
        color *= 0.82 + 0.18 * NdotL;
    }

    gl_FragColor = vec4(color, base_color.a);
}
"#;

fn js_error(error: ScenixError) -> JsValue {
    JsValue::from_str(&error.to_string())
}

fn with_color(mut geometry: Geometry, color: Color) -> Geometry {
    geometry.colors = vec![color; geometry.positions.len()];
    geometry
}

fn helper_geometry() -> Geometry {
    let mut positions = Vec::new();
    let mut colors = Vec::new();

    let axis_len = 2.0f32;
    let x_color = Color::from_hex(0xFF4444);
    let y_color = Color::from_hex(0x44FF44);
    let z_color = Color::from_hex(0x4444FF);

    positions.push(Vec3::new(0.0, 0.0, 0.0));
    positions.push(Vec3::new(axis_len, 0.0, 0.0));
    colors.push(x_color);
    colors.push(x_color);

    positions.push(Vec3::new(0.0, 0.0, 0.0));
    positions.push(Vec3::new(0.0, axis_len, 0.0));
    colors.push(y_color);
    colors.push(y_color);

    positions.push(Vec3::new(0.0, 0.0, 0.0));
    positions.push(Vec3::new(0.0, 0.0, axis_len));
    colors.push(z_color);
    colors.push(z_color);

    let grid_color = Color::from_hex(0x556677);
    let grid_half = 3.0f32;
    let grid_step = 0.5f32;
    let mut t = -grid_half;
    while t <= grid_half + 0.001 {
        positions.push(Vec3::new(t, 0.0, -grid_half));
        positions.push(Vec3::new(t, 0.0, grid_half));
        positions.push(Vec3::new(-grid_half, 0.0, t));
        positions.push(Vec3::new(grid_half, 0.0, t));
        for _ in 0..4 {
            colors.push(grid_color);
        }
        t += grid_step;
    }

    Geometry {
        positions,
        normals: Vec::new(),
        uvs: Vec::new(),
        uvs2: Vec::new(),
        colors,
        indices: Vec::new(),
        tangents: Vec::new(),
    }
}

fn default_orbit() -> OrbitController {
    let mut orbit = OrbitController::new(Vec3::new(0.0, 0.3, 0.0), 5.0);
    orbit.theta = 0.4;
    orbit.phi = 0.35;
    orbit.rotate_sensitivity = 0.005;
    orbit.zoom_sensitivity = 0.12;
    orbit.damping = 0.92;
    orbit
}

#[wasm_bindgen]
impl WebRenderer {
    /// 为 `canvas` 创建渲染器并注册生成的 Scenix Engine Lab 场景。
    pub async fn new(canvas: HtmlCanvasElement) -> Result<WebRenderer, JsValue> {
        crate::set_panic_hook();
        let (width, height) = canvas_size(&canvas);
        let config = RendererConfig::new(width, height);
        let mut renderer = Renderer::new(wgpu::SurfaceTarget::Canvas(canvas), config)
            .await
            .map_err(js_error)?;
        let lab = generated_lab(&mut renderer, width, height)?;
        Ok(Self { renderer, lab })
    }

    /// 渲染一帧。`timestamp_ms` 应来自 `requestAnimationFrame`。
    pub fn tick(&mut self, timestamp_ms: f64) -> Result<(), JsValue> {
        self.lab.tick(timestamp_ms);
        self.renderer
            .render(&self.lab.scene, &self.lab.camera)
            .map_err(js_error)?;
        Ok(())
    }

    /// 调整画布和渲染器大小。零尺寸将被钳位为一像素。
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        let (width, height) = clamp_canvas_size(width, height);
        self.lab.resize(width, height);
        self.renderer.resize(width, height).map_err(js_error)?;
        Ok(())
    }

    /// 更新指针位置。
    pub fn on_pointer_move(&mut self, x: f32, y: f32) {
        self.lab.on_pointer_move(x, y);
    }

    /// 更新指针位置和按下状态。
    pub fn on_pointer_down(&mut self, button: i16, x: f32, y: f32) {
        self.lab.on_pointer_down(button, x, y);
    }

    /// 更新指针位置、按下状态和选中对象。
    pub fn on_pointer_up(&mut self, button: i16, x: f32, y: f32) {
        self.lab.on_pointer_up(
            button,
            x,
            y,
            self.renderer.config().width.max(1) as f32,
            self.renderer.config().height.max(1) as f32,
        );
    }

    /// 前后移动生成的轨道相机。
    pub fn on_wheel(&mut self, delta_y: f32) {
        self.lab.on_wheel(delta_y);
    }

    /// 当 DOM 键映射到 scenekit 输入时标记为按下。
    pub fn on_key_down(&mut self, code: &str) {
        self.lab.on_key_down(code);
    }

    /// 当 DOM 键映射到 scenekit 输入时标记为释放。
    pub fn on_key_up(&mut self, code: &str) {
        self.lab.on_key_up(code);
    }

    /// 转发紧凑触摸事件（`0=start, 1=move, 2=end, 3=cancel`）。
    pub fn on_touch(&mut self, id: u64, phase: u8, x: f32, y: f32, pressure: f32) {
        self.lab.on_touch(id, phase, x, y, pressure);
    }

    /// 更新浏览器指针锁定所有权。
    pub fn set_pointer_locked(&mut self, locked: bool) {
        self.lab.set_pointer_locked(locked);
    }

    /// 转发锁定时的相对指针移动。
    pub fn on_pointer_motion(&mut self, delta_x: f32, delta_y: f32) {
        self.lab.on_pointer_motion(delta_x, delta_y);
    }

    /// 更新一个标准游戏手柄连接槽位。
    pub fn set_gamepad_connected(&mut self, index: u8, connected: bool) {
        self.lab.set_gamepad_connected(index, connected);
    }

    /// 更新一个标准游戏手柄轴（`0..=3`）。
    pub fn set_gamepad_axis(&mut self, index: u8, axis: u8, value: f32) {
        self.lab.set_gamepad_axis(index, axis, value);
    }

    /// 更新一个标准游戏手柄按钮（`0..=16`）。
    pub fn set_gamepad_button(&mut self, index: u8, button: u8, value: f32) {
        self.lab.set_gamepad_button(index, button, value);
    }

    /// 启用或暂停动画。
    pub fn set_paused(&mut self, paused: bool) {
        self.lab.set_paused(paused);
    }

    /// 返回动画是否已暂停。
    pub fn paused(&self) -> bool {
        self.lab.paused()
    }

    /// 显示或隐藏辅助几何体。
    pub fn set_helpers_visible(&mut self, visible: bool) {
        self.lab.set_helpers_visible(visible);
    }

    /// 返回辅助几何体是否可见。
    pub fn helpers_visible(&self) -> bool {
        self.lab.helpers_visible()
    }

    /// 在可选对象上启用或禁用线框预览材质。
    pub fn set_wireframe_enabled(&mut self, enabled: bool) {
        self.lab.set_wireframe_enabled(enabled);
    }

    /// 返回是否启用了线框预览。
    pub fn wireframe_enabled(&self) -> bool {
        self.lab.wireframe_enabled()
    }

    /// 存储 Bloom UI 开关。当前浏览器包装器在功能标志中报告它。
    pub fn set_bloom_enabled(&mut self, enabled: bool) {
        self.lab.set_bloom_enabled(enabled);
    }

    /// 返回 Bloom UI 开关是否启用。
    pub fn bloom_enabled(&self) -> bool {
        self.lab.bloom_enabled()
    }

    /// 存储 SSAO UI 开关。当前浏览器包装器在功能标志中报告它。
    pub fn set_ssao_enabled(&mut self, enabled: bool) {
        self.lab.set_ssao_enabled(enabled);
    }

    /// 返回 SSAO UI 开关是否启用。
    pub fn ssao_enabled(&self) -> bool {
        self.lab.ssao_enabled()
    }

    /// 恢复默认轨道相机。
    pub fn reset_camera(&mut self) {
        self.lab.reset_camera();
    }

    /// 返回生成的场景名称。
    pub fn scene_name(&self) -> String {
        String::from("Scenix Engine Lab")
    }

    /// 返回最近的每秒帧数估计值。
    pub fn fps(&self) -> f32 {
        self.lab.fps()
    }

    /// 返回选中的场景节点名称。
    pub fn selected_node_name(&self) -> String {
        self.lab.selected_node_name()
    }

    /// 返回原始选中的节点 ID，未选中时返回零。
    pub fn selected_node_id(&self) -> u64 {
        self.lab.selected_node_id()
    }

    /// 设置活动编辑器变换模式。
    pub fn set_transform_mode(&mut self, mode: &str) {
        self.lab.set_transform_mode(mode);
    }

    /// 返回 `translate`、`rotate` 或 `scale`。
    pub fn transform_mode(&self) -> String {
        String::from(self.lab.transform_mode_label())
    }

    /// 序列化当前场景检查器快照。
    pub fn inspector_snapshot_json(&self) -> String {
        self.lab.inspector_snapshot_json()
    }

    /// 返回当前射线投射命中距离。
    pub fn raycast_distance(&self) -> f32 {
        self.lab.raycast_distance()
    }

    /// 返回当前选中的材质标签。
    pub fn active_material(&self) -> String {
        self.lab.active_material()
    }

    /// 以紧凑字符串形式返回活动浏览器演示功能标志。
    pub fn active_feature_flags(&self) -> String {
        self.lab.active_feature_flags()
    }
}

/// 为没有可用 WebGPU 的浏览器提供 WebGL2 优先回退渲染器。
#[wasm_bindgen]
pub struct WebGlRenderer {
    canvas: HtmlCanvasElement,
    gl: WebGlBackendContext,
    capability: WebGlCapabilityLevel,
    program: WebGlProgramState,
    lab: LabRuntime,
    meshes: BTreeMap<MeshId, WebGlMesh>,
    materials: BTreeMap<MaterialId, WebGlMaterial>,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl WebGlRenderer {
    /// 为生成的 Scenix Engine Lab 场景创建 WebGL 渲染器。
    pub async fn new(canvas: HtmlCanvasElement) -> Result<WebGlRenderer, JsValue> {
        crate::set_panic_hook();
        let (width, height) = canvas_size(&canvas);
        let gl = webgl_context(&canvas)?;
        let capability = gl.capability();
        let program = create_webgl_program(&gl)?;
        let lab = LabRuntime::new(width, height);
        let mut renderer = Self {
            canvas,
            gl,
            capability,
            program,
            lab,
            meshes: BTreeMap::new(),
            materials: BTreeMap::new(),
            width,
            height,
        };
        renderer.resize(width, height)?;
        renderer.register_lab_assets()?;
        Ok(renderer)
    }

    /// 渲染一帧 WebGL 帧。
    pub fn tick(&mut self, timestamp_ms: f64) -> Result<(), JsValue> {
        self.lab.tick(timestamp_ms);
        self.draw();
        Ok(())
    }

    /// 调整 WebGL 视口大小。零尺寸将被钳位为一像素。
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        let (width, height) = clamp_canvas_size(width, height);
        self.width = width;
        self.height = height;
        self.lab.resize(width, height);
        let ratio = window().map_or(1.0, |window| window.device_pixel_ratio().max(1.0));
        let pixel_width = (width as f64 * ratio).round().max(1.0) as u32;
        let pixel_height = (height as f64 * ratio).round().max(1.0) as u32;
        self.canvas.set_width(pixel_width);
        self.canvas.set_height(pixel_height);
        self.gl
            .viewport(0, 0, pixel_width as i32, pixel_height as i32);
        Ok(())
    }

    /// 更新指针位置。
    pub fn on_pointer_move(&mut self, x: f32, y: f32) {
        self.lab.on_pointer_move(x, y);
    }

    /// 更新指针按下状态。
    pub fn on_pointer_down(&mut self, button: i16, x: f32, y: f32) {
        self.lab.on_pointer_down(button, x, y);
    }

    /// 更新指针释放状态并运行拾取。
    pub fn on_pointer_up(&mut self, button: i16, x: f32, y: f32) {
        self.lab.on_pointer_up(
            button,
            x,
            y,
            self.width.max(1) as f32,
            self.height.max(1) as f32,
        );
    }

    /// 更新滚轮回退输入。
    pub fn on_wheel(&mut self, delta_y: f32) {
        self.lab.on_wheel(delta_y);
    }

    /// 更新按键按下状态。
    pub fn on_key_down(&mut self, code: &str) {
        self.lab.on_key_down(code);
    }

    /// 更新按键释放状态。
    pub fn on_key_up(&mut self, code: &str) {
        self.lab.on_key_up(code);
    }

    /// 转发紧凑触摸事件。
    pub fn on_touch(&mut self, id: u64, phase: u8, x: f32, y: f32, pressure: f32) {
        self.lab.on_touch(id, phase, x, y, pressure);
    }

    /// 更新指针锁定所有权。
    pub fn set_pointer_locked(&mut self, locked: bool) {
        self.lab.set_pointer_locked(locked);
    }

    /// 转发相对指针移动。
    pub fn on_pointer_motion(&mut self, delta_x: f32, delta_y: f32) {
        self.lab.on_pointer_motion(delta_x, delta_y);
    }

    /// 更新一个标准游戏手柄连接槽位。
    pub fn set_gamepad_connected(&mut self, index: u8, connected: bool) {
        self.lab.set_gamepad_connected(index, connected);
    }

    /// 更新一个标准游戏手柄轴。
    pub fn set_gamepad_axis(&mut self, index: u8, axis: u8, value: f32) {
        self.lab.set_gamepad_axis(index, axis, value);
    }

    /// 更新一个标准游戏手柄按钮。
    pub fn set_gamepad_button(&mut self, index: u8, button: u8, value: f32) {
        self.lab.set_gamepad_button(index, button, value);
    }

    /// 启用或暂停动画。
    pub fn set_paused(&mut self, paused: bool) {
        self.lab.set_paused(paused);
    }

    /// 返回动画是否已暂停。
    pub fn paused(&self) -> bool {
        self.lab.paused()
    }

    /// 显示或隐藏辅助几何体。
    pub fn set_helpers_visible(&mut self, visible: bool) {
        self.lab.set_helpers_visible(visible);
    }

    /// 返回辅助几何体是否可见。
    pub fn helpers_visible(&self) -> bool {
        self.lab.helpers_visible()
    }

    /// 启用或禁用线框预览。
    pub fn set_wireframe_enabled(&mut self, enabled: bool) {
        self.lab.set_wireframe_enabled(enabled);
    }

    /// 返回是否启用了线框预览。
    pub fn wireframe_enabled(&self) -> bool {
        self.lab.wireframe_enabled()
    }

    /// 存储 Bloom UI 开关。
    pub fn set_bloom_enabled(&mut self, enabled: bool) {
        self.lab.set_bloom_enabled(enabled);
    }

    /// 返回 Bloom UI 开关是否启用。
    pub fn bloom_enabled(&self) -> bool {
        self.lab.bloom_enabled()
    }

    /// 存储 SSAO UI 开关。
    pub fn set_ssao_enabled(&mut self, enabled: bool) {
        self.lab.set_ssao_enabled(enabled);
    }

    /// 返回 SSAO UI 开关是否启用。
    pub fn ssao_enabled(&self) -> bool {
        self.lab.ssao_enabled()
    }

    /// 恢复默认轨道相机。
    pub fn reset_camera(&mut self) {
        self.lab.reset_camera();
    }

    /// 返回生成的场景名称。
    pub fn scene_name(&self) -> String {
        String::from("Scenix Engine Lab")
    }

    /// 返回最近的每秒帧数估计值。
    pub fn fps(&self) -> f32 {
        self.lab.fps()
    }

    /// 返回选中的场景节点名称。
    pub fn selected_node_name(&self) -> String {
        self.lab.selected_node_name()
    }

    /// 返回选中的节点 ID，未选中时返回零。
    pub fn selected_node_id(&self) -> u64 {
        self.lab.selected_node_id()
    }

    /// 设置活动编辑器变换模式。
    pub fn set_transform_mode(&mut self, mode: &str) {
        self.lab.set_transform_mode(mode);
    }

    /// 返回活动编辑器变换模式。
    pub fn transform_mode(&self) -> String {
        String::from(self.lab.transform_mode_label())
    }

    /// 序列化当前场景检查器快照。
    pub fn inspector_snapshot_json(&self) -> String {
        self.lab.inspector_snapshot_json()
    }

    /// 返回当前射线投射命中距离。
    pub fn raycast_distance(&self) -> f32 {
        self.lab.raycast_distance()
    }

    /// 返回当前选中的材质标签。
    pub fn active_material(&self) -> String {
        self.lab.active_material()
    }

    /// 以紧凑字符串形式返回活动 WebGL 功能标志。
    pub fn active_feature_flags(&self) -> String {
        let shadows = match self.capability {
            WebGlCapabilityLevel::WebGl2 => "webgl2-soft",
            WebGlCapabilityLevel::WebGl1 => "fallback",
        };
        format!(
            "backend={}, parity={}, helpers={}, wireframe={}, bloom={}, ssao={}, textures=true, materials=true, lights=true, shadows={}, raycaster=true, animato=true",
            self.capability.label(),
            self.capability.parity_label(),
            self.lab.helpers_visible(),
            self.lab.wireframe_enabled(),
            self.lab.bloom_enabled(),
            self.lab.ssao_enabled(),
            shadows
        )
    }
}

impl WebGlRenderer {
    fn register_lab_assets(&mut self) -> Result<(), JsValue> {
        let geometries: Vec<(MeshId, Geometry)> = self
            .lab
            .geometries
            .iter()
            .map(|(id, geometry)| (*id, geometry.clone()))
            .collect();
        for (mesh_id, geometry) in geometries {
            self.register_mesh(mesh_id, &geometry)?;
        }
        self.register_pbr_material(
            MaterialId::new(1),
            &PbrMaterial::new()
                .named("lab blue PBR")
                .albedo(Color::from_hex(0x4EA1FF))
                .metallic_roughness(0.18, 0.38),
        )?;
        let mut toon = ToonMaterial::new().steps(4).outline(0.025, Color::BLACK);
        toon.color = Color::from_hex(0xFFCC66);
        self.register_toon_material(MaterialId::new(2), &toon)?;
        self.register_physical_material(
            MaterialId::new(3),
            &PhysicalMaterial::new()
                .base(
                    PbrMaterial::new()
                        .albedo(Color::from_hex(0xD970FF))
                        .metallic_roughness(0.55, 0.25),
                )
                .clearcoat(0.65, 0.16),
        )?;
        self.register_lambert_material(
            MaterialId::new(4),
            &LambertMaterial::new().color(Color::from_hex(0x2D3446)),
        )?;
        self.register_unlit_material(
            MaterialId::new(5),
            &UnlitMaterial::new().color(Color::from_hex(0xA7F3D0)),
        )?;
        self.register_wireframe_material(
            MaterialId::new(6),
            &WireframeMaterial {
                color: Color::from_hex(0xE8F0FF),
                opacity: 0.85,
                line_width: 1.0,
                double_sided: true,
            },
        )?;
        Ok(())
    }

    fn register_mesh(&mut self, mesh_id: MeshId, geometry: &Geometry) -> Result<(), JsValue> {
        if geometry.positions.len() > u16::MAX as usize {
            return Err(JsValue::from_str(
                "WebGL fallback supports up to 65535 vertices per mesh",
            ));
        }
        geometry
            .validate()
            .map_err(|error| JsValue::from_str(&error.to_string()))?;

        let vertex_count = geometry.positions.len();
        let mut vertices = Vec::with_capacity(vertex_count * 10);
        for index in 0..vertex_count {
            let position = geometry.positions[index];
            let normal = geometry.normals.get(index).copied().unwrap_or(Vec3::Y);
            let uv = geometry.uvs.get(index).copied().unwrap_or(Vec2::ZERO);
            let color = geometry.colors.get(index).copied().unwrap_or(Color::WHITE);
            vertices.extend_from_slice(&[
                position.x, position.y, position.z, normal.x, normal.y, normal.z, uv.x, uv.y,
                color.r, color.g, color.b, color.a,
            ]);
        }

        let indices: Vec<u16> = if geometry.indices.is_empty() {
            (0..vertex_count as u16).collect()
        } else {
            geometry.indices.iter().map(|index| *index as u16).collect()
        };
        let mut line_indices = Vec::with_capacity(indices.len() * 2);
        for triangle in indices.as_chunks::<3>().0 {
            line_indices.extend_from_slice(&[
                triangle[0],
                triangle[1],
                triangle[1],
                triangle[2],
                triangle[2],
                triangle[0],
            ]);
        }

        let vertex_buffer = self.create_array_buffer(&vertices)?;
        let index_buffer = self.create_element_buffer(&indices)?;
        let line_index_buffer = self.create_element_buffer(&line_indices)?;
        self.meshes.insert(
            mesh_id,
            WebGlMesh {
                vertex_buffer,
                index_buffer,
                line_index_buffer,
                index_count: indices.len() as i32,
                line_index_count: line_indices.len() as i32,
            },
        );
        Ok(())
    }

    fn register_pbr_material(
        &mut self,
        id: MaterialId,
        material: &PbrMaterial,
    ) -> Result<(), JsValue> {
        let texture = self.create_material_texture(material.albedo)?;
        self.materials.insert(
            id,
            WebGlMaterial {
                color: material.albedo,
                texture: Some(texture),
                model: WebGlMaterialModel::Pbr,
                metallic: material.metallic,
                roughness: material.roughness,
                clearcoat: 0.0,
                unlit: false,
                wireframe: false,
            },
        );
        Ok(())
    }

    fn register_physical_material(
        &mut self,
        id: MaterialId,
        material: &PhysicalMaterial,
    ) -> Result<(), JsValue> {
        let texture = self.create_material_texture(material.base.albedo)?;
        self.materials.insert(
            id,
            WebGlMaterial {
                color: material.base.albedo,
                texture: Some(texture),
                model: WebGlMaterialModel::Physical,
                metallic: material.base.metallic,
                roughness: material.base.roughness,
                clearcoat: material.clearcoat,
                unlit: false,
                wireframe: false,
            },
        );
        Ok(())
    }

    fn register_unlit_material(
        &mut self,
        id: MaterialId,
        material: &UnlitMaterial,
    ) -> Result<(), JsValue> {
        let texture = self.create_material_texture(material.color)?;
        self.materials.insert(
            id,
            WebGlMaterial {
                color: material.color,
                texture: Some(texture),
                model: WebGlMaterialModel::Unlit,
                metallic: 0.0,
                roughness: 1.0,
                clearcoat: 0.0,
                unlit: true,
                wireframe: false,
            },
        );
        Ok(())
    }

    fn register_lambert_material(
        &mut self,
        id: MaterialId,
        material: &LambertMaterial,
    ) -> Result<(), JsValue> {
        let texture = self.create_material_texture(material.color)?;
        self.materials.insert(
            id,
            WebGlMaterial {
                color: material.color,
                texture: Some(texture),
                model: WebGlMaterialModel::Lambert,
                metallic: 0.0,
                roughness: 1.0,
                clearcoat: 0.0,
                unlit: false,
                wireframe: false,
            },
        );
        Ok(())
    }

    fn register_toon_material(
        &mut self,
        id: MaterialId,
        material: &ToonMaterial,
    ) -> Result<(), JsValue> {
        let texture = self.create_material_texture(material.color)?;
        self.materials.insert(
            id,
            WebGlMaterial {
                color: material.color,
                texture: Some(texture),
                model: WebGlMaterialModel::Toon,
                metallic: 0.0,
                roughness: 0.82,
                clearcoat: 0.0,
                unlit: false,
                wireframe: false,
            },
        );
        Ok(())
    }

    fn register_wireframe_material(
        &mut self,
        id: MaterialId,
        material: &WireframeMaterial,
    ) -> Result<(), JsValue> {
        self.materials.insert(
            id,
            WebGlMaterial {
                color: Color::rgba(
                    material.color.r,
                    material.color.g,
                    material.color.b,
                    material.opacity.min(material.color.a),
                ),
                texture: None,
                model: WebGlMaterialModel::Unlit,
                metallic: 0.0,
                roughness: 1.0,
                clearcoat: 0.0,
                unlit: true,
                wireframe: true,
            },
        );
        Ok(())
    }

    fn create_material_texture(&self, color: Color) -> Result<WebGlTexture, JsValue> {
        let texture = self
            .gl
            .create_texture()
            .ok_or_else(|| JsValue::from_str("failed to create WebGL material texture"))?;
        let pixels = [
            (color.r.clamp(0.0, 1.0) * 255.0).round() as u8,
            (color.g.clamp(0.0, 1.0) * 255.0).round() as u8,
            (color.b.clamp(0.0, 1.0) * 255.0).round() as u8,
            (color.a.clamp(0.0, 1.0) * 255.0).round() as u8,
        ];
        self.gl
            .bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));
        self.gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MIN_FILTER,
            WebGlRenderingContext::LINEAR as i32,
        );
        self.gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MAG_FILTER,
            WebGlRenderingContext::LINEAR as i32,
        );
        self.gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_S,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        self.gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_T,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        self.gl.tex_image_2d_with_u8(
            WebGlRenderingContext::TEXTURE_2D,
            0,
            WebGlRenderingContext::RGBA as i32,
            1,
            1,
            0,
            WebGlRenderingContext::RGBA,
            WebGlRenderingContext::UNSIGNED_BYTE,
            Some(&pixels),
        )?;
        Ok(texture)
    }

    fn create_array_buffer(&self, values: &[f32]) -> Result<WebGlBuffer, JsValue> {
        let buffer = self
            .gl
            .create_buffer()
            .ok_or_else(|| JsValue::from_str("failed to create WebGL vertex buffer"))?;
        self.gl
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        let array = Float32Array::new_with_length(values.len() as u32);
        array.copy_from(values);
        self.gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            array.as_ref(),
            WebGlRenderingContext::STATIC_DRAW,
        );
        Ok(buffer)
    }

    fn create_element_buffer(&self, values: &[u16]) -> Result<WebGlBuffer, JsValue> {
        let buffer = self
            .gl
            .create_buffer()
            .ok_or_else(|| JsValue::from_str("failed to create WebGL index buffer"))?;
        self.gl
            .bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));
        let array = Uint16Array::new_with_length(values.len() as u32);
        array.copy_from(values);
        self.gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            array.as_ref(),
            WebGlRenderingContext::STATIC_DRAW,
        );
        Ok(buffer)
    }

    fn draw(&mut self) {
        self.gl.enable(WebGlRenderingContext::DEPTH_TEST);
        self.gl.depth_func(WebGlRenderingContext::LEQUAL);
        self.gl.disable(WebGlRenderingContext::CULL_FACE);
        let clear = if self.lab.ssao_enabled() { 0.025 } else { 0.04 };
        self.gl.clear_color(clear, clear * 1.6, clear * 2.5, 1.0);
        self.gl.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );
        self.gl.use_program(Some(&self.program.program));
        self.gl.uniform3f(
            Some(&self.program.light_direction_uniform),
            -0.45,
            -0.85,
            -0.25,
        );
        self.gl.uniform4f(
            Some(&self.program.point_position_range_uniform),
            2.0,
            2.1,
            1.4,
            5.0,
        );
        self.gl
            .uniform4f(Some(&self.program.point_color_uniform), 0.4, 0.8, 1.0, 1.6);
        self.gl.uniform1i(Some(&self.program.texture_uniform), 0);
        self.gl.uniform1f(
            Some(&self.program.bloom_uniform),
            if self.lab.bloom_enabled() { 1.0 } else { 0.0 },
        );
        self.gl.uniform1f(
            Some(&self.program.ssao_uniform),
            if self.lab.ssao_enabled() { 1.0 } else { 0.0 },
        );

        let view_projection = self.lab.camera.view_projection().to_cols_array();
        self.gl.uniform_matrix4fv_with_f32_array(
            Some(&self.program.view_projection_uniform),
            false,
            &view_projection,
        );

        for node_id in self.lab.scene.iter_depth_first() {
            let Some(node) = self.lab.scene.get(node_id) else {
                continue;
            };
            if !node.visible {
                continue;
            }
            let (mesh_id, material_id) = match &node.kind {
                NodeKind::Mesh {
                    mesh_id,
                    material_id,
                } => (*mesh_id, *material_id),
                _ => continue,
            };
            let Some(mesh) = self.meshes.get(&mesh_id) else {
                continue;
            };
            let material = self.materials.get(&material_id);
            let color = material.map_or(Color::WHITE, |material| material.color);
            let unlit = material.is_none_or(|material| material.unlit || material.wireframe);
            let wireframe = material.is_some_and(|material| material.wireframe);
            let material_model = material.map_or(0.0, |material| match material.model {
                WebGlMaterialModel::Pbr => 0.0,
                WebGlMaterialModel::Physical => 1.0,
                WebGlMaterialModel::Toon => 2.0,
                WebGlMaterialModel::Lambert => 3.0,
                WebGlMaterialModel::Unlit => 4.0,
            });
            let metallic = material.map_or(0.0, |material| material.metallic);
            let roughness = material.map_or(1.0, |material| material.roughness);
            let clearcoat = material.map_or(0.0, |material| material.clearcoat);
            let model = self
                .lab
                .scene
                .world_matrix(node_id)
                .unwrap_or(scenekit_math::Mat4::IDENTITY)
                .to_cols_array();
            self.gl.uniform_matrix4fv_with_f32_array(
                Some(&self.program.model_uniform),
                false,
                &model,
            );
            self.gl
                .uniform4fv_with_f32_array(Some(&self.program.material_uniform), &color.to_array());
            self.gl
                .uniform1f(Some(&self.program.material_model_uniform), material_model);
            self.gl.uniform4f(
                Some(&self.program.metallic_roughness_uniform),
                metallic,
                roughness,
                clearcoat,
                0.0,
            );
            self.gl.uniform1f(
                Some(&self.program.unlit_uniform),
                if unlit { 1.0 } else { 0.0 },
            );
            self.gl.active_texture(WebGlRenderingContext::TEXTURE0);
            if let Some(texture) = material.and_then(|material| material.texture.as_ref()) {
                self.gl
                    .bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(texture));
                self.gl
                    .uniform1f(Some(&self.program.use_texture_uniform), 1.0);
            } else {
                self.gl
                    .bind_texture(WebGlRenderingContext::TEXTURE_2D, None);
                self.gl
                    .uniform1f(Some(&self.program.use_texture_uniform), 0.0);
            }
            self.bind_mesh(mesh);
            if wireframe {
                self.gl.bind_buffer(
                    WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                    Some(&mesh.line_index_buffer),
                );
                self.gl.draw_elements_with_i32(
                    WebGlRenderingContext::LINES,
                    mesh.line_index_count,
                    WebGlRenderingContext::UNSIGNED_SHORT,
                    0,
                );
            } else {
                self.gl.bind_buffer(
                    WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                    Some(&mesh.index_buffer),
                );
                self.gl.draw_elements_with_i32(
                    WebGlRenderingContext::TRIANGLES,
                    mesh.index_count,
                    WebGlRenderingContext::UNSIGNED_SHORT,
                    0,
                );
            }
        }
    }

    fn bind_mesh(&self, mesh: &WebGlMesh) {
        const STRIDE: i32 = 12 * 4;
        self.gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&mesh.vertex_buffer),
        );
        self.gl
            .enable_vertex_attrib_array(self.program.position_attrib);
        self.gl.vertex_attrib_pointer_with_i32(
            self.program.position_attrib,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            STRIDE,
            0,
        );
        self.gl
            .enable_vertex_attrib_array(self.program.normal_attrib);
        self.gl.vertex_attrib_pointer_with_i32(
            self.program.normal_attrib,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            STRIDE,
            3 * 4,
        );
        self.gl
            .enable_vertex_attrib_array(self.program.color_attrib);
        self.gl.enable_vertex_attrib_array(self.program.uv_attrib);
        self.gl.vertex_attrib_pointer_with_i32(
            self.program.uv_attrib,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            STRIDE,
            6 * 4,
        );
        self.gl.vertex_attrib_pointer_with_i32(
            self.program.color_attrib,
            4,
            WebGlRenderingContext::FLOAT,
            false,
            STRIDE,
            8 * 4,
        );
    }
}

/// 浏览器渲染器，安全时选择 WebGPU，否则选择 WebGL。
#[wasm_bindgen]
pub struct BrowserRenderer {
    backend: BrowserBackend,
}

enum BrowserBackend {
    WebGpu(Box<WebRenderer>),
    WebGl(Box<WebGlRenderer>),
}

#[wasm_bindgen]
impl BrowserRenderer {
    /// 创建具有自动后端选择的浏览器渲染器。
    pub async fn new(canvas: HtmlCanvasElement) -> Result<BrowserRenderer, JsValue> {
        Self::new_with_preference(canvas, BrowserBackendPreference::Auto).await
    }

    /// 创建具有显式后端首选项的浏览器渲染器。
    pub async fn new_with_preference(
        canvas: HtmlCanvasElement,
        preference: BrowserBackendPreference,
    ) -> Result<BrowserRenderer, JsValue> {
        match preference {
            BrowserBackendPreference::WebGpu => {
                WebRenderer::new(canvas).await.map(|renderer| Self {
                    backend: BrowserBackend::WebGpu(Box::new(renderer)),
                })
            }
            BrowserBackendPreference::WebGl => {
                WebGlRenderer::new(canvas).await.map(|renderer| Self {
                    backend: BrowserBackend::WebGl(Box::new(renderer)),
                })
            }
            BrowserBackendPreference::Auto => {
                if should_try_webgpu() {
                    let webgl_canvas = canvas.clone();
                    match WebRenderer::new(canvas).await {
                        Ok(renderer) => Ok(Self {
                            backend: BrowserBackend::WebGpu(Box::new(renderer)),
                        }),
                        Err(_) => WebGlRenderer::new(webgl_canvas).await.map(|renderer| Self {
                            backend: BrowserBackend::WebGl(Box::new(renderer)),
                        }),
                    }
                } else {
                    WebGlRenderer::new(canvas).await.map(|renderer| Self {
                        backend: BrowserBackend::WebGl(Box::new(renderer)),
                    })
                }
            }
        }
    }

    /// 渲染一帧。
    pub fn tick(&mut self, timestamp_ms: f64) -> Result<(), JsValue> {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.tick(timestamp_ms),
            BrowserBackend::WebGl(renderer) => renderer.tick(timestamp_ms),
        }
    }

    /// 调整活动浏览器后端大小。
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.resize(width, height),
            BrowserBackend::WebGl(renderer) => renderer.resize(width, height),
        }
    }

    /// 更新指针位置。
    pub fn on_pointer_move(&mut self, x: f32, y: f32) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.on_pointer_move(x, y),
            BrowserBackend::WebGl(renderer) => renderer.on_pointer_move(x, y),
        }
    }

    /// 更新指针按下状态。
    pub fn on_pointer_down(&mut self, button: i16, x: f32, y: f32) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.on_pointer_down(button, x, y),
            BrowserBackend::WebGl(renderer) => renderer.on_pointer_down(button, x, y),
        }
    }

    /// 更新指针释放状态并运行拾取。
    pub fn on_pointer_up(&mut self, button: i16, x: f32, y: f32) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.on_pointer_up(button, x, y),
            BrowserBackend::WebGl(renderer) => renderer.on_pointer_up(button, x, y),
        }
    }

    /// 更新滚轮回退输入。
    pub fn on_wheel(&mut self, delta_y: f32) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.on_wheel(delta_y),
            BrowserBackend::WebGl(renderer) => renderer.on_wheel(delta_y),
        }
    }

    /// 更新按键按下状态。
    pub fn on_key_down(&mut self, code: &str) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.on_key_down(code),
            BrowserBackend::WebGl(renderer) => renderer.on_key_down(code),
        }
    }

    /// 更新按键释放状态。
    pub fn on_key_up(&mut self, code: &str) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.on_key_up(code),
            BrowserBackend::WebGl(renderer) => renderer.on_key_up(code),
        }
    }

    /// 转发紧凑触摸事件到活动后端。
    pub fn on_touch(&mut self, id: u64, phase: u8, x: f32, y: f32, pressure: f32) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.on_touch(id, phase, x, y, pressure),
            BrowserBackend::WebGl(renderer) => renderer.on_touch(id, phase, x, y, pressure),
        }
    }

    /// 更新指针锁定所有权。
    pub fn set_pointer_locked(&mut self, locked: bool) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.set_pointer_locked(locked),
            BrowserBackend::WebGl(renderer) => renderer.set_pointer_locked(locked),
        }
    }

    /// 转发相对指针移动。
    pub fn on_pointer_motion(&mut self, delta_x: f32, delta_y: f32) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.on_pointer_motion(delta_x, delta_y),
            BrowserBackend::WebGl(renderer) => renderer.on_pointer_motion(delta_x, delta_y),
        }
    }

    /// 更新一个标准游戏手柄连接槽位。
    pub fn set_gamepad_connected(&mut self, index: u8, connected: bool) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.set_gamepad_connected(index, connected),
            BrowserBackend::WebGl(renderer) => renderer.set_gamepad_connected(index, connected),
        }
    }

    /// 更新一个标准游戏手柄轴。
    pub fn set_gamepad_axis(&mut self, index: u8, axis: u8, value: f32) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.set_gamepad_axis(index, axis, value),
            BrowserBackend::WebGl(renderer) => renderer.set_gamepad_axis(index, axis, value),
        }
    }

    /// 更新一个标准游戏手柄按钮。
    pub fn set_gamepad_button(&mut self, index: u8, button: u8, value: f32) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.set_gamepad_button(index, button, value),
            BrowserBackend::WebGl(renderer) => renderer.set_gamepad_button(index, button, value),
        }
    }

    /// 启用或暂停动画。
    pub fn set_paused(&mut self, paused: bool) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.set_paused(paused),
            BrowserBackend::WebGl(renderer) => renderer.set_paused(paused),
        }
    }

    /// 返回动画是否已暂停。
    pub fn paused(&self) -> bool {
        match &self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.paused(),
            BrowserBackend::WebGl(renderer) => renderer.paused(),
        }
    }

    /// 显示或隐藏辅助几何体。
    pub fn set_helpers_visible(&mut self, visible: bool) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.set_helpers_visible(visible),
            BrowserBackend::WebGl(renderer) => renderer.set_helpers_visible(visible),
        }
    }

    /// 返回辅助几何体是否可见。
    pub fn helpers_visible(&self) -> bool {
        match &self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.helpers_visible(),
            BrowserBackend::WebGl(renderer) => renderer.helpers_visible(),
        }
    }

    /// 启用或禁用线框预览。
    pub fn set_wireframe_enabled(&mut self, enabled: bool) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.set_wireframe_enabled(enabled),
            BrowserBackend::WebGl(renderer) => renderer.set_wireframe_enabled(enabled),
        }
    }

    /// 返回是否启用了线框预览。
    pub fn wireframe_enabled(&self) -> bool {
        match &self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.wireframe_enabled(),
            BrowserBackend::WebGl(renderer) => renderer.wireframe_enabled(),
        }
    }

    /// 存储 Bloom UI 开关。
    pub fn set_bloom_enabled(&mut self, enabled: bool) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.set_bloom_enabled(enabled),
            BrowserBackend::WebGl(renderer) => renderer.set_bloom_enabled(enabled),
        }
    }

    /// 返回 Bloom UI 开关是否启用。
    pub fn bloom_enabled(&self) -> bool {
        match &self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.bloom_enabled(),
            BrowserBackend::WebGl(renderer) => renderer.bloom_enabled(),
        }
    }

    /// 存储 SSAO UI 开关。
    pub fn set_ssao_enabled(&mut self, enabled: bool) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.set_ssao_enabled(enabled),
            BrowserBackend::WebGl(renderer) => renderer.set_ssao_enabled(enabled),
        }
    }

    /// 返回 SSAO UI 开关是否启用。
    pub fn ssao_enabled(&self) -> bool {
        match &self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.ssao_enabled(),
            BrowserBackend::WebGl(renderer) => renderer.ssao_enabled(),
        }
    }

    /// 恢复默认轨道相机。
    pub fn reset_camera(&mut self) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.reset_camera(),
            BrowserBackend::WebGl(renderer) => renderer.reset_camera(),
        }
    }

    /// 返回活动后端类型。
    pub fn backend_kind(&self) -> BrowserBackendKind {
        match &self.backend {
            BrowserBackend::WebGpu(_) => BrowserBackendKind::WebGpu,
            BrowserBackend::WebGl(_) => BrowserBackendKind::WebGl,
        }
    }

    /// 返回活动后端标签。
    pub fn backend_label(&self) -> String {
        match self.backend_kind() {
            BrowserBackendKind::WebGpu => String::from("webgpu"),
            BrowserBackendKind::WebGl => String::from("webgl"),
            BrowserBackendKind::CanvasFallback => String::from("canvas-fallback"),
        }
    }

    /// 返回生成的场景名称。
    pub fn scene_name(&self) -> String {
        String::from("Scenix Engine Lab")
    }

    /// 返回最近的每秒帧数估计值。
    pub fn fps(&self) -> f32 {
        match &self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.fps(),
            BrowserBackend::WebGl(renderer) => renderer.fps(),
        }
    }

    /// 返回选中的场景节点名称。
    pub fn selected_node_name(&self) -> String {
        match &self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.selected_node_name(),
            BrowserBackend::WebGl(renderer) => renderer.selected_node_name(),
        }
    }

    /// 返回选中的节点 ID，未选中时返回零。
    pub fn selected_node_id(&self) -> u64 {
        match &self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.selected_node_id(),
            BrowserBackend::WebGl(renderer) => renderer.selected_node_id(),
        }
    }

    /// 设置活动编辑器变换模式。
    pub fn set_transform_mode(&mut self, mode: &str) {
        match &mut self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.set_transform_mode(mode),
            BrowserBackend::WebGl(renderer) => renderer.set_transform_mode(mode),
        }
    }

    /// 返回活动编辑器变换模式。
    pub fn transform_mode(&self) -> String {
        match &self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.transform_mode(),
            BrowserBackend::WebGl(renderer) => renderer.transform_mode(),
        }
    }

    /// 序列化活动后端场景检查器快照。
    pub fn inspector_snapshot_json(&self) -> String {
        match &self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.inspector_snapshot_json(),
            BrowserBackend::WebGl(renderer) => renderer.inspector_snapshot_json(),
        }
    }

    /// 返回当前射线投射命中距离。
    pub fn raycast_distance(&self) -> f32 {
        match &self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.raycast_distance(),
            BrowserBackend::WebGl(renderer) => renderer.raycast_distance(),
        }
    }

    /// 返回当前选中的材质标签。
    pub fn active_material(&self) -> String {
        match &self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.active_material(),
            BrowserBackend::WebGl(renderer) => renderer.active_material(),
        }
    }

    /// 以紧凑字符串形式返回活动浏览器演示功能标志。
    pub fn active_feature_flags(&self) -> String {
        match &self.backend {
            BrowserBackend::WebGpu(renderer) => renderer.active_feature_flags(),
            BrowserBackend::WebGl(renderer) => renderer.active_feature_flags(),
        }
    }
}
