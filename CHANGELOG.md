# 更新日志

scenekit 的所有重要变更将记录在此文件中。

格式遵循 [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)。
本项目遵循 [Semantic Versioning](https://semver.org/spec/v2.0.0.html)。

## [未发布]

## [1.5.0] - 2026-07-14

### 新增

- 新增了零分配的聚合输入，支持触摸、手势、四种标准游戏手柄、指针锁定、逻辑/物理视口指标以及瞬态键盘/指针切换。
- 新增了 Arcball、Trackball、Map、FirstPerson 和 PointerLock 控制器，以及现有 Orbit 和 Fly 控件的聚合输入更新。
- 新增了场景编辑器元数据、层策略、确定性选择状态、变换模式/空间/约束，以及平移/旋转/缩放对齐。
- 新增了 BVH 复用 API、透视和正交相机的框选、拖拽平面、可逆拖拽控件和可逆变换控件。
- 新增了可复用的变换/包围盒/选择/对齐网格 Gizmo，具有分析式手柄碰撞检测和持久化线段缓冲区。
- 新增了场景、相机、材质、灯光、纹理、动画混合器、渲染器诊断和 GPU 资源的类型化检查器快照，以及固定到 `0.33.3` 的可选 egui 适配器。
- 新增了按需渲染器 ID/法线/深度拾取，具有密集 `NodeId` 映射、裁剪像素请求、可复用 GPU/回读资源和世界坐标重建。
- 新增了 DPR 感知的浏览器指标和触摸、指针锁定、游戏手柄、变换模式、选择及检查器 JSON 的图元转发。
- 新增了五个 v1.5 示例、聚焦集成测试、GPU 拾取覆盖率、交互热路径基准测试、发布文档和版本派生的发布自动化。

### 变更

- 将所有 17 个工作空间 crate 和内部依赖需求提升至 `1.5.0`；保留 Rust 1.89、edition 2024 和现有默认 feature。
- `LineGeometry` 和 BVH/射线投射路径现在暴露了可复用输出方法，以避免热循环中的重复分配。
- 网站和独立 WASM 查看器清单现在与工作空间发布版本匹配。

### 兼容性与迁移说明

- 现有的 v1.4 控制器、场景、光线投射器和辅助工具 API 仍然可用；v1.5 的新增内容为增量式添加。
- `editor` 和 `egui` 是可选的门面 feature。GPU 拾取需要渲染器/WebGPU 路径；WebGL 继续使用 CPU BVH 拾取。
- egui 集成渲染共享的检查器模型，但有意不提供 v1.10 计划中的完整编辑器外壳。

## [1.4.0] - 2026-07-09

### 新增

- 新增了基于片段的动画运行时：`AnimationClip`、`AnimationAction`、`AnimationMixer`、`PropertyBinding`、`ClipTrack`/`ClipChannel`、`LoopMode`（Once / Repeat / PingPong）、`AnimationMarker`/`AnimationEvent`、交叉淡入淡出、叠加混合以及确定性的逐帧采样。
- 新增了关键帧轨道 `KeyframeScalar`、`KeyframeVec3`、`KeyframeQuat`、`KeyframeColor` 和 `KeyframeBool`，支持 `Linear`、`Step` 和 `CubicSpline` 插值（四元数使用 slerp + 最短弧）。
- 新增了 `PropertyBinding` 类型化目标，支持节点、骨骼、材质、相机、灯光和变形权重，以及用于稳定累加器查找的 `BindingKey`。
- 新增了 `LightAnimator`、`LightAnimationTarget`、`LightStoreMut` 和 `LightStores`，用于动画化点光源/聚光灯/方向光的颜色、强度、范围和聚光角度。
- 新增了 `MorphWeightAnimator` 和 `MorphWeightStoreMut`，用于动画化变形目标权重。
- 新增了 `RetargetMap` 骨骼按名称重定向。
- 新增了 `scenekit-mesh` 蒙皮数据模型：`SkinningAttributes`、`SkinningData`、`MorphWeights`、`final_joint_matrices`、`cpu_skin` 和 `apply_morph` CPU 回退。
- 新增了 `scenekit-renderer` GPU 蒙皮 + 变形上传钩子：`GpuSkinningRegistry`、`register_skin`、`update_bone_matrices`、`register_morph_targets`、`update_morph_weights` 和 `SKINNING_WGSL` 代码片段。
- 新增了 `scenekit-helpers` 的 `AnimationPathHelper` 和 `PoseHelper` 调试几何体。
- 新增了 `scenekit-loader` 解码的动画访问器输出字节到 `LoadedAnimationChannel::output`。
- 新增了门面 `clip_from_loaded` 桥接，从导入片段到运行时。
- 新增了 `examples/animation_runtime.rs`、`examples/animation_mixer.rs`、`examples/skeleton_skinning.rs` 和 `examples/animation_events.rs`。
- 新增了 `tests/animation_runtime.rs` 和 `benches/animation_mixer_bench.rs`。
- 新增了 `docs/release-v1.4.0.md` 和 `docs/examples/animation-runtime.md`。
- 新增了 `.github/release-notes/1.4.0.md` 作为 GitHub Release 正文。

### 变更

- 将所有工作空间 crate 和内部依赖需求提升至 `1.4.0`。
- 将 Animato 提升至 `1.7.0`；之前的 `1.6.0` 发布门控已解决。
- `ScenixAnimationDriver::tick` 现在接受 `lights` 和 `morphs` 存储，除了现有的节点/相机/材质/骨骼动画器外，还会裁剪 `light_animators` 和 `morph_animators`。
- `DriverStats` 新增了 `light_animators` 和 `morph_animators` 计数器。
- `scenekit-animato` 现在依赖 `scenekit-light` 以支持灯光动画器。
- 更新了 README、架构说明、路线图、feature 矩阵、CI 和发布工作流、示例以及动画运行时发布说明。

### 迁移说明

- `ScenixAnimationDriver::tick` 签名现在包含 `lights` 和 `morphs`；如果未使用，请传递空的 `LightStores` / `BTreeMap<MeshId, Vec<f32>>` 存储。
- `LoadedAnimationChannel` 新增了 `output: Vec<f32>` 字段；请相应更新结构体字面量。
- Animato `1.5.0` → `1.7.0` 对于 `std` / `tween` / `spring` / `serde` feature 集是直接替换的；无需在 scenekit 端进行代码更改。

## [1.3.0] - 2026-06-16

### 新增

- 新增了 `AssetPackage`、`AssetManager`、资产请求、异步加载句柄、依赖图、诊断、内存预算记账和过期缓存失效，用于 v1.3 资产管线。
- 新增了 `AssetId`、`SkinId` 和 `AnimationClipId`。
- 新增了 `GltfLoader::load_package_file`、`load_package_bytes` 和 `load_package_url`，用于包导入，同时保留 `GltfAsset`。
- 新增了 glTF 包附属文件，支持蒙皮、蒙皮属性、变形目标、动画片段元数据、材质扩展元数据、纹理变换、变体、KTX2/BasisU 说明、Draco 诊断和 meshopt 诊断。
- 新增了导出器辅助工具，支持 glTF 摘要 JSON/GLB 字节、OBJ、STL、PLY 和场景 JSON。
- 新增了 `RendererAssetExt::register_asset_package`，通过门面在启用 `loader` 和 `renderer` 时使用。
- 新增了 `asset_pipeline`、`asset_manager`、`export_scene`、`animation_import` 和 `compressed_assets` 示例。
- 新增了 `.github/release-notes/v1.3.0.md` 作为 GitHub Release 正文。

### 变更

- 将所有工作空间 crate 和内部依赖需求提升至 `1.3.0`。
- 为 Animato `1.6.0` 准备了发布文档作为发布门控。Cargo 仍使用 Animato `1.5.0`，直到 `1.6.0` 发布并解决。
- 更新了 README、loader 文档、feature 矩阵、工作流、示例、路线图和资产管线发布架构说明。

## [1.2.0] - 2026-06-13

### 新增

- 新增了渲染器拥有的 GPU 纹理上传，支持 `Texture2D`、`TextureCube` 和 `Texture3D`，包括 Mipmap 感知的字节范围、采样器转换和压缩格式能力检查。
- 新增了增量式渲染器生命周期 API，用于更新、注销和清除纹理资源，以及渲染目标创建、渲染到纹理、纹理回读、环境描述符、诊断、资源统计和管线缓存统计。
- 新增了渲染器注册半球光、面光源和光照探针数据，与现有的环境光、方向光、点光源和聚光灯并存。
- 新增了 `examples/render_target_capture.rs`，并扩展了渲染器示例，以测试带纹理的 PBR、卡通渐变纹理、环境贴图和渲染目标。
- 新增了 `docs/release-v1.2.0.md`，包含渲染器对等迁移说明和小型带纹理材质代码示例。

### 变更

- 将所有工作空间 crate 和内部依赖需求提升至 `1.2.0`。
- 更新了 `scenekit-animato` 以使用 Animato `1.5.0`。
- 重新设计了活跃渲染器绘制路径，批量处理逐绘制 uniform 写入，并绑定真实的材质纹理和灯光 uniform。
- 升级了浏览器回退，在 WebGPU 不可用时优先使用真实的 WebGL2 渲染器路径，包括纹理采样、材质 uniform、方向/点光源照明、卡通/物理近似、动画、拾取，以及明确的 WebGL2/WebGL1 对等诊断。
- 更新了 README、API 文档、示例文档、feature 矩阵、工作流和 GitHub Release 自动化，用于渲染器与材质对等发布。

## [1.1.0] - 2026-05-31

### 新增

- 新增了 `scenekit-wasm::BrowserRenderer`，用于在 WebGPU 和 WebGL 之间自动选择浏览器后端。
- 新增了 `scenekit-wasm::WebGlRenderer`，作为生成的 Scenix Engine Lab 场景在 WebGPU 不可用或不适合时的浏览器回退渲染器。
- 新增了 `BrowserBackendPreference` 和 `BrowserBackendKind`，以便应用可以强制使用 WebGPU、强制使用 WebGL 或报告活跃后端。
- 新增了门面和 `scenekit-wasm` 测试，用于新的浏览器后端枚举。

### 变更

- 将所有工作空间 crate 和内部依赖需求提升至 `1.1.0`。
- 更新了网站演示桥接，使用 `BrowserRenderer` 并在 Canvas2D 回退之前尝试 WebGL。
- 更新了 README、架构说明、WASM 文档、发布说明和 feature 矩阵文本，用于 WebGPU 到 WebGL 的浏览器回退。
- 更新了 CI、Pages 和发布网站构建，传递 `NO_COLOR=false` 以确保 Trunk 兼容性。

## [1.0.0] - 2026-05-27

### 新增

- 新增了 `docs/` 下的稳定 v1 文档集，包括入门指南、核心概念、材质、平台、基准测试和发布说明指南。
- 新增了 `website/` 下的独立 Leptos CSR 网站，包含生成的 Scenix Engine Lab 演示、控件、crate 地图、示例、SEO 元数据和 GitHub Pages 部署支持。
- 新增了剩余的架构示例：物理材质、卡通着色、实例化、LOD、变形目标、雾、精灵粒子和环境贴图。
- 新增了 GitHub Pages、覆盖率、package-check、website-build 和 release-note 工作流覆盖，用于稳定发布。

### 变更

- 将所有工作空间 crate 和内部依赖需求提升至 `1.0.0`。
- 围绕相机视图投影矩阵、逐绘制世界矩阵、可复用 uniform 缓冲区、材质预览 uniform 和缓存管线布局稳定了渲染器帧路径。
- 扩展了渲染器材质注册，覆盖 PBR、Physical、Unlit、Lambert、Toon、Wireframe/debug 和 Normal 预览材质。
- 通过去重脏根节点并在子树遍历期间避免子向量克隆，优化了场景变换传播。
- 扩展了浏览器封装，增加了生成的演示内容、切换、选择状态、FPS/材质获取器，以及网站的非 panic 回退行为。
- 为稳定 v1 API 契约重写了 README、架构说明、路线图、发布自动化和发布说明。

### 迁移说明

- 将 `0.9` 依赖需求替换为 `1`。
- 保留 loader、renderer、post、Animato 和 WASM 集成的显式 feature 标志；这些重量级路径仍然是可选的。
- 渲染器用户可以通过新的稳定 `register_*_material` 方法注册高级预览/调试材质。

## [0.9.0] - 2026-05-26

### 新增

- 新增了 `scenekit-animato` crate，包含与 Animato 1.4.0 兼容的 `Vec3`、`Quat` 和 `Color` 封装，以及标量、向量、四元数、颜色和布尔轨道。
- 新增了节点、相机、PBR 材质、骨骼姿势和确定性驱动器动画 API，将 Animato 补间和弹簧应用于现有 scenekit 数据。
- 新增了 `scenekit-wasm` crate，包含 DOM 键/指针映射辅助工具、panic 钩子设置、有效 Canvas 大小裁剪，以及浏览器 `WebRenderer` 封装（围绕现有渲染器和生成的立方体场景）。
- 新增了 `examples/animato_integration.rs`、`examples/wasm_viewer/` 和 `benches/animato_bridge_bench.rs`。
- 新增了节点、相机、材质、骨骼、驱动器、serde、WASM 辅助映射和门面导出的集成测试。

### 变更

- 将所有工作空间 crate 提升至 `0.9.0`。
- 更新了 `scenekit` 门面 crate，添加了可选的 `animato` 和 `wasm` feature，同时保持 v0.8 默认 CPU 创作、光线投射器和辅助工具 feature 不变。
- 更新了 README、路线图、架构说明、CI 检查、发布顺序和生成的 GitHub Release 说明，用于集成发布。

## [0.8.0] - 2026-05-26

### 新增

- 新增了 `scenekit-raycaster` crate，包含 `Raycaster`、`Bvh`、`GeometryProvider`、相机 NDC 射线辅助工具和精确的世界空间网格三角形相交。
- 新增了节点级 SAH BVH 构建/遍历，基于可见场景网格 AABB，支持层过滤和暴力验证。
- 新增了 `scenekit-helpers` crate，包含经过验证的 `LineGeometry`、网格、坐标轴、包围盒、箭头、灯光、相机和骨骼调试辅助工具。
- 新增了 `examples/raycasting.rs`、`examples/helpers_demo.rs`、`benches/bvh_bench.rs` 和 `benches/helpers_bench.rs`。
- 新增了射线图元、相机射线、BVH 与暴力拾取对比、层/可见性过滤、辅助几何体验证、辅助输出计数、serde 往返和门面导出的集成测试。

### 变更

- 将所有工作空间 crate 提升至 `0.8.0`。
- 更新了 `scenekit` 门面 crate，默认启用并重新导出 `scenekit-raycaster` 和 `scenekit-helpers`。
- 更新了 README、路线图、架构说明、CI 检查、发布顺序和生成的 GitHub Release 说明，用于光线投射与辅助工具发布。

## [0.7.0] - 2026-05-26

### 新增

- 新增了 `scenekit-loader` crate，包含 CPU 端 glTF/GLB、OBJ/MTL、STL、PNG/JPEG/WebP、KTX2、HDR/EXR 和路径缓存加载 API。
- 新增了 `GltfLoader`、`GltfAsset`、`LoadedCamera`、`LoadedLight`、`LoaderOptions` 和 `AssetCache`，用于渲染器无关的资产导入。
- 新增了 `scenekit-post` crate，包含 `PostStack`、`PostEffect`、`PostTarget`、`PostContext`、泛光、SSAO、色调映射、FXAA、TAA、SMAA、景深、雾、描边和运动模糊配置。
- 新增了可选的渲染器后处理管线集成，通过 `Renderer::set_post_stack`、`Renderer::post_stack` 和 `Renderer::post_stack_mut`。
- 新增了 `examples/gltf_scene.rs`、`examples/post_processing.rs`、`benches/loader_bench.rs` 和 `benches/post_bench.rs`。
- 新增了生成的 loader 夹具、图像/KTX2/STL/OBJ 解析、缓存行为、后处理管线排序、后处理配置裁剪、门面导出和可选 GPU 后处理冒烟覆盖的集成测试。

### 变更

- 将所有工作空间 crate 提升至 `0.7.0`。
- 更新了 `scenekit` 门面 crate，添加了可选的 `loader` 和 `post` feature。
- 更新了 README、路线图、架构说明、CI 检查、发布顺序和生成的 GitHub Release 说明，用于加载器与后处理发布。

## [0.6.0] - 2026-05-25

### 新增

- 新增了 `scenekit-renderer` crate，包含可选的 `wgpu` 渲染、headless 和 surface 目标、渲染器拥有的网格/材质/纹理/灯光注册表、帧统计、G-buffer 目标、阴影贴图图集分配和渲染目标调整大小支持。
- 新增了 `GpuMaterial` 实现，支持 `PbrMaterial`、`UnlitMaterial` 和 `LambertMaterial`，以及稳定的材质 uniform 字节打包。
- 新增了 `PipelineCache`、渲染器管线键、裁剪辅助工具、透明和不透明绘制排序，以及用于首个渲染器通道结构的嵌入式 WGSL 着色器入口点。
- 新增了渲染器示例，包括无头立方体、PBR 球体和阴影场景。
- 新增了渲染器配置验证、几何体打包、资源注册表错误、格式映射、裁剪错误、排序、材质 uniform 字节、门面导出和渲染器 serde 的 CPU 集成测试。
- 新增了 GPU 门控测试，用于管线缓存复用、无头帧缓冲区冒烟渲染和调整目标重新创建。
- 新增了 `benches/render_bench.rs`，用于 1K、10K 和 100K 三角形场景渲染提交。

### 变更

- 将所有工作空间 crate 提升至 `0.6.0`。
- 更新了 `scenekit` 门面 crate，在可选 `renderer` feature 后暴露渲染器 API，同时保持默认 feature 仅为 CPU。
- 更新了 README、路线图、架构说明、CI 检查、发布顺序和生成的 GitHub Release 说明，用于渲染器发布。

## [0.5.0] - 2026-05-23

### 新增

- 新增了 `scenekit-texture` crate，包含原始 CPU `Texture2D`、`TextureCube`、`Texture3D`、`VideoTexture`、`Sampler`、确定性 `TextureAtlas` 打包、`TextureFormat` 字节大小辅助工具和 RGBA8 CPU Mipmap 生成。
- 新增了 `scenekit-camera` crate，包含透视、正交和立方体相机、WebGPU 深度视锥体提取、屏幕到射线辅助工具，以及消耗 `scenekit-input` 状态的轨道和飞行控制器。
- 新增了 CPU 端示例，用于带相机射线的纹理 Mipmap 和轨道相机控件。
- 新增了纹理验证、图集打包、Mipmap、视频帧更新、相机投影/视图行为、视锥体可见性、立方体相机矩阵、控制器裁剪、门面导出和 serde 往返的集成测试。
- 新增了纹理 Mipmap/图集工作和相机视锥体/控制器工作的仅编译基准测试。

### 变更

- 将所有工作空间 crate 提升至 `0.5.0`。
- 更新了 `scenekit` 门面 crate，默认启用并重新导出 `scenekit-camera` 和 `scenekit-texture`，通过默认 `camera` 和 `texture` feature。
- 更新了 README、路线图、架构说明、CI 检查、发布顺序和生成的 GitHub Release 说明，用于纹理与相机发布。

## [0.4.0] - 2026-05-20

### 新增

- 新增了 `scenekit-material` crate，包含 `Material` trait、紧凑的 `PipelineKey`、`AlphaMode`、PBR、物理、无光照、Lambert、卡通、法线、线框、深度、线段、点和自定义 WGSL 着色器材质。
- 新增了 `scenekit-light` crate，包含环境光、方向光、点光源、聚光灯、半球光和面光源，经过验证的 `ShadowConfig`，以及原始采样的球谐光照 `LightProbe` 投影。
- 新增了 CPU 端示例，用于材质/灯光场景设置、材质管线键和光照探针。
- 新增了材质管线键唯一性、alpha 行为、材质/灯光 serde 往返、灯光构造函数、阴影验证、场景灯光附加和 SH 投影验证的集成测试。

### 变更

- 将所有工作空间 crate 提升至 `0.4.0`。
- 更新了 `scenekit` 门面 crate，默认启用并重新导出 `scenekit-material` 和 `scenekit-light`，通过默认 `material` 和 `light` feature。
- 更新了 README、路线图、架构说明、CI 检查、发布顺序和生成的 GitHub Release 说明，用于材质与灯光发布。

## [0.3.0] - 2026-05-17

### 新增

- 新增了 `scenekit-mesh` crate，包含 CPU 端 `Geometry`、`Mesh`、`MorphTarget`、`InstancedMesh`、`BatchedMesh`、缓冲区布局元数据和渲染器无关的图元生成。
- 新增了面加权法线生成、带手性的 UV 导数切线生成、几何体包围盒、验证和索引几何体合并。
- 新增了标准图元：长方体、球体、平面、圆柱体、圆锥体、胶囊体、环面、环面结、二十面球体、圆、环、车削、挤压、管和形状几何体。
- 新增了 `Shape` 支持，用于挤压时的外部轮廓和孔侧壁。
- 新增了网格集成测试，覆盖验证、法线、切线、合并、包围盒、实例化、批处理、图元有效性、缠绕顺序、UV 范围、门面导出和 serde 往返。
- 新增了 `benches/mesh_gen_bench.rs`，用于图元生成、切线计算和几何体合并吞吐量。

### 变更

- 将所有工作空间 crate 提升至 `0.3.0`。
- 更新了 `scenekit` 门面 crate，默认启用并重新导出 `scenekit-mesh`，通过默认 `mesh` feature。
- 更新了 README、路线图、CI 检查、发布顺序和生成的 GitHub Release 说明，用于几何体发布。

## [0.2.0] - 2026-05-16

### 新增

- 新增了 `scenekit-scene` crate，包含基于 SlotMap 的 `SceneGraph`、图局部 `NodeId` 句柄、根管理、父子层级操作以及确定性的深度优先和广度优先遍历。
- 新增了 `SceneNode`、`NodeKind`、`Fog`、`Sprite`、`BillboardMode` 和 `LodGroup` 场景数据类型。
- 新增了带缓存 `Mat4` 世界矩阵和 `Transform` 世界查询的脏子树世界变换传播。
- 新增了基于 Result 的层级变更，用于无效 ID 和循环预防。
- 新增了 `scenekit-scene` 的 `no_std + alloc` 支持，默认启用 `std`。
- 新增了场景图集成测试，覆盖层级不变性、变换传播、移除级联、遍历顺序、重新父化、循环预防、场景支持类型、门面导出和 serde 往返。
- 新增了仅编译的 10K 节点场景图基准测试目标。

### 变更

- 将所有工作空间 crate 提升至 `0.2.0`。
- 更新了 `scenekit` 门面 crate，默认启用并重新导出 `scenekit-scene`，通过默认 `scene` feature。
- 更新了 CI 和发布工作流，用于新的 scene crate。

## [0.1.0] - 2026-05-15

### 新增

- 新增了 `scenekit-math` crate，包含自定义 `no_std` 标量 `f32` 数学库：`Vec2`、`Vec3`、`Vec4`、`Mat3`、`Mat4`、`Quat`、`Euler`、`Transform`、`Ray3`、`Aabb`、`Sphere`、`Plane`、`Spherical` 和 `Cylindrical`。
- 新增了 `scenekit-math` 的可选 `libm`、`serde` 和 `approx` 支持。
- 新增了 `scenekit-core` crate，包含类型化 ID、`Color`、颜色空间辅助工具、错误枚举和共享 trait。
- 新增了 `scenekit-core::GpuUpload` 的可选 `gpu` 支持。
- 新增了 `scenekit-input` crate，包含固定位集 `KeyboardState`、`PointerState`、`KeyCode`、`PointerButton` 和 `Modifiers`。
- 新增了 `scenekit` 门面 crate，重新导出 v0.1.0 Foundation API。
- 新增了数学运算、颜色转换、射线相交、包围盒、变换和输入状态的单元测试。
- 新增了门面和 serde 集成测试。
- 新增了仅编译的数学基准测试目标。
- 新增了 v0.1.0 范围的 CI 和发布工作流。
- 重写了 README，仅记录已发布的 Foundation API 接口。

[Unreleased]: https://github.com/launcher-rs/scenekit/compare/v1.5.0...HEAD
[1.5.0]: https://github.com/launcher-rs/scenekit/compare/v1.4.0...v1.5.0
[1.4.0]: https://github.com/launcher-rs/scenekit/compare/v1.3.0...v1.4.0
[1.3.0]: https://github.com/launcher-rs/scenekit/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/launcher-rs/scenekit/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/launcher-rs/scenekit/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/launcher-rs/scenekit/compare/v0.9.0...v1.0.0
[0.9.0]: https://github.com/launcher-rs/scenekit/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/launcher-rs/scenekit/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/launcher-rs/scenekit/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/launcher-rs/scenekit/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/launcher-rs/scenekit/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/launcher-rs/scenekit/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/launcher-rs/scenekit/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/launcher-rs/scenekit/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/launcher-rs/scenekit/releases/tag/v0.1.0
