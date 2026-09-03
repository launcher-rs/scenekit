# scenekit — 项目路线图

> *意大利语：scenekit — scene（场景），即万物登场的舞台。*
> 一个专业级、与渲染器无关的 Rust 3D 场景库。

本路线图追踪从 `v0.1.0` 到 `v1.1.0` 的已完成路径，以及 1.0 之后可能成为未来 `v1.x` 版本的构想。
每个里程碑都是一个可运行的、已发布的 crate — 而非草案。没有测试、文档和基准测试的内容不会发布。

---

## 状态图例

| 符号 | 含义 |
|--------|---------|
| ✅ | 已完成 |
| 🔄 | 进行中 |
| 📋 | 已计划 |
| 🔮 | 未来 / 1.0 之后 |

---

## 版本概览

| 版本 | 名称 | 重点 | 状态 |
|---------|------|-------|--------|
| `v0.1.0` | 基础 | 数学、核心 trait、ID、颜色、错误类型 | ✅ |
| `v0.2.0` | 场景图 | 场景节点树、变换、遍历、雾效、LOD | ✅ |
| `v0.3.0` | 几何体 | 网格、所有图元、变形目标、实例化/批处理网格 | ✅ |
| `v0.4.0` | 材质与灯光 | 材质 trait、PBR、物理、卡通、所有灯光类型 | ✅ |
| `v0.5.0` | 纹理与相机 | 纹理加载、采样器、图集、相机类型、控制器 | ✅ |
| `v0.6.0` | 渲染器 | wgpu 管线、延迟+前向渲染、阴影贴图 | ✅ |
| `v0.7.0` | 加载器与后处理 | GLTF/OBJ/STL 加载器、后处理栈 | ✅ |
| `v0.8.0` | 射线检测与辅助工具 | BVH 射线检测器、调试辅助工具、输入抽象 | ✅ |
| `v0.9.0` | 集成 | animato 桥接、WASM 浏览器支持、框架兼容 | ✅ |
| `v1.0.0` | 稳定版 | API 冻结、完整文档、示例、CI 全绿 | ✅ |
| `v1.1.0` | 浏览器回退 | WebGPU-to-WebGL 浏览器回退与更新的发布自动化 | ✅ |
| `v1.2.0` | 渲染器对等 | 真实材质 GPU 路径、纹理绑定、灯光、阴影、IBL、渲染目标 | ✅ |
| `v1.3.0` | 资产管线 | glTF 扩展、动画导入、压缩诊断、额外加载器、导出器、资产管理员 | ✅ |
| `v1.4.0` | 动画运行时 | 片段/混合器/动作层、骨骼动画、变形播放、重定向辅助工具 | ✅ |
| `v1.5.0` | 交互工具 | 变换/拖拽/指针锁定控制、选择辅助工具、编辑器图元 | ✅ |
| `v1.6.0` | 着色器节点 | 新的可选着色器图和节点材质 crate | 🔮 |
| `v1.7.0` | 粒子 | 新的可选 CPU/GPU 粒子 crate | 🔮 |
| `v1.8.0` | 环境系统 | 新的可选地形、天空和水面 crate | 🔮 |
| `v1.9.0` | 运行时桥接 | 新的可选 XR、音频和物理 crate | 🔮 |
| `v1.10.0` | 编辑器工具 | 新的可选编辑器和 UI 覆盖层 crate | 🔮 |
| `v1.x+` | 高级渲染 | 后处理效果、几何体扩展、修改器、GPU 驱动渲染、实时全局光照 | 🔮 |

---

## v0.1.0 — 基础

**目标：** scenekit 最小可用版本。开发者可以创建 3D 数学类型、变换，并使用核心 trait 系统。无需 GPU。

### 已发布的 crate

- `scenekit-math` `v0.1.0`
- `scenekit-core` `v0.1.0`
- `scenekit-input` `v0.1.0`
- `scenekit` `v0.1.0`（门面 — 仅包含 math + core + input）

### 交付物

**`scenekit-math`**
- [x] `Vec2` — `new`、`dot`、`length`、`normalize`、`lerp`、`angle_between`
- [x] `Vec3` — `new`、`dot`、`cross`、`length`、`normalize`、`lerp`、`reflect`、`angle_between`
- [x] `Vec4` — `new`、`dot`、`length`、`normalize`、`lerp`
- [x] `Mat3` — `identity`、`from_mat4`、`determinant`、`inverse`、`transpose`
- [x] `Mat4` — `identity`、`look_at`、`perspective`、`orthographic`、`inverse`、`transpose`、`mul`
- [x] `Quat` — `identity`、`from_axis_angle`、`from_euler_xyz`、`slerp`、`normalize`、`inverse`、`mul`
- [x] `Euler` — `new`、`from_quat`、`from_mat4`、`to_quat`、6 种旋转顺序
- [x] `Transform` — `IDENTITY`、`to_mat4`、`mul_transform`、`inverse`、`forward`、`right`、`up`
- [x] `Ray3` — `at`、`intersect_aabb`、`intersect_sphere`、`intersect_triangle`
- [x] `Aabb` — `from_points`、`center`、`half_extents`、`contains_point`、`intersects_aabb`、`merge`、`surface_area`
- [x] `Plane` — `from_normal_and_point`、`from_three_points`、`signed_distance`、`intersect_ray`
- [x] `Spherical` — `from_vec3`、`to_vec3`、`clamp_phi`
- [x] `Cylindrical` — `from_vec3`、`to_vec3`
- [x] `no_std` 编译门控，`libm` feature 用于三角函数
- [x] 所有类型的 `serde` feature
- [x] `approx` feature 用于 `AbsDiffEq` 实现
- [x] 所有公开项的完整文档注释
- [x] 测试：`Mat4::perspective` 生成正确的视锥体
- [x] 测试：`Quat::slerp` 在 t=0、t=0.5、t=1 时正确插值
- [x] 测试：`Transform::to_mat4` 与 decompose 双向转换
- [x] 测试：`Ray3::intersect_triangle` Möller–Trumbore 正确性

**`scenekit-core`**
- [x] `NodeId`、`MeshId`、`MaterialId`、`TextureId`、`LightId` — 基于 `u64` 的 `Copy + Hash + Eq` 新类型
- [x] `Renderable` trait — `fn render_order() -> u32`
- [x] `Bounded` trait — `fn aabb() -> Aabb`、`fn bounding_sphere() -> (Vec3, f32)`
- [x] `GpuUpload` trait（`gpu` feature 之后）— `type GpuData: bytemuck::Pod`、`fn to_gpu()`
- [x] `Named` trait — `fn name()`、`fn set_name()`
- [x] `Color` 结构体 — RGBA f32、`rgb()`、`rgba()`、`hex()`、`to_linear()`、`to_srgb()`
- [x] `ColorSpace` 枚举 — `Linear`、`Srgb`
- [x] `ScenixError` 枚举 — `LoadError`、`GpuError`、`ValidationError`
- [x] `no_std` 兼容
- [x] 测试：颜色十六进制解析、sRGB ↔ 线性转换正确性

**`scenekit-input`**
- [x] `PointerState` — 位置、增量、按键位掩码
- [x] `KeyboardState` — `is_pressed()`、`on_key_down()`、`on_key_up()`
- [x] `KeyCode` 枚举 — WASD、方向键、空格、Shift、Ctrl
- [x] `Modifiers` 结构体 — Shift、Ctrl、Alt、Meta
- [x] 测试：按键按下/释放状态跟踪

**文档与基础设施**
- [x] `README.md` 包含安装、快速入门、功能表
- [x] `ARCHITECTURE.md`
- [x] `ROADMAP.md`（本文件）
- [x] `CONTRIBUTING.md`
- [x] `CHANGELOG.md` 包含 `## [0.1.0]` 条目
- [x] `LICENSE-MIT` 和 `LICENSE-APACHE`
- [x] `.github/workflows/ci.yml` — 测试、clippy、fmt、文档、no_std
- [x] `.github/workflows/publish.yml` — 按依赖顺序发布到 crates.io
- [x] `benches/math_bench.rs` — Mat4 乘法、Quat slerp、AABB 相交

---

## v0.2.0 — 场景图

**目标：** 一个可运行的场景图，支持父子层级、变换传播和遍历。开发者可以构建节点树并计算世界变换。

### 已发布的 crate

- `scenekit-scene` `v0.2.0`（新增）
- 所有先前 crate 升级到 `v0.2.0`

### 交付物

**`scenekit-scene`**
- [x] `SceneGraph` — 基于 `SlotMap` 的节点存储、根节点管理
- [x] `SceneNode` — `name`、`transform`、`visible`、`layer`、`NodeKind`
- [x] `NodeKind` 枚举 — `Empty`、`Group`、`Mesh`、`Light`、`Camera`、`Sprite`
- [x] `graph.add(node) -> NodeId`
- [x] `graph.add_child(parent, node) -> Result<NodeId, ValidationError>`
- [x] `graph.remove(id)` — 移除节点及其所有子节点
- [x] `graph.get(id) -> Option<&SceneNode>`、`graph.get_mut(id)`
- [x] `graph.parent(id)`、`graph.children(id)`
- [x] `graph.find_by_name(name) -> Option<NodeId>`
- [x] 脏标记变换传播 — `graph.update_world_transforms()`
- [x] `graph.world_matrix(id) -> Option<Mat4>`
- [x] `graph.iter_depth_first()`、`graph.iter_breadth_first()`
- [x] `Fog` — `Fog::Linear { near, far, color }`、`Fog::Exponential { density, color }`
- [x] `LodGroup` — 排序的 `(max_distance, MeshId)` 级别，`fn select(distance: f32) -> Option<MeshId>`
- [x] `Sprite` — `width`、`height`、`texture_id`、公告牌朝向模式
- [x] 测试：父子层级、世界变换正确性、级联删除
- [x] 测试：深度优先遍历顺序、脏标记正确性
- [x] `benches/scene_graph_bench.rs` — 10K 节点遍历 + 变换传播

---

## v0.3.0 — 几何体

**目标：** 所有几何体类型、顶点缓冲区管理和网格图元。开发者可以生成任何标准 3D 形状。

### 已发布的 crate

- `scenekit-mesh` `v0.3.0`（新增）
- 所有先前 crate 升级到 `v0.3.0`

### 交付物

**`scenekit-mesh`**
- [x] `Geometry` 结构体 — 位置、法线、UV、UV2、颜色、索引、切线
- [x] `geometry.compute_normals()` — 面加权顶点法线
- [x] `geometry.compute_tangents()` — 带手性的切线生成
- [x] `geometry.aabb()`、`geometry.bounding_sphere()`
- [x] `geometry.merge(other)` — 合并几何体
- [x] `Mesh` 结构体 — `Geometry` + `MaterialId`
- [x] `BufferLayout`、`VertexAttribute`、`IndexFormat`
- [x] `MorphTarget` — `name`、`positions_delta`、`normals_delta`、`weight`
- [x] `InstancedMesh` — `mesh_id`、`material_id`、`transforms: Vec<Mat4>`、`set_transform_at()`
- [x] `BatchedMesh` — 单次绘制调用中的多个几何体

**图元（均返回 `Geometry`）**
- [x] `box_geometry(w, h, d, w_seg, h_seg, d_seg)`
- [x] `sphere_geometry(radius, width_seg, height_seg)`
- [x] `plane_geometry(w, h, w_seg, h_seg)`
- [x] `cylinder_geometry(top_r, bottom_r, height, radial_seg, height_seg, open_ended)`
- [x] `cone_geometry(radius, height, radial_seg, height_seg)`
- [x] `capsule_geometry(radius, height, cap_seg, radial_seg)`
- [x] `torus_geometry(radius, tube, radial_seg, tubular_seg)`
- [x] `torus_knot_geometry(radius, tube, tubular_seg, radial_seg, p, q)`
- [x] `icosphere_geometry(radius, subdivisions)`
- [x] `circle_geometry(radius, segments, theta_start, theta_length)`
- [x] `ring_geometry(inner_r, outer_r, theta_seg, phi_seg)`
- [x] `lathe_geometry(points, segments, phi_start, phi_length)`
- [x] `extrude_geometry(shape, depth, bevel_thickness, bevel_size, bevel_segments)`
- [x] `tube_geometry(path, tubular_seg, radius, radial_seg, closed)`
- [x] `shape_geometry(shape)` — 2D 形状三角化

- [x] 测试：每个图元生成有效法线（dot(n, face_normal) > 0）
- [x] 测试：每个图元在给定分段参数下顶点数正确
- [x] 测试：UV 坐标在 [0, 1] 范围内
- [x] `benches/mesh_gen_bench.rs` — 图元生成吞吐量

---

## v0.4.0 — 材质与灯光

**目标：** 材质系统和所有灯光类型。开发者可以创建 PBR、卡通和自定义着色器材质，并使用所有标准灯光类型照亮场景。

### 已发布的 crate

- `scenekit-material` `v0.4.0`（新增）
- `scenekit-light` `v0.4.0`（新增）

### 交付物

**`scenekit-material`**
- [x] `Material` trait — `pipeline_key()`、`is_transparent()`、`double_sided()`、`alpha_cutoff()`
- [x] `PipelineKey` 结构体 — 决定使用哪个着色器管线
- [x] `AlphaMode` 枚举 — `Opaque`、`Mask(f32)`、`Blend`
- [x] `PbrMaterial` — 反照率、金属度、粗糙度、法线/AO/自发光纹理、Alpha 模式
- [x] `PhysicalMaterial` — 清漆、光泽、透射、厚度、折射率、虹彩
- [x] `UnlitMaterial` — 颜色 + 可选纹理，无光照
- [x] `LambertMaterial` — 仅漫反射，比 PBR 更快
- [x] `ToonMaterial` — 渐变贴图、离散步进、轮廓线
- [x] `NormalMaterial` — 调试法线 → RGB
- [x] `WireframeMaterial` — 线框覆盖层
- [x] `DepthMaterial` — 阴影通道深度输出
- [x] `LineMaterial` — 宽度、虚线模式、颜色
- [x] `PointsMaterial` — 点大小、衰减
- [x] `ShaderMaterial` — 自定义 WGSL 顶点/片段着色器、原始 uniform
- [x] 测试：不同材质配置下 `PipelineKey` 的唯一性

**`scenekit-light`**
- [x] `AmbientLight` — 颜色、强度
- [x] `DirectionalLight` — 方向、颜色、强度、可选 `ShadowConfig`
- [x] `PointLight` — 颜色、强度、范围、衰减、可选 `ShadowConfig`
- [x] `SpotLight` — 颜色、强度、范围、角度、半影、可选 `ShadowConfig`
- [x] `HemisphereLight` — sky_color、ground_color、强度
- [x] `AreaLight` — 宽度、高度、颜色、强度（LTC 近似）
- [x] `LightProbe` — 9 系数 SH、`from_coefficients()`、`from_cube_faces()`、`from_equirectangular_samples()`
- [x] `ShadowConfig` — map_size、near、far、bias、pcf_radius、cascades
- [x] 测试：从原始立方体采样的 SH 投影产生非零系数

---

## v0.5.0 — 纹理与相机

**目标：** CPU 端纹理管理和相机系统。开发者可以加载纹理、配置采样器，并设置带轨道控制的透视/正交/立方体相机。

### 已发布的 crate

- `scenekit-texture` `v0.5.0`（新增）
- `scenekit-camera` `v0.5.0`（新增）

### 交付物

**`scenekit-texture`**
- [x] `Texture2D` — 宽度、高度、格式、数据、mip 级别
- [x] `TextureCube` — 6 个面、格式、mip 级别
- [x] `Texture3D` — 宽度、高度、深度、格式、数据
- [x] `VideoTexture` — 逐帧更新接口
- [x] `Sampler` — 放大/缩小/mip 过滤器、地址 u/v/w、各向异性、比较
- [x] `TextureAtlas` — 矩形打包、按名称查找 UV
- [x] `TextureFormat` 枚举 — Rgba8、Rgba16Float、Depth32Float、Bc7、Astc、Etc2
- [x] `mipmap::generate(data, width, height) -> Vec<Vec<u8>>` — CPU mipmap 生成
- [x] 测试：图集打包容纳预期数量的矩形，UV 坐标有效

**`scenekit-camera`**
- [x] `PerspectiveCamera` — fov_y、aspect、near、far、`projection_matrix()`、`view_matrix()`
- [x] `OrthographicCamera` — left、right、top、bottom、near、far
- [x] `CubeCamera` — 6 面渲染用于环境贴图
- [x] `Frustum` — 从 VP 矩阵提取的 6 个平面、`contains_point()`、`intersects_aabb()`
- [x] `OrbitController` — 目标、距离、最小/最大极角、缩放、阻尼
- [x] `FlyController` — 速度、灵敏度、WASD 移动
- [x] 控制器消费来自 `scenekit-input` 的 `PointerState` + `KeyboardState`
- [x] 测试：视锥体正确剔除视体积外的点
- [x] 测试：轨道控制器将极角限制在 [min, max]

---

## v0.6.0 — 渲染器

**目标：** 通过 wgpu 进行 GPU 渲染。开发者可以使用 PBR 材质、阴影和延迟+前向管线渲染场景。

### 已发布的 crate

- `scenekit-renderer` `v0.6.0`（新增）

### 交付物

**`scenekit-renderer`**
- [x] `Renderer` — 拥有 `wgpu::Device`、`Queue`、`Surface`、`PipelineCache`、`GpuScene`
- [x] `RendererConfig` — 宽度、高度、采样数、vsync、hdr、present_mode、backends
- [x] `Renderer::new(window, config)` — 异步初始化
- [x] `Renderer::headless(config)` — 用于测试、工具和截取的离屏渲染器
- [x] `Renderer::render(&scene, &camera)` — 完整帧渲染
- [x] `Renderer::resize(w, h)` — 表面/离屏目标重新配置
- [x] `GpuMaterial` trait — `bind_group_layout()`、`to_uniform_bytes()`、`create_bind_group()`
- [x] `GpuMaterial` 为 `PbrMaterial`、`UnlitMaterial`、`LambertMaterial` 实现
- [x] `PipelineCache` — 按材质/通道/目标状态键控，延迟编译
- [x] `GpuScene` — 渲染器拥有的网格/材质/纹理/灯光注册表
- [x] `FrameContext` — 每帧相机 VP、分辨率和相机位置状态

**渲染通道**
- [x] `shadow_pass.rs` — 仅深度通道标记和 `ShadowMapAtlas`
- [x] `geometry_pass.rs` — G-buffer 通道标记和 `GBuffer`
- [x] `lighting_pass.rs` — 延迟光照通道标记
- [x] `forward_pass.rs` — 透明前向通道标记
- [x] `culling.rs` — 使用场景图边界的视锥体剔除
- [x] `sort.rs` — 前到后不透明排序和后到前透明排序

**着色器 (WGSL)**
- [x] `pbr.vert.wgsl`、`pbr.frag.wgsl` — PBR 顶点/片段着色器入口点
- [x] `unlit.frag.wgsl` — 无光照片段着色器
- [x] `shadow_depth.vert.wgsl` — 阴影通道顶点着色器
- [x] `deferred_resolve.wgsl` — 延迟光照全屏四边形

- [x] 测试：管线缓存对相同 `PipelineKey` 返回相同管线
- [x] 测试：离屏渲染产生非黑色帧缓冲区
- [x] `benches/render_bench.rs` — 1K / 10K / 100K 三角形的帧时间
- [x] `examples/hello_cube.rs` — 离屏立方体渲染
- [x] `examples/pbr_sphere.rs` — 带环境光和方向光设置的 PBR 球体
- [x] `examples/shadow_demo.rs` — 带阴影贴图配置的方向光

---

## v0.7.0 — 加载器与后处理

**目标：** 资产加载和后处理效果。开发者可以加载 GLTF 文件并应用泛光、SSAO、色调映射和其他效果。

### 已发布的 crate

- `scenekit-loader` `v0.7.0`（新增）
- `scenekit-post` `v0.7.0`（新增）

### 交付物

**`scenekit-loader`**
- [x] `GltfLoader::load(path) -> Result<GltfAsset>` — 网格、材质、纹理、相机、层级
- [x] `GltfLoader::load_url(url) -> Result<GltfAsset>` — `http` feature 之后的异步 HTTP
- [x] `obj::load(path) -> Result<Vec<Geometry>>` — OBJ + MTL 解析
- [x] `stl::load(path) -> Result<Geometry>` — 二进制 + ASCII STL
- [x] `image::load(path) -> Result<Texture2D>` — PNG、JPEG、WebP
- [x] `ktx2::load(path) -> Result<Texture2D>` — KTX2 容器元数据和支持的原始纹理格式
- [x] `hdr::load(path) -> Result<TextureCube>` — HDR/EXR 兼容图像解码为立方体纹理数据
- [x] `AssetCache` — 使用 `Arc<T>` 的规范路径去重、失效和清除
- [x] 测试：生成的 glTF/GLB、OBJ/MTL、STL、图像、KTX2、HDR 立方体、缓存和 serde 元数据覆盖

**`scenekit-post`**
- [x] `PostStack` — 有序的效果链，构建器模式
- [x] 全屏 GPU 通道栈，带仅增长的临时目标和缓存管线
- [x] 泛光 — 阈值、强度、半径
- [x] SSAO — 半径、强度、偏置
- [x] 色调映射 — `ToneMapper::None`、`Reinhard`、`Aces`、`Exposure`
- [x] FXAA — 快速近似抗锯齿通道
- [x] TAA — 反馈和抖动通道
- [x] SMAA — 质量预设通道
- [x] 景深 — 对焦距离、光圈、模糊半径
- [x] 雾效 — 屏幕空间雾颜色/密度混合
- [x] 轮廓线 — 亮度边缘轮廓线
- [x] 运动模糊 — 紧凑的屏幕空间模糊通道
- [x] 测试：PostStack 排序、移除、清除行为、配置钳制、serde 和 GPU 门控的冒烟路径
- [x] `examples/post_processing.rs` — 栈：SSAO + Bloom + ToneMap + FXAA + TAA
- [x] `examples/gltf_scene.rs` — 生成、加载、注册和渲染一个小型 glTF 场景

---

## v0.8.0 — 射线检测与辅助工具

**目标：** BVH 加速的射线检测和调试可视化。开发者可以用鼠标拾取对象并可视化场景结构。

### 已发布的 crate

- `scenekit-raycaster` `v0.8.0`（新增）
- `scenekit-helpers` `v0.8.0`（新增）

### 交付物

**`scenekit-raycaster`**
- [x] `Raycaster` — `cast_ray(scene, ray) -> Option<Intersection>`
- [x] `Raycaster::cast_ray_all(scene, ray) -> Vec<Intersection>` — 所有命中，按距离排序
- [x] `Raycaster::from_camera_ndc(camera, ndc_x, ndc_y) -> Ray3`
- [x] `Intersection` — `node_id`、`distance`、`point`、`normal`、`uv`
- [x] `Bvh` — 基于 SAH 的场景 AABB 列表构建
- [x] `Bvh::traverse(ray) -> Vec<NodeId>` — 候选列表
- [x] 测试：射线-AABB、射线-三角形、射线-球体相交正确性
- [x] 测试：BVH 产生与暴力搜索相同的结果（正确性证明）
- [x] `benches/bvh_bench.rs` — BVH 构建 + 1K 射线查询

**`scenekit-helpers`**
- [x] `LineGeometry` — 用于辅助输出的验证线列表存储
- [x] `GridHelper` — `to_geometry()` → 线列表网格平面
- [x] `AxesHelper` — `to_geometry()` → RGB XYZ 轴线
- [x] `BoundingBoxHelper` — 线框 AABB
- [x] `ArrowHelper` — 带可配置头部的方向箭头
- [x] `SpotLightHelper`、`PointLightHelper`、`DirectionalLightHelper`
- [x] `CameraHelper` — 视锥体线框可视化
- [x] `SkeletonHelper` — 骨骼可视化
- [x] `examples/raycasting.rs` — 使用 BVH 的鼠标拾取
- [x] `examples/helpers_demo.rs` — 一个场景中的所有辅助工具

---

## v0.9.0 — 集成

**目标：** 将 scenekit 连接到 animato 和浏览器。开发者可以使用弹簧/补间动画化场景属性，并在网页中运行 scenekit。

### 已发布的 crate

- `scenekit-animato` `v0.9.0`（新增）
- `scenekit-wasm` `v0.9.0`（新增）

### 交付物

**`scenekit-animato`**
- [x] `AnimVec3`、`AnimQuat`、`AnimColor` 包装器用于 Animato 插值，旋转使用四元数 slerp
- [x] `ScalarTrack`、`Vec3Track`、`QuatTrack`、`ColorTrack`、`BoolTrack` 由 Animato 1.4.0 的补间和弹簧支持
- [x] `NodeAnimator` — 将轨道绑定到 `NodeId` 变换和可见性字段
- [x] `NodeAnimationTarget` 枚举 — `Translation`、`Rotation`、`Scale`、`Visibility`
- [x] `CameraAnimator` — 通过 `CameraStoreMut` 动画化 fov、位置、目标、上向量和正交边界
- [x] `MaterialAnimator` — 动画化 PBR 反照率、不透明度、自发光、粗糙度和金属度字段
- [x] `SkeletonPose`、`BoneAnimation`、`SkinnedMeshAnimator` — 驱动显式骨骼变换数组
- [x] `ScenixAnimationDriver` — 每帧滴答所有绑定的动画器，支持暂停/恢复、添加/移除/清除、完成修剪和确定性顺序
- [x] 测试：节点变换/可见性动画、相机存储、PBR 材质字段、骨骼姿势、驱动器行为、serde 双向转换

**`scenekit-wasm`**
- [x] `WebRenderer` — 包装 `Renderer`、`SceneGraph`、`PerspectiveCamera`、`PointerState` 和 `KeyboardState` 用于 `<canvas>` + `requestAnimationFrame`
- [x] `WebRenderer::new(canvas) -> Result<WebRenderer, JsValue>` — 异步初始化
- [x] `WebRenderer::tick(timestamp_ms)` — 从 rAF 调用
- [x] `WebRenderer::resize(w, h)`
- [x] `on_pointer_move/down/up`、`on_wheel`、`on_key_down/up` — DOM 输入转发
- [x] `key_code_from_dom`、`pointer_button_from_dom`、`canvas_size`、`clamp_canvas_size` 和 panic hook 辅助工具
- [x] `examples/wasm_viewer/` — 生成场景的浏览器查看器
- [x] `examples/animato_integration.rs` — 弹簧相机目标 + 补间节点/材质动画
- [x] 测试/检查：DOM 映射单元测试、零大小调整钳制、wasm 目标编译、wasm 查看器编译

---

## v1.0.0 — 稳定版

**目标：** API 冻结。每个公开项都有文档，每个示例都编译，每个功能都有测试，CI 在 stable + beta + nightly 上全绿。

### 交付物

**API 稳定性**
- [x] 审查公开门面和子系统 API 以满足稳定 v1 契约
- [x] 保持可选重型系统在显式 feature 之后
- [x] 优先选择增量 v1 变更并记录弃用策略

**文档**
- [x] `docs/getting-started.md`
- [x] `docs/concepts.md`
- [x] `docs/materials-guide.md`
- [x] `docs/platform-guide.md`
- [x] `docs/benchmarks.md`
- [x] `docs/release-v1.1.0.md`
- [x] README、架构笔记、变更日志和发布自动化已更新

**测试**
- [x] stable/beta/nightly 测试工作流
- [x] `scenekit-wasm` 和独立查看器示例的 WASM 编译检查
- [x] 使用 lavapipe 的离屏 GPU 测试工作流
- [x] 使用 `cargo-llvm-cov` 的覆盖率工作流步骤
- [x] 渲染器材质注册和门面 v1 集成覆盖率

**CI**
- [x] `stable`、`beta`、`nightly` 测试
- [x] WASM 目标检查
- [x] CPU/no_std crate 的 no-default 检查
- [x] Clippy `--all-features -- -D warnings`
- [x] `cargo fmt --check`
- [x] 基准测试编译门控

**发布**
- [x] `CHANGELOG.md` 包含稳定版发布
- [x] GitHub Release 使用 `docs/release-v1.1.0.md`
- [x] 添加了 GitHub Pages 网站和 WASM 演示工作流

---

## 当前重点

项目已完成 v1.5.0，正在准备计划在 v1.6.0 发布的可选着色器节点工作。
新工作必须保持稳定的模块化 API，保持重型依赖可选，并在移除公开 API 之前添加弃用。

Scenix 不仅是一个网站或 WASM 演示库。未来的工作必须将**桌面、移动和 Web**视为一等运行时目标：

- 桌面：通过 `wgpu` 表面支持 Linux、Windows 和 macOS。
- 移动：通过 `wgpu` 支持 Android 和 iOS，原生生命周期处理、触摸/手柄输入和移动纹理格式。
- Web：WASM 优先 WebGPU，WebGL2 回退，以及干净的不可用后端处理。

## 未来里程碑 (`v1.x`)

以下版本号是规划桶，而非发布承诺。每个里程碑只应在测试、文档、示例和基准测试就绪时发布。

## v1.2.0 — 渲染器与材质对等

**目标：** 从稳定预览渲染转向生产级材质和光照行为。

### 已发布/更新的 crate

- `scenekit-renderer` — 主要工作：真实 GPU 材质路径、纹理绑定、灯光、阴影、IBL、渲染目标、诊断、资源生命周期和场景到渲染器同步。
- `scenekit-material` — 材质参数、纹理槽位、物理材质扩展、Alpha 行为、管线键和未来节点材质集成点。
- `scenekit-texture` — 采样器元数据、mipmap、压缩纹理格式、视频纹理更新契约和 GPU 上传所需的纹理验证。
- `scenekit-light` — 渲染器上传所需的灯光/阴影数据、级联、探针和面光源渲染器端元数据。
- `scenekit-post` — 后处理目标复用、深度/法线/运动输入契约以及渲染器变更影响后处理时的后端回退钩子。
- `scenekit-wasm` — WebGPU/WebGL 功能对等说明、回退行为和浏览器渲染器冒烟路径。
- `scenekit` — 任何新公开渲染器/材质/纹理 API 的门面重导出和 feature 门控。
- 此版本不计划新增 crate。

- [x] 为 `Texture2D`、`TextureCube`、`Texture3D`、mip 级别和采样器提供真实 GPU 纹理分配和绑定。
- [x] PBR 着色器路径，支持反照率纹理、金属度-粗糙度因子、自发光、顶点颜色、Alpha 遮罩、Alpha 混合和双面兼容渲染。
- [x] 物理着色器路径，支持实时清漆/光泽/透射方向的材质状态和环境响应。
- [x] 环境光、半球光、方向光、点光、聚光灯、面光源和光探针的真实光照集成。
- [x] 阴影图集分配、阴影元数据上传、PCF/偏置配置和 v1.2 示例的实际阴影回退行为。
- [x] 环境光照描述符，支持立方体纹理注册和可选光探针辐照度。
- [x] 用于 2D、HDR、深度元数据、离屏捕获、渲染到纹理和回读的渲染目标。
- [x] 针对桌面、移动、WebGPU 和 WebGL 回退的色彩管理和色调映射策略文档。
- [x] 渲染器诊断，包括资源计数、纹理内存、uniform 内存、渲染目标和管线缓存活动。
- [x] 用于注册、更新、注销和清除工作流的资源生命周期 API。
- [x] 场景到渲染器同步通过稳定 ID 和渲染器注册 API 保持显式。
- [x] 桌面、移动、WebGPU 和 WebGL 回退的渲染器功能能力矩阵。

## v1.3.0 — 资产管线

**目标：** 让 Scenix 适用于真实生产资产，而不仅仅是生成场景。

### 已发布/更新的 crate

- `scenekit-loader` — 主要工作：glTF 扩展、蒙皮、变形、动画导入、额外加载器、导出器、资产缓存、异步加载、热重载和资产元数据。
- `scenekit-mesh` — 导入资产所需的几何体属性、变形目标导入、蒙皮相关顶点属性和网格压缩/解压缩集成点。
- `scenekit-material` — 清漆、透射、体积、光泽、镜面、折射率、虹彩、自发光强度、纹理变换和变体的导入材质扩展映射。
- `scenekit-texture` — KTX2/BasisU 元数据、压缩纹理验证、纹理变换元数据和图像/HDR/EXR 集成点。
- `scenekit-scene` — 导入层级、节点元数据、变体元数据和已加载场景组织。
- `scenekit-camera` — 导入透视/正交相机转换和元数据。
- `scenekit-light` — 导入精确灯光和未来 IES/灯光元数据。
- `scenekit-animato` — 将导入的动画数据移交给计划在 `v1.4.0` 中的动画运行时。
- `scenekit-renderer` — 为已注册加载资产提供可选的资产到 GPU 便利辅助工具。
- `scenekit` — 新加载器/导出器 API 的门面重导出和 feature 门控。
- 此版本不计划新增 crate。

- [x] glTF 蒙皮、骨骼元数据、变形目标、动画片段和节点/灯光/相机扩展支持。
- [x] glTF 材质扩展：透射、体积、镜面、折射率、自发光强度、纹理变换、变体、KTX2/BasisU 说明、meshopt 诊断和 Draco 诊断。
- [x] FBX、Collada、PLY、SVG、USD/USDZ、3MF、VOX、VTK、Rhino 3DM、LDraw、TTF/字体、IES、DDS、TGA、TIFF、EXR、UltraHDR 和 LUT 格式的额外加载器支持矩阵，在完全解码不可用时仅提供诊断行为。
- [x] 导出器：glTF/GLB 摘要、OBJ、STL、PLY 和场景 JSON。
- [x] 资产管理员，支持异步加载、进度状态、取消、依赖图、内存预算、缓存失效和桌面过期文件轮询。
- [x] 涵盖资产包、管理器/缓存、导出器、导入动画元数据和压缩资产诊断的资产示例。
- [x] 为已加载资产包提供资产到 GPU 便利辅助工具，同时保留手动渲染器注册。

## v1.4.0 — 动画运行时

**目标：** 保持 Animato 作为值引擎，同时添加与 Three.js 片段和混合器相当的场景/资产动画层。

### 已发布/更新的 crate

- `scenekit-animato` — 主要工作：`AnimationClip`、`AnimationAction`、`AnimationMixer`、属性绑定、混合、事件、循环模式和确定性采样。
- `scenekit-loader` — 已加载的动画片段、骨骼、变形动画通道和从 glTF/FBX 风格资产导入的片段元数据。
- `scenekit-mesh` — 变形权重、蒙皮数据模型、顶点属性和 CPU/GPU 蒙皮数据移交。
- `scenekit-scene` — 节点、可见性、变换和场景层级目标的动画属性路径。
- `scenekit-material` — 材质字段和纹理驱动材质变体的动画目标。
- `scenekit-camera` — 相机位置、目标、投影和正交边界的动画目标。
- `scenekit-light` — 灯光强度、颜色、范围、角度和阴影相关字段的动画目标。
- `scenekit-renderer` — GPU 蒙皮、变形上传、动画驱动资源更新和渲染器同步钩子。
- `scenekit-helpers` — 骨骼、路径和动画调试辅助工具。
- `scenekit` — 新动画 API 的门面重导出和 feature 门控。
- 此版本不计划新增 crate。

- [x] `AnimationClip`、`AnimationAction` 和 `AnimationMixer` 等效物用于导入片段。
- [x] 节点变换、可见性、材质字段、相机、灯光、变形权重和骨骼的属性绑定。
- [x] 播放控制：循环模式、暂停/恢复、时间缩放、标记、事件、交叉淡入淡出、加法混合和确定性采样。
- [x] 骨骼动画数据模型、GPU 蒙皮路径、CPU 回退测试、骨骼辅助工具和姿势调试。
- [x] 重定向辅助工具和可选 IK 辅助工具。
- [x] 导入动画工作流的动画路径辅助工具和文档。

## v1.5.0 — 控制、交互与编辑器图元

**目标：** 支持产品查看器、编辑器、游戏、类 CAD 工具和数据可视化，而无需每个应用重建交互基础。

### 已发布/更新的 crate

- `scenekit-camera` — 主要相机控制工作：弧球、轨迹球、地图、第一人称、指针锁定以及改进的轨道/飞行行为。
- `scenekit-input` — 触摸、手势、手柄、指针锁定、高 DPI 标准化和跨平台输入映射。
- `scenekit-raycaster` — 选择框/视锥体拾取、拖拽平面支持、悬停/活跃/选中工作流、图层遮罩和拾取辅助工具。
- `scenekit-helpers` — 变换小部件几何体、选择辅助工具、边界辅助工具、相机/灯光/骨骼编辑器辅助工具和捕捉/网格视觉效果。
- `scenekit-scene` — 选择元数据、图层策略、编辑器端节点元数据和场景检查器支持。
- `scenekit-renderer` — 编辑器拾取和视口覆盖层所需的对象 ID/深度/法线缓冲区或回读钩子。
- `scenekit-wasm` — 指针锁定、触摸手势、拖拽控制和 WebView/浏览器查看器行为的浏览器输入转发。
- `scenekit` — 新控制/输入/辅助工具 API 的门面重导出和 feature 门控。
- 此版本不计划新增 crate；`scenekit-editor` 应等待这些图元就绪。

- [x] 弧球、轨迹球、地图、第一人称、指针锁定、拖拽和变换控制。
- [x] 平移、旋转、缩放、边界、相机、灯光和骨骼小部件。
- [x] 选择框/视锥体拾取、悬停/活跃/选中状态模型、拖拽平面、捕捉、网格约束和图层遮罩。
- [x] 场景图、相机、灯光、材质、纹理、动画、渲染器统计和 GPU 资源的检查器数据模型。
- [x] Web 覆盖层支持和可选原生 egui 检查器集成。
- [x] 轨道、平移、捏合缩放、拖拽和变换操作的移动触摸手势映射。

## v1.6.0 — 着色器节点与节点材质

**目标：** 在原始 `ShaderMaterial` 之上添加类型化着色器图层，而不削弱底层 WGSL 逃生舱口。

### 已发布/更新的 crate

- `scenekit-nodes` — 新的可选 crate，用于着色器节点、材质图、序列化节点图、WGSL 生成和 WebGL 兼容子集验证。
- `scenekit-material` — 节点材质集成点、材质图引用、管线键集成和内置节点材质描述符。
- `scenekit-renderer` — 节点着色器编译、着色器缓存集成、绑定组布局生成、诊断和回退行为。
- `scenekit-post` — 实际可行时的后处理节点图集成。
- `scenekit-wasm` — 生成着色器的 WebGPU/WebGL 兼容性检查。
- `scenekit` — 可选 `nodes` 门面 feature 和重导出。

- [ ] `scenekit-nodes` crate 脚手架，包含文档、测试和门面 feature。
- [ ] 常量、uniform、属性、变量、纹理、数学、色彩空间、光照、雾效、色调映射和后处理效果的类型化节点。
- [ ] 用于渲染器集成的 WGSL 后端。
- [ ] 用于浏览器回退的 WebGL 兼容子集验证器。
- [ ] 通过 `scenekit-renderer` 渲染的第一个节点材质。
- [ ] 编辑器生成材质图的序列化格式。

## v1.7.0 — 粒子

**目标：** 为效果、可视化、精灵和轻量级模拟添加可复用的粒子系统。

### 已发布/更新的 crate

- `scenekit-particles` — 新的可选 crate，用于发射器、粒子数据、CPU 模拟、支持时的 GPU 模拟、模块和示例。
- `scenekit-scene` — 粒子场景附件或节点元数据。
- `scenekit-mesh` — 公告牌/精灵/点几何体辅助工具和粒子缓冲区布局。
- `scenekit-material` — 粒子、精灵、点、软粒子和翻页材质支持。
- `scenekit-texture` — 图集/翻页纹理支持和粒子纹理元数据。
- `scenekit-renderer` — 粒子绘制路径、批处理、实例化、GPU 缓冲区、可选计算路径和回退路径。
- `scenekit-wasm` — WebGPU/WebGL 粒子能力说明。
- `scenekit` — 可选 `particles` 门面 feature 和重导出。

- [ ] CPU 粒子发射器，支持生成、生命周期、速度、加速度、颜色、大小、旋转和曲线模块。
- [ ] 精灵/点粒子渲染示例。
- [ ] 粒子纹理图集和翻页动画支持。
- [ ] 批处理粒子上传路径。
- [ ] 支持时的可选 GPU 计算模拟。
- [ ] 非计算粒子场景的 WebGL 回退策略。

## v1.8.0 — 地形、天空与水面

**目标：** 为真实场景、游戏、产品查看器和模拟添加可复用的环境系统。

### 已发布/更新的 crate

- `scenekit-terrain` — 新的可选 crate，用于高度图、分块 LOD、splat 贴图、地形碰撞数据、流式传输和地形示例。
- `scenekit-sky` — 新的可选 crate，用于程序化天空、大气散射、太阳/天空光照辅助工具、环境贴图生成和接地天空盒支持。
- `scenekit-water` — 新的可选 crate，用于水面、波浪、泡沫、反射、折射、菲涅耳行为和水下辅助工具。
- `scenekit-renderer` — 地形 LOD 绘制路径、天空/背景路径、水面反射/折射目标、环境捕获钩子和回退行为。
- `scenekit-material` — 地形、天空、大气和水面材质描述符。
- `scenekit-texture` — 高度图、法线贴图、splat 贴图和环境纹理处理。
- `scenekit-light` — 太阳/天空/环境光集成。
- `scenekit-scene` — 环境对象附件和元数据。
- `scenekit` — 可选 `terrain`、`sky` 和 `water` 门面 feature 和重导出。

- [ ] 带分块 LOD 示例的高度图地形。
- [ ] 地形 splat 贴图材质路径。
- [ ] 程序化天空背景和太阳/天空光照辅助工具。
- [ ] 带反射/折射渲染目标的水面平面。
- [ ] 环境功能的移动/Web 能力说明。
- [ ] 共享环境演示场景。

## v1.9.0 — XR、音频与物理桥接

**目标：** 为沉浸式应用、空间音频和模拟添加可选运行时桥接，同时保持核心渲染器独立。

### 已发布/更新的 crate

- `scenekit-xr` — 新的可选 crate，用于 WebXR/OpenXR 会话、控制器、手部追踪、命中测试、锚点、平面、深度传感、估计光照和移动 XR 生命周期。
- `scenekit-audio` — 新的可选 crate，用于音频监听器、位置音频源、流式音频、分析器数据和场景节点附件。
- `scenekit-physics` — 新的可选 crate，用于 Rapier/Jolt 桥接、刚体、碰撞器、角色控制器辅助工具、调试可视化和场景变换同步。
- `scenekit-scene` — XR、音频和物理的节点附件和同步元数据。
- `scenekit-input` — XR 控制器、手柄、触摸和设备输入映射。
- `scenekit-camera` — XR 相机装备、立体相机辅助工具和监听器/相机同步。
- `scenekit-helpers` — 物理碰撞器辅助工具、XR/控制器辅助工具和调试可视化。
- `scenekit-renderer` — 需要时的立体/XR 渲染目标钩子和调试绘制支持。
- `scenekit-wasm` — WebXR 浏览器集成。
- `scenekit` — 可选 `xr`、`audio` 和 `physics` 门面 feature 和重导出。

- [ ] 一个 WebXR 或 OpenXR 查看器示例。
- [ ] XR 控制器输入和姿态映射。
- [ ] 附加到场景节点的音频监听器加位置音频源。
- [ ] 带变换同步的物理刚体/碰撞器桥接。
- [ ] 角色控制器或简单碰撞示例。
- [ ] 控制器、碰撞器和物理状态的调试辅助工具。

## v1.10.0 — 编辑器与 UI 工具

**目标：** 仅在渲染器、资产、动画、控制和资源生命周期 API 足够强大之后构建编辑器端工具。

### 已发布/更新的 crate

- `scenekit-editor` — 新的可选 crate，用于可视化编辑器外壳、资产浏览器、场景检查器、小部件、材质编辑器、动画时间线、导入/导出工作流和项目元数据。
- `scenekit-ui` 或 `scenekit-egui` — 新的可选 crate，用于跨平台调试 UI 覆盖层、渲染器统计、场景检查器面板和工具小部件。
- `scenekit-scene` — 编辑器元数据、检查器支持、选择状态和序列化钩子。
- `scenekit-renderer` — 视口覆盖层、对象 ID/深度/法线缓冲区、渲染器统计和资源检查器钩子。
- `scenekit-material` — 材质检查器和节点材质编辑器支持。
- `scenekit-loader` — 资产浏览器、导入/导出工作流、重载和资产依赖图集成。
- `scenekit-animato` — 动画时间线、片段、动作控制和预览播放。
- `scenekit-helpers` — 变换小部件、选择辅助工具、调试覆盖层和编辑器视觉效果。
- `scenekit-wasm` — 实际可行时的浏览器编辑器支持。
- `scenekit` — 可选 `editor` 和 `ui` 门面 feature 和重导出。

- [ ] 最小场景检查器/编辑器外壳。
- [ ] 渲染器统计覆盖层。
- [ ] 带选择和变换编辑的场景图面板。
- [ ] 带纹理槽位和物理材质字段的材质检查器。
- [ ] 使用加载器/导出器 API 的资产浏览器。
- [ ] 导入片段的动画时间线预览。
- [ ] 保存/加载项目或场景元数据格式。

## v1.x+ — 高级渲染与几何体扩展

**目标：** 追踪尚未分配到特定未来版本的高级工作。

| 领域 | 未来工作 |
|------|-------------|
| 后处理 | SSR、SSGI、GTAO、SAO、LUT、胶片、晕影、色差、故障效果、半调、像素化、残像、过渡、遮罩、降噪、光束、镜头光晕 |
| 几何体 | 多面体、四面体、八面体、十二面体、圆角盒、文本、贴花、凸体、参数化、NURBS、边、线框几何体 |
| 修改器 | 简化、细分、边分割、曲线流动、网格表面采样器、凸包、OBB、八叉树 |
| 场景对象 | 反射器、折射器、镜头光晕、移动立方体、阴影捕捉器、替代体、贴花、体积切片 |
| GPU 驱动渲染 | 间接绘制、GPU 剔除、集群/前向+光照、大场景渲染器路径 |
| 实时全局光照 | SSGI 优先，实际可行时的未来探针/网格/路径选项 |

### 可选未来 crate

| Crate | 说明 |
|------|-------|
| `scenekit-nodes` | 着色器图、类型化着色器节点、节点材质、序列化材质图、WGSL 后端和 WebGL 兼容子集 |
| `scenekit-particles` | CPU 粒子、支持时的 GPU 粒子、精灵批处理、发射器模块和粒子示例 |
| `scenekit-terrain` | 高度图地形、分块 LOD、splat 贴图、地形碰撞数据和流式传输 |
| `scenekit-sky` | 程序化天空、大气散射、太阳/天空光照辅助工具和接地天空盒支持 |
| `scenekit-water` | 水面、反射/折射辅助工具、泡沫、波浪和水下/菲涅耳材质支持 |
| `scenekit-xr` | WebXR 和 OpenXR 支持，用于 VR/AR、控制器、手部追踪、命中测试、锚点、平面、估计光照和移动 XR 生命周期 |
| `scenekit-audio` | 音频监听器、空间音频源、流式音频、分析器数据和场景节点附件 |
| `scenekit-physics` | Rapier/Jolt 桥接、刚体、碰撞器、角色控制器辅助工具、调试可视化和场景变换同步 |
| `scenekit-editor` | 可视化场景编辑器、资产浏览器、场景检查器、小部件、材质编辑器、动画时间线、导入/导出工作流 |
| `scenekit-ui` 或 `scenekit-egui` | 桌面、移动和 Web 的跨平台调试 UI 覆盖层 |

### Crate 工作图

**目标：** 在创建 Issue 或新 crate 之前明确未来工作归属。

首先应扩展现有 crate：

| 工作区域 | 主要 Crate | 待做工作 |
|-----------|------------------|------------|
| 生产渲染器 | `scenekit-renderer` | PBR/物理 GPU 着色器、真实灯光、阴影、IBL、渲染目标、渲染器统计、GPU 资源生命周期 |
| 材质模型 | `scenekit-material` | 材质参数、管线键、纹理槽位、物理扩展、未来节点材质集成点 |
| 纹理系统 | `scenekit-texture`、`scenekit-renderer` | 压缩格式、mipmap、采样器元数据、视频纹理更新、GPU 上传/绑定、纹理内存核算 |
| 灯光与阴影 | `scenekit-light`、`scenekit-renderer` | 阴影配置、级联、探针、灯光上传、阴影图集集成、面光源渲染器行为 |
| 资产导入/导出 | `scenekit-loader` | glTF 扩展、动画导入、蒙皮、变形、额外加载器、导出器、资产缓存、异步加载、热重载 |
| 后处理 | `scenekit-post`、`scenekit-renderer` | 额外效果、深度/法线/运动缓冲区、后处理图、目标复用、后端 feature 回退 |
| 导入动画 | `scenekit-animato`、`scenekit-loader`、`scenekit-mesh` | 动画片段、混合器/动作层、属性绑定、骨骼播放、变形播放、混合 |
| 控制与拾取 | `scenekit-camera`、`scenekit-input`、`scenekit-raycaster`、`scenekit-helpers` | 弧球/轨迹球/地图/第一人称/指针锁定控制、拖拽辅助工具、选择框、小部件几何体 |
| 场景图数据 | `scenekit-scene`、`scenekit-core` | 资源/版本跟踪、图层策略、场景到渲染器同步元数据、更强的 ID 和错误处理 |
| 浏览器运行时 | `scenekit-wasm`、`scenekit-renderer` | WebGPU/WebGL 对等说明、回退行为、浏览器冒烟场景、WebView 支持 |
| 桌面/移动运行时 | `scenekit-renderer`、`scenekit-input`、示例 | winit 示例、Android/iOS 示例、表面丢失/重建、高 DPI、触摸、手势、手柄 |
| 可视化验证 | `tests/`、`examples/`、`benches/` | 基准图像、渲染器冒烟测试、兼容性场景、基准测试门控 |

仅在实现开始时创建未来 crate：

| 未来 Crate | 创建时机 | 首个有用交付物 |
|--------------|-------------|--------------------------|
| `scenekit-nodes` | 着色器图工作需要超过 `ShaderMaterial` 的功能 | 一个通过 `scenekit-renderer` 渲染的节点材质，包含文档和测试 |
| `scenekit-particles` | 精灵/公告牌示例需要可复用发射器 | 带一个渲染器示例的 CPU 粒子发射器 |
| `scenekit-terrain` | 高度图/分块 LOD 工作开始 | 带 LOD 示例的高度图地形网格 |
| `scenekit-sky` | 环境/天空功能超出材质范围 | 驱动场景光照或背景的程序化天空 |
| `scenekit-water` | 水面需要反射/折射渲染目标 | 带反射/折射路径的水面平面示例 |
| `scenekit-xr` | WebXR/OpenXR 运行时工作开始 | 一个带控制器输入的 WebXR 或 OpenXR 查看器示例 |
| `scenekit-audio` | 空间音频集成开始 | 附加到场景节点的监听器加位置音频源 |
| `scenekit-physics` | 物理同步设计就绪 | 带调试辅助工具示例的刚体/碰撞器桥接 |
| `scenekit-editor` | 检查器/小部件/资源 API 足够就绪 | 使用现有 crate 的最小场景检查器/编辑器外壳 |
| `scenekit-ui` 或 `scenekit-egui` | 调试 UI 需要可复用覆盖层 | 渲染器统计覆盖层和场景检查器面板 |

在拥有 crate 目录、feature 门控计划、文档、测试和至少一个示例之前，不要将未来 crate 添加到 workspace 清单中。

### 跨领域资源管理

**目标：** 让 Scenix 可用于长时间运行的应用、编辑器、移动应用和资产密集型场景，而不泄露所有权或隐藏 GPU 成本。

- [ ] CPU 数据、GPU 数据、资产缓存条目、渲染目标、阴影贴图、后处理目标和浏览器回退资源的稳定资源生命周期策略。
- [ ] 几何体、纹理、材质、灯光、变换、骨骼、变形权重和动画驱动数据的脏/版本跟踪。
- [ ] 顶点缓冲区、索引缓冲区、纹理、uniform 缓冲区、渲染目标、阴影图集和后处理临时目标的 GPU 内存核算。
- [ ] 几何体内存、纹理内存、后处理目标、阴影贴图和资产缓存大小的资源预算控制。
- [ ] 用于热重载、编辑器删除操作、场景卸载、设备丢失和移动挂起/恢复的显式清理 API。
- [ ] 不支持的格式、不支持的 GPU 功能、过期句柄、无效 ID、超出预算、上传失败和设备/表面丢失的清晰错误类别。

### 跨领域验证与可视化测试

**目标：** 用可重复的测试证明渲染器和资产行为，而不仅仅是示例。

- [ ] 参考场景、材质球、阴影、后处理、glTF 示例模型和选定动画帧的基准图像测试。
- [ ] 按后端记录的像素差异和感知差异容差。
- [ ] 纹理、阴影、环境贴图、透明排序、后处理效果和回读的离屏渲染器冒烟测试。
- [ ] 带记录回退行为的浏览器 WebGPU 和 WebGL 冒烟场景。
- [ ] Linux/Vulkan、Windows/DX12 或 Vulkan 和 macOS/Metal 的桌面兼容性表。
- [ ] Android/Vulkan 和 iOS/Metal 的移动兼容性表，包括生命周期、触摸、DPI 和压缩纹理。
- [ ] 功能支持标签：`Full`、`Partial`、`Fallback` 或 `Unsupported`，用于桌面、移动、WebGPU 和 WebGL。
- [ ] PBR、物理材质扩展、glTF 扩展、蒙皮、变形目标、动画片段、后处理效果和拾取的合规性场景。
- [ ] 大场景、纹理上传、资产加载、BVH 构建、动画采样和帧渲染时间的基准测试。
- [ ] CPU/no_std、所有 feature、wasm 编译、渲染器冒烟、文档、示例和基准测试编译检查的 CI 门控。

### 跨平台示例矩阵

**目标：** 让每个支持的运行时通过可运行示例对用户可见。

| 目标 | 必需示例 |
|--------|-------------------|
| 桌面 | winit 表面应用、egui 覆盖层应用、Tauri/WebView 应用、离屏/离屏捕获 |
| 移动 | Android Vulkan/wgpu 应用、iOS Metal/wgpu 应用、触摸轨道控制、打包资产加载、挂起/恢复 |
| Web | WebGPU 查看器、WebGL 回退查看器、从 URL 加载资产、优雅的不可用后端 UI |
| 共享 | 相同场景在桌面、移动和 Web 上渲染，带记录的功能差异 |

## 未来工作规则

- 每个功能必须声明桌面、移动和 Web 支持级别。
- 重型系统通过专注 crate 或门面 feature 保持可选。
- CPU 创作 crate 必须保持与渲染器无关。
- 仅浏览器 API 不得泄露到桌面/移动核心 API。
- 移动端专有约束（如生命周期、触摸输入、DPI、表面重建和压缩纹理）必须经过深思熟虑地处理。
- 原生桌面、Android、iOS 和 Web 示例应在功能成熟时添加，而非绑定到单个发布桶。
- `scenekit-input` 应作为跨平台需求增长触摸、手势、手柄、指针锁定和高 DPI 标准化。
- BC、ASTC、ETC2、KTX2/BasisU 的纹理能力检测和回退转码应由相关渲染器/资产里程碑处理。
- 每当添加渲染器功能时，应记录 WebGPU/WebGL 对等说明和优雅的功能回退行为。
- 新渲染器功能需要专注的示例、测试和至少一个基准测试或冒烟测试。

---

## 贡献 scenekit

参见 [`CONTRIBUTING.md`](./CONTRIBUTING.md) 了解如何设置工作区、运行测试和提交拉取请求。

现在贡献的最佳方式是提出一个专注的 1.1 后规划 Issue 或 PR，同时保持稳定的 API 契约。

---

*路线图版本：1.5.0 控制、交互与编辑器图元 — 最后更新于 2026 年 7 月 14 日*
*下一个里程碑：v1.6.0 着色器节点与节点材质*
*项目：launcher-rs — github.com/launcher-rs/scenekit*
*配套库：animato — github.com/launcher-rs/animato*
