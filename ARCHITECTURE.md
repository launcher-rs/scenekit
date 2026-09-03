# scenekit — 完整项目架构

> *意大利语: scenekit — 场景，万物呈现的舞台。*
>
> 一个专业级、渲染器无关的 Rust 3D 场景库。
> 构建为干净的 Cargo 工作区。由 `wgpu` 驱动。由 `animato` 动画化。
> 为游戏、创意工具、数据可视化、原生桌面、移动设备、WASM 浏览器等设计。

---

## 目录

1. [项目愿景](#1-项目愿景)
2. [为什么使用工作区 — 而不是单一 crate](#2-为什么使用工作区--而不是单一-crate)
3. [工作区布局](#3-工作区布局)
4. [Crate 规范](#4-crate-规范)
5. [数据流与渲染循环](#5-数据流与渲染循环)
6. [类型系统设计](#6-类型系统设计)
7. [GPU 架构](#7-gpu-架构)
8. [功能标志策略](#8-功能标志策略)
9. [错误处理策略](#9-错误处理策略)
10. [测试策略](#10-测试策略)
11. [性能指南](#11-性能指南)
12. [集成目标](#12-集成目标)
13. [CI / CD 流水线](#13-ci--cd-流水线)
14. [发布清单](#14-发布清单)
15. [命名与风格约定](#15-命名与风格约定)
16. [平台支持与框架集成](#16-平台支持与框架集成)
17. [未来路线图](#17-未来路线图)

---

## 1. 项目愿景

scenekit 围绕一个原则构建：**任何可以描述的 3D 对象都可以被渲染和动画化。**

其他一切 —— 场景图、相机、材质、灯光、阴影、后处理、资产加载、GPU 批处理 —— 都清晰地层叠在这个基础之上。每一层都位于自己的 crate 中，可以独立使用或与其他层组合。

scenekit 是双库生态系统的**渲染部分**。`animato` 处理*事物如何移动*。scenekit 处理*事物的外观和位置*。它们共同构成了 Rust 的完整 Three.js 等效物。

### 设计目标

| 目标 | 决策 |
|------|------|
| Three.js 人体工程学，Rust 性能 | 到处使用构建器模式，零强制运行时开销 |
| `wgpu` 作为 GPU 后端 | 在 Vulkan、Metal、DX12、WebGPU 上运行 —— 一个代码库 |
| 渲染器无关的场景图 | `scenekit-scene` 和 `scenekit-math` 零 GPU 依赖 |
| 干净的 crate 边界 | 每个关注点都在自己的 crate 中 |
| 可组合，而非单体 | 只使用你需要的 crate |
| 类型安全的节点层次结构 | `NodeId` 新类型，无原始指针图 |
| 一流的 `animato` 集成 | 将 animato 补间直接插入场景变换 |
| 桌面 + 移动 + Web 对等性 | 相同的场景/数据 API 目标 Vulkan、Metal、DX12、WebGPU 和 WebGL2 回退路径 |
| `no_std` 就绪核心 | `scenekit-math` 和 `scenekit-core` 无需 `std` 或堆即可编译 |
| 可序列化场景 | 所有公共数据类型的可选 `serde` 功能 |
| 可发现 | 一个门面 crate（`scenekit`）重新导出所有内容 |

### 非目标

- scenekit **不**实现游戏引擎 ECS。它管理场景图，而非实体系统。
- scenekit **不**拥有窗口或事件循环。它接受 `wgpu::Surface`；调用者管理窗口。
- 稳定的 v1 核心**不**实现物理模拟。通过 `scenekit-raycaster` 进行碰撞检测仅用于拾取；未来的物理属于可选的 `scenekit-physics` 桥接。
- 稳定的 v1 核心**不**包含音频。未来的音频属于可选的 `scenekit-audio` crate。

### 与 Animato 的关系

```
animato (计算动画值)
    ↓  通过 scenekit-animato 桥接
scenekit (将这些值应用于 3D 变换、材质、相机)
    ↓  通过 scenekit-renderer
wgpu (绘制像素)
```

Animato 是可选依赖项。scenekit 完全可以不使用它。

---

## 2. 为什么使用工作区 — 而不是单一 crate

单一的 `src/` crate 对于 3D 库来说很快变得难以管理。scenekit 从第一天起就使用 Cargo 工作区解决这个问题。

**好处：**

- **编译时隔离。** 对 `scenekit-post` 的更改不会重新编译 `scenekit-math`。
- **清晰的所有权。** 每个 crate 只有一个职责。从事 PBR 材质的贡献者只需要理解 `scenekit-material`。
- **细粒度依赖。** 只需要场景图的用户添加 `scenekit-scene`。他们永远不会下载 `wgpu` 或 `gltf`。
- **并行编译。** Cargo 并行编译独立的 crate。
- **独立所有权。** 每个 crate 以相同的稳定工作区版本发布，但每个 crate 保持自己的依赖接口和实现边界。
- **可选 GPU。** 数学和场景层是纯 Rust —— GPU crate 是可选的。

---

## 3. 工作区布局

```
scenekit/
├── Cargo.toml                          ← 工作区根（这里没有 [lib]）
├── README.md
├── ARCHITECTURE.md                     ← 本文件
├── ROADMAP.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── LICENSE-MIT
├── LICENSE-APACHE
│
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                      ← 代码检查、测试、no_std、WASM、文档、覆盖率、包、基准测试
│   │   ├── pages.yml                   ← Leptos/Trunk GitHub Pages 部署
│   │   └── publish.yml                 ← 标签驱动的 cargo publish 和 GitHub Release
│   └── ISSUE_TEMPLATE/
│       ├── bug_report.md
│       └── feature_request.md
│
├── crates/
│   ├── scenekit-math/                     ← Vec2/3/4, Mat4, Quat, Transform, Ray, AABB (no_std)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── vec.rs                  ← Vec2, Vec3, Vec4
│   │       ├── mat.rs                  ← Mat3, Mat4
│   │       ├── quat.rs                 ← 四元数、旋转辅助工具
│   │       ├── euler.rs                ← 欧拉角 (XYZ/YXZ/ZYX 顺序)
│   │       ├── transform.rs            ← 变换 (位置 + 旋转 + 缩放)
│   │       ├── ray.rs                  ← Ray3, 参数化相交
│   │       ├── bounds.rs               ← AABB, 球体包围盒
│   │       ├── plane.rs                ← 平面 (法线 + 距离), 半空间测试
│   │       ├── spherical.rs            ← 球坐标 (半径, phi, theta)
│   │       └── cylindrical.rs          ← 柱坐标 (半径, theta, y)
│   │
│   ├── scenekit-core/                     ← Trait、ID、错误、Color (no_std)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── traits.rs               ← Renderable, Bounded, Resizable, Drawable
│   │       ├── ids.rs                  ← NodeId, MeshId, MaterialId, TextureId, LightId
│   │       ├── color.rs                ← Color (RGBA f32), ColorSpace 枚举
│   │       └── error.rs                ← scenekitError, LoadError, GpuError
│   │
│   ├── scenekit-scene/                    ← SceneGraph, SceneNode, 变换层次结构
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── graph.rs                ← SceneGraph (基于 slot-map 的节点存储)
│   │       ├── node.rs                 ← SceneNode, NodeKind 枚举
│   │       ├── transform.rs            ← 本地/世界变换传播
│   │       ├── visitor.rs              ← 深度优先遍历、BFS 迭代器
│   │       ├── fog.rs                  ← 雾 (线性), FogExp2 (指数密度)
│   │       ├── lod.rs                  ← LodGroup: 基于距离的几何切换
│   │       └── sprite.rs               ← Sprite: 面向相机的广告牌四边形
│   │
│   ├── scenekit-camera/                   ← 相机类型和投影数学
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── perspective.rs          ← PerspectiveCamera (fov, aspect, near, far)
│   │       ├── orthographic.rs         ← OrthographicCamera (left/right/top/bottom)
│   │       ├── cube_camera.rs          ← CubeCamera (6 面捕获用于环境贴图)
│   │       ├── frustum.rs              ← 视锥体平面、可见性测试
│   │       └── controller.rs           ← OrbitController, FlyController (std 功能)
│   │
│   ├── scenekit-mesh/                     ← 几何缓冲区和图元生成器
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── geometry.rs             ← Geometry: 顶点、法线、UV、索引
│   │       ├── mesh.rs                 ← Mesh = Geometry + MaterialId
│   │       ├── buffer.rs               ← BufferLayout, VertexAttribute, IndexFormat
│   │       ├── morph.rs                ← MorphTarget: 面部/变形动画的混合形状
│   │       ├── primitives/
│   │       │   ├── mod.rs
│   │       │   ├── box_prim.rs         ← BoxGeometry(w, h, d, segments)
│   │       │   ├── sphere.rs           ← SphereGeometry(radius, widthSeg, heightSeg)
│   │       │   ├── plane.rs            ← PlaneGeometry(w, h, wSeg, hSeg)
│   │       │   ├── cylinder.rs         ← CylinderGeometry(top, bottom, height, seg)
│   │       │   ├── cone.rs             ← ConeGeometry(radius, height, radialSeg)
│   │       │   ├── capsule.rs          ← CapsuleGeometry(radius, height, rings, seg)
│   │       │   ├── torus.rs            ← TorusGeometry(radius, tube, tubeSeg, radSeg)
│   │       │   ├── torus_knot.rs       ← TorusKnotGeometry(radius, tube, p, q)
│   │       │   ├── icosphere.rs        ← IcosphereGeometry(radius, subdivisions)
│   │       │   ├── circle.rs           ← CircleGeometry(radius, segments, arc)
│   │       │   ├── ring.rs             ← RingGeometry(inner, outer, thetaSeg, phiSeg)
│   │       │   ├── lathe.rs            ← LatheGeometry(points, segments, arc)
│   │       │   ├── extrude.rs          ← ExtrudeGeometry(shape, depth, bevel)
│   │       │   ├── tube.rs             ← TubeGeometry(path, tubularSeg, radius)
│   │       │   └── shape_geom.rs       ← ShapeGeometry(shape) — 2D 形状 → 三角化网格
│   │       ├── instanced.rs            ← InstancedMesh (变换数组 + 间接绘制)
│   │       └── batched.rs              ← BatchedMesh (多几何单次绘制调用)
│   │
│   ├── scenekit-material/                 ← Material trait 和内置材质类型
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── traits.rs               ← Material trait, ShaderSource, PipelineKey
│   │       ├── pbr.rs                  ← PbrMaterial (反照率、金属度、粗糙度、AO)
│   │       ├── physical.rs             ← PhysicalMaterial (清漆、光泽、透射、IOR)
│   │       ├── unlit.rs                ← UnlitMaterial (颜色/纹理，无光照)
│   │       ├── lambert.rs              ← LambertMaterial (仅漫反射，比 PBR 更快)
│   │       ├── toon.rs                 ← ToonMaterial (卡通着色，渐变贴图)
│   │       ├── normal.rs               ← NormalMaterial (调试: 表面法线 → RGB)
│   │       ├── wireframe.rs            ← WireframeMaterial
│   │       ├── depth.rs                ← DepthMaterial (用于阴影通道)
│   │       ├── line.rs                 ← LineMaterial (宽度、虚线、颜色)
│   │       ├── points.rs               ← PointsMaterial (点大小、衰减)
│   │       └── shader.rs               ← ShaderMaterial (自定义 WGSL、uniform 槽)
│   │
│   ├── scenekit-light/                    ← 灯光类型和阴影贴图管理
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── ambient.rs              ← AmbientLight (颜色、强度)
│   │       ├── directional.rs          ← DirectionalLight (方向、颜色、强度、阴影)
│   │       ├── point.rs                ← PointLight (位置、颜色、强度、衰减)
│   │       ├── spot.rs                 ← SpotLight (位置、目标、角度、半影)
│   │       ├── hemisphere.rs           ← HemisphereLight (天空颜色、地面颜色)
│   │       ├── area.rs                 ← AreaLight (矩形发射器、LTC 近似)
│   │       ├── probe.rs                ← LightProbe (基于 SH 的环境光照，从 v0.4 的原始样本)
│   │       └── shadow.rs               ← ShadowMap, ShadowConfig (PCF, 偏移, 级联)
│   │
│   ├── scenekit-texture/                  ← 纹理加载、采样、图集
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── texture.rs              ← Texture2D, TextureCube, Texture3D
│   │       ├── sampler.rs              ← Sampler (过滤、包裹、各向异性)
│   │       ├── atlas.rs                ← TextureAtlas (精灵表、UV 矩形打包)
│   │       ├── video.rs                ← VideoTexture (从视频源逐帧更新)
│   │       ├── mipmap.rs               ← CPU mipmap 生成
│   │       └── format.rs               ← TextureFormat 枚举、压缩 (BC, ASTC, ETC2)
│   │
│   ├── scenekit-renderer/                 ← wgpu 渲染管线和帧循环
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── renderer.rs             ← Renderer: 拥有 Device, Queue, Surface
│   │       ├── pipeline.rs             ← 以 PipelineKey 为键的 RenderPipeline 缓存
│   │       ├── pass/
│   │       │   ├── mod.rs
│   │       │   ├── shadow_pass.rs      ← 仅深度通道用于阴影贴图
│   │       │   ├── geometry_pass.rs    ← G-buffer 通道 (延迟路径)
│   │       │   ├── lighting_pass.rs    ← 延迟光照解析
│   │       │   └── forward_pass.rs     ← 前向+ 通道 (透明物体默认)
│   │       ├── gpu_scene.rs            ← 将 SceneGraph 数据上传到 GPU 缓冲区
│   │       ├── culling.rs              ← 视锥体 + 遮挡剔除
│   │       ├── sort.rs                 ← 透明物体深度排序
│   │       └── frame.rs                ← FrameContext, 每帧 uniform 缓冲区
│   │
│   ├── scenekit-loader/                   ← 3D 格式和图像的资产加载器
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── gltf.rs                 ← GLTF 2.0 加载器 (网格、材质、蒙皮、动画)
│   │       ├── obj.rs                  ← Wavefront OBJ + MTL 加载器
│   │       ├── stl.rs                  ← STL 加载器 (3D 打印格式)
│   │       ├── fbx.rs                  ← FBX 加载器 (Autodesk 交换)
│   │       ├── draco.rs                ← Draco 网格解压缩 (Google)
│   │       ├── image.rs                ← PNG/JPEG/WebP/KTX2 → Texture2D
│   │       ├── hdr.rs                  ← HDR/EXR → TextureCube 用于 IBL
│   │       └── cache.rs                ← AssetCache (去重、异步加载、热重载)
│   │
│   ├── scenekit-post/                     ← 后处理效果管线
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── stack.rs                ← PostStack: 有序效果链
│   │       ├── bloom.rs                ← Bloom (阈值、强度、模糊通道)
│   │       ├── ssao.rs                 ← SSAO (屏幕空间环境光遮蔽)
│   │       ├── tonemap.rs              ← ToneMapper (ACES, Reinhard, Filmic, AgX)
│   │       ├── fxaa.rs                 ← FXAA (快速近似抗锯齿)
│   │       ├── taa.rs                  ← TAA (时间抗锯齿、抖动矩阵)
│   │       ├── smaa.rs                 ← SMAA (增强型子像素形态抗锯齿)
│   │       ├── dof.rs                  ← 景深 (散景、光圈、焦距)
│   │       ├── fog.rs                  ← 体积雾 (指数、基于高度)
│   │       ├── outline.js              ← 轮廓效果 (选中对象高亮)
│   │       └── motion_blur.js          ← 逐物体运动模糊 (速度缓冲区)
│   │
│   ├── scenekit-raycaster/                ← 光线-场景相交和 BVH 加速
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── raycaster.rs            ← Raycaster: 向 SceneGraph 投射光线
│   │       ├── intersection.rs         ← 相交结果 (节点、距离、UV、法线)
│   │       ├── bvh.rs                  ← BVH (包围体层次结构、SAH 构建)
│   │       └── tests.rs                ← 光线-AABB、光线-三角形、光线-球体测试
│   │
│   ├── scenekit-animato/                  ← 桥接: animato 动画 → scenekit 变换
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── values.rs               ← AnimVec3, AnimQuat, AnimColor 包装器
│   │       ├── tracks.rs               ← Scalar/Vec3/Quat/Color/Bool 轨道
│   │       ├── scene.rs                ← NodeAnimator: 将轨道绑定到 NodeId
│   │       ├── camera.rs               ← CameraAnimator 和 CameraStoreMut
│   │       ├── material.rs             ← 用于 PBR 字段的 MaterialAnimator
│   │       ├── skeleton.rs             ← SkinnedMeshAnimator: 驱动骨骼变换
│   │       └── driver.rs               ← scenekitAnimationDriver: 更新所有绑定的动画器
│   │
│   ├── scenekit-wasm/                     ← WebGPU / WebGL2 浏览器集成
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── input.rs                ← DOM 键/按钮映射辅助工具
│   │       └── web.rs                  ← BrowserRenderer, WebRenderer, WebGlRenderer, 生成的场景
│   │
│   ├── scenekit-helpers/                  ← 调试可视化辅助工具
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── grid.rs                 ← GridHelper (可配置的网格平面)
│   │       ├── axes.rs                 ← AxesHelper (RGB XYZ 轴线)
│   │       ├── bounding_box.rs         ← BoundingBoxHelper (线框 AABB)
│   │       ├── arrow.rs                ← ArrowHelper (方向箭头网格)
│   │       ├── light_helper.rs         ← SpotLightHelper, PointLightHelper, DirLightHelper
│   │       ├── camera_helper.rs        ← CameraHelper (视锥体线框)
│   │       └── skeleton_helper.rs      ← SkeletonHelper (骨骼可视化)
│   │
│   ├── scenekit-input/                    ← 共享输入状态类型
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── pointer.rs              ← PointerState, PointerButton, PointerEvent
│   │       ├── keyboard.rs             ← KeyboardState, KeyCode, Modifiers
│   │       ├── touch.rs                ← TouchState, TouchPoint, 捏合/旋转手势
│   │       └── gamepad.rs              ← GamepadState, GamepadButton, 轴
│   │
│   └── scenekit/                          ← 门面 crate — 用户添加到 Cargo.toml 的那个
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs                  ← pub use 来自每个子 crate 的所有内容
│
├── examples/
│   ├── hello_cube.rs                   ← 旋转立方体，无光照材质
│   ├── pbr_sphere.rs                   ← 带有基于图像光照的 PBR 球体
│   ├── physical_material.rs             ← 清漆车漆 + 玻璃透射
│   ├── toon_shading.rs                 ← 带有自定义渐变贴图的 ToonMaterial
│   ├── gltf_scene.rs                   ← 加载并显示 GLTF 文件
│   ├── shadow_demo.rs                  ← 方向光 + PCF 阴影贴图
│   ├── raycasting.rs                   ← 使用 BVH 的鼠标拾取
│   ├── post_processing.rs              ← 完整 PostStack: SSAO + Bloom + ToneMap + TAA
│   ├── instanced_mesh.rs               ← 10,000 个实例化立方体
│   ├── animato_integration.rs          ← 弹簧驱动的相机 + 补间材质颜色
│   ├── orbit_camera.rs                 ← 带有鼠标输入的 OrbitController
│   ├── lod_demo.rs                     ← 带有基于距离几何交换的 LodGroup
│   ├── morph_targets.rs                ← 来自 GLTF 的面部混合形状
│   ├── fog_demo.rs                     ← 场景雾 + 体积后处理雾
│   ├── helpers_demo.rs                 ← GridHelper + AxesHelper + LightHelpers
│   ├── sprite_particles.rs             ← 使用 Sprites 的广告牌粒子系统
│   ├── environment_map.rs              ← CubeCamera IBL 捕获 + 反射
│   └── wasm_viewer/                    ← 生成的场景浏览器查看器
│       ├── src/lib.rs
│       └── www/index.html
│
├── website/                            ← 部署在 /scenekit/ 的 Leptos CSR 站点
│   ├── Cargo.toml                      ← 独立工作区以隔离网站依赖
│   ├── Trunk.toml
│   ├── index.html
│   ├── public/
│   └── src/
│
├── docs/                               ← 稳定的 v1 用户文档
│
├── benches/
│   ├── scene_graph_bench.rs            ← 10K 节点图遍历 + 变换传播
│   ├── render_bench.rs                 ← 1K / 10K / 100K 三角形帧时间
│   ├── bvh_bench.rs                    ← BVH 构建 + 1K 光线查询
│   ├── mesh_gen_bench.rs               ← 图元生成吞吐量
│   └── culling_bench.rs                ← 10K 物体视锥体剔除
│
└── tests/
    ├── scene_hierarchy.rs              ← 父/子、世界变换正确性
    ├── camera_frustum.rs               ← 视锥体平面提取、可见性测试
    ├── mesh_primitives.rs              ← 顶点数量、法线有效性、UV 范围
    ├── material_pipeline.rs            ← 管线缓存命中/未命中正确性
    ├── loader_gltf.rs                  ← 参考 GLTF 资产的往返加载
    └── raycaster_correctness.rs        ← 光线-三角形相交精度
```

### 未来工作区扩展

上述布局描述了已发布的 v1.1 工作区。未来的系统如音频、物理、XR、编辑器工具、粒子、地形、天空、水面和着色器节点应仅在开始实现时作为**新的可选 crate** 添加。它们不应在 `Cargo.toml` 中列为活动工作区成员，直到 crate 目录、测试、文档、示例和功能标志存在。

计划的未来 crate 可以像这样扩展工作区：

```text
crates/
├── scenekit-nodes/       ← 可选着色器图和节点材质系统
├── scenekit-particles/   ← 可选 CPU/GPU 粒子系统
├── scenekit-terrain/     ← 可选高度图地形和分块 LOD
├── scenekit-sky/         ← 可选程序化天空和大气
├── scenekit-water/       ← 可选水面、反射和折射辅助工具
├── scenekit-xr/          ← 可选 WebXR/OpenXR 集成
├── scenekit-audio/       ← 可选空间音频桥接
├── scenekit-physics/     ← 可选 Rapier/Jolt 物理桥接
├── scenekit-editor/      ← 可选可视化编辑器外壳和工具
└── scenekit-ui/          ← 可选跨平台调试 UI 覆盖层
```

添加未来 crate 的规则：

- 将 crate 保持在集中的门面功能后面可选；
- 将重量级第三方依赖排除在默认功能之外；
- 记录桌面、移动、WebGPU 和 WebGL 支持级别；
- 在列为已发布之前，至少添加一个示例、一个 API 文档页面和专注的测试；
- 除非记录了弃用路径，否则避免将稳定的 v1 API 移动到新 crate 中。

## 稳定契约

稳定的 API 契约保持 scenekit 的模块化：

- 默认门面功能是 CPU 创作加上 BVH 光线投射和辅助几何体；
- 加载器、渲染器、后处理、Animato 和 WASM 路径保持可选；
- 场景数据保持渲染器无关，GPU 资源保持渲染器拥有；
- 公共 API 变更应该是累加的，在移除之前有弃用；
- 桌面和移动应用程序是一流的运行时目标，通过 `wgpu` surface；
- 网站是由 Trunk 构建并部署到 GitHub Pages `/scenekit/` 的静态 Leptos CSR 应用，但它只是一个部署目标。

### 根 `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = [
    "crates/scenekit-math",
    "crates/scenekit-core",
    "crates/scenekit-scene",
    "crates/scenekit-camera",
    "crates/scenekit-mesh",
    "crates/scenekit-material",
    "crates/scenekit-light",
    "crates/scenekit-texture",
    "crates/scenekit-renderer",
    "crates/scenekit-loader",
    "crates/scenekit-post",
    "crates/scenekit-raycaster",
    "crates/scenekit-animato",
    "crates/scenekit-wasm",
    "crates/scenekit-helpers",
    "crates/scenekit-input",
    "crates/scenekit",
]

[workspace.package]
version      = "0.1.0"
edition      = "2024"
license      = "MIT OR Apache-2.0"
repository   = "https://github.com/scenekit/scenekit"
authors      = ["scenekit"]
rust-version = "1.89"

[workspace.dependencies]
# 内部 crate — 版本固定到工作区
scenekit-math       = { path = "crates/scenekit-math",       version = "0.1" }
scenekit-core       = { path = "crates/scenekit-core",       version = "0.1" }
scenekit-scene      = { path = "crates/scenekit-scene",      version = "0.1" }
scenekit-camera     = { path = "crates/scenekit-camera",     version = "0.1" }
scenekit-mesh       = { path = "crates/scenekit-mesh",       version = "0.1" }
scenekit-material   = { path = "crates/scenekit-material",   version = "0.1" }
scenekit-light      = { path = "crates/scenekit-light",      version = "0.1" }
scenekit-texture    = { path = "crates/scenekit-texture",    version = "0.1" }
scenekit-loader     = { path = "crates/scenekit-loader",     version = "0.1" }
scenekit-post       = { path = "crates/scenekit-post",       version = "0.1" }
scenekit-renderer   = { path = "crates/scenekit-renderer",   version = "0.1" }
scenekit-raycaster  = { path = "crates/scenekit-raycaster",  version = "0.1" }
scenekit-animato    = { path = "crates/scenekit-animato",    version = "0.1" }
scenekit-wasm       = { path = "crates/scenekit-wasm",       version = "0.1" }
scenekit-helpers    = { path = "crates/scenekit-helpers",    version = "0.1" }
scenekit-input      = { path = "crates/scenekit-input",      version = "0.1" }

# 外部 crate — 共享版本固定
wgpu             = { version = "29.0.3" }
bytemuck         = { version = "0.1",   features = ["derive"] }
serde            = { version = "0.1",   features = ["derive"] }
image            = { version = "0.25.10", default-features = false }
gltf             = { version = "1.4.1",   default-features = false }
ktx2             = { version = "0.4.0" }
tobj             = { version = "4.0.3", default-features = false }
stl_io           = { version = "0.11.0" }
reqwest          = { version = "0.12", default-features = false }
slotmap          = { version = "0.1" }
ahash            = { version = "0.8" }
log              = { version = "0.4" }
winit            = { version = "0.30.13" }
raw-window-handle = { version = "0.6" }
pollster         = { version = "0.4" }
wasm-bindgen     = { version = "0.2" }
js-sys           = { version = "0.3" }
web-sys          = { version = "0.3", features = ["HtmlCanvasElement", "Window"] }
animato          = { version = "1.4.0", default-features = false }
criterion        = { version = "0.5", features = ["html_reports"] }
approx           = { version = "0.5" }
thiserror        = { version = "2" }
```

---

## 4. Crate 规范

---

### 4.1 `scenekit-math`

**职责：** 所有 3D 数学图元。这是其他每个 crate 构建的基础。必须在 `no_std` 环境中编译，零外部依赖。

**依赖规则：** 此 crate 仅依赖 `libcore` 和可选的 `libm`（用于 `no_std` 三角函数）。

#### `src/vec.rs`

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 { pub x: f32, pub y: f32 }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4 { pub x: f32, pub y: f32, pub z: f32, pub w: f32 }

impl Vec3 {
    pub const ZERO: Self;
    pub const ONE:  Self;
    pub const X:    Self;    // (1, 0, 0)
    pub const Y:    Self;    // (0, 1, 0)
    pub const Z:    Self;    // (0, 0, 1)
    pub const UP:   Self;    // (0, 1, 0) — 世界向上

    pub fn dot(self, rhs: Self) -> f32;
    pub fn cross(self, rhs: Self) -> Self;
    pub fn length(self) -> f32;
    pub fn length_squared(self) -> f32;
    pub fn normalize(self) -> Self;
    pub fn lerp(self, rhs: Self, t: f32) -> Self;
    pub fn distance(self, rhs: Self) -> f32;
    pub fn reflect(self, normal: Self) -> Self;
    pub fn angle_between(self, rhs: Self) -> f32;      // 弧度
}
```

#### `src/mat.rs`

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat4 { cols: [Vec4; 4] }    // 列主序，匹配 wgpu/WGSL 约定

impl Mat4 {
    pub const IDENTITY: Self;

    pub fn perspective(fov_y_rad: f32, aspect: f32, near: f32, far: f32) -> Self;
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self;
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self;
    pub fn from_translation(v: Vec3) -> Self;
    pub fn from_rotation(q: Quat) -> Self;
    pub fn from_scale(v: Vec3) -> Self;
    pub fn from_trs(t: Vec3, r: Quat, s: Vec3) -> Self;    // 一次调用组合 TRS

    pub fn mul_mat4(self, rhs: Self) -> Self;
    pub fn mul_vec4(self, rhs: Vec4) -> Vec4;
    pub fn mul_vec3(self, rhs: Vec3) -> Vec3;               // 应用齐次除法
    pub fn inverse(self) -> Option<Self>;
    pub fn transpose(self) -> Self;
    pub fn to_cols_array(self) -> [f32; 16];                // 用于 wgpu 缓冲区上传
}
```

#### `src/quat.rs`

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat { pub x: f32, pub y: f32, pub z: f32, pub w: f32 }

impl Quat {
    pub const IDENTITY: Self;

    pub fn from_axis_angle(axis: Vec3, angle_rad: f32) -> Self;
    pub fn from_euler_xyz(x: f32, y: f32, z: f32) -> Self;    // 角度为弧度
    pub fn from_rotation_arc(from: Vec3, to: Vec3) -> Self;    // 两个方向之间的最小旋转

    pub fn mul_quat(self, rhs: Self) -> Self;
    pub fn mul_vec3(self, rhs: Vec3) -> Vec3;
    pub fn conjugate(self) -> Self;
    pub fn inverse(self) -> Self;
    pub fn normalize(self) -> Self;
    pub fn slerp(self, rhs: Self, t: f32) -> Self;             // 球面线性插值
    pub fn to_mat4(self) -> Mat4;
    pub fn to_euler_xyz(self) -> Vec3;                          // 提取欧拉角
    pub fn angle_between(self, rhs: Self) -> f32;
}
```

#### `src/transform.rs`

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation:    Quat,
    pub scale:       Vec3,
}

impl Transform {
    pub const IDENTITY: Self;

    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self;
    pub fn from_translation(v: Vec3) -> Self;
    pub fn from_rotation(q: Quat) -> Self;
    pub fn looking_at(eye: Vec3, target: Vec3, up: Vec3) -> Self;

    pub fn to_mat4(self) -> Mat4;
    pub fn mul_transform(self, rhs: Self) -> Self;    // 组合两个变换
    pub fn inverse(self) -> Self;
    pub fn forward(self) -> Vec3;     // 局部空间中的 -Z，转换到世界
    pub fn right(self) -> Vec3;       // 局部空间中的 X
    pub fn up(self) -> Vec3;          // 局部空间中的 Y

    pub fn translate_by(self, delta: Vec3) -> Self;
    pub fn rotate_by(self, q: Quat) -> Self;
    pub fn scale_by(self, s: Vec3) -> Self;
}
```

#### `src/ray.rs` 和 `src/bounds.rs`

```rust
pub struct Ray3 {
    pub origin:    Vec3,
    pub direction: Vec3,    // 始终归一化
}

impl Ray3 {
    pub fn at(&self, t: f32) -> Vec3;
    pub fn intersect_aabb(&self, aabb: &Aabb) -> Option<f32>;
    pub fn intersect_sphere(&self, center: Vec3, radius: f32) -> Option<f32>;
    pub fn intersect_triangle(&self, a: Vec3, b: Vec3, c: Vec3) -> Option<(f32, Vec2)>;
    // 返回 (t, 重心 UV) 或 None
}

pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn from_points(points: &[Vec3]) -> Self;
    pub fn center(&self) -> Vec3;
    pub fn half_extents(&self) -> Vec3;
    pub fn contains_point(&self, p: Vec3) -> bool;
    pub fn intersects_aabb(&self, other: &Self) -> bool;
    pub fn transform(&self, mat: Mat4) -> Self;    // 保守变换
    pub fn merge(&self, other: &Self) -> Self;
    pub fn surface_area(&self) -> f32;             // 用于 SAH BVH 构建器
}
```

#### `src/euler.rs`

```rust
/// 欧拉角分解的旋转顺序。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RotationOrder { XYZ, YXZ, ZXY, ZYX, YZX, XZY }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Euler {
    pub x: f32,    // 俯仰角，弧度
    pub y: f32,    // 偏航角，弧度
    pub z: f32,    // 滚转角，弧度
    pub order: RotationOrder,
}

impl Euler {
    pub fn new(x: f32, y: f32, z: f32, order: RotationOrder) -> Self;
    pub fn from_quat(q: Quat, order: RotationOrder) -> Self;
    pub fn from_mat4(m: Mat4, order: RotationOrder) -> Self;
    pub fn to_quat(self) -> Quat;
}
```

#### `src/plane.rs`

```rust
/// 由单位法线和从原点的有符号距离定义的平面。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    pub normal:   Vec3,    // 单位长度
    pub distance: f32,     // 从原点的有符号距离
}

impl Plane {
    pub fn from_normal_and_point(normal: Vec3, point: Vec3) -> Self;
    pub fn from_three_points(a: Vec3, b: Vec3, c: Vec3) -> Self;
    pub fn signed_distance(&self, p: Vec3) -> f32;
    pub fn project_point(&self, p: Vec3) -> Vec3;
    pub fn intersect_ray(&self, ray: &Ray3) -> Option<f32>;
    pub fn intersect_line(&self, a: Vec3, b: Vec3) -> Option<Vec3>;
}
```

#### `src/spherical.rs` 和 `src/cylindrical.rs`

```rust
/// 球坐标 — OrbitController 内部使用。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spherical {
    pub radius: f32,
    pub phi:    f32,    // 从 Y 轴的极角 (0..π)
    pub theta:  f32,    // XZ 平面中的方位角 (0..2π)
}

impl Spherical {
    pub fn from_vec3(v: Vec3) -> Self;
    pub fn to_vec3(self) -> Vec3;
    pub fn clamp_phi(self, min: f32, max: f32) -> Self;
}

/// 柱坐标 — 用于径向放置。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cylindrical {
    pub radius: f32,
    pub theta:  f32,    // XZ 平面中的角度
    pub y:      f32,    // 高度
}

impl Cylindrical {
    pub fn from_vec3(v: Vec3) -> Self;
    pub fn to_vec3(self) -> Vec3;
}
```

**`Cargo.toml`:**

```toml
[package]
name        = "scenekit-math"
description = "scenekit 渲染库的 3D 数学图元。"

[features]
default = ["std"]
std     = []
libm    = ["dep:libm"]    # 通过 libm 启用 no_std 三角函数
serde   = ["dep:serde"]
approx  = ["dep:approx"]  # 用于测试的 approx::AbsDiffEq 实现

[dependencies]
libm  = { version = "0.2", optional = true }
serde = { workspace = true, optional = true }
approx = { version = "0.5", optional = true }
```

---

### 4.2 `scenekit-core`

**职责：** 共享 trait、ID 新类型、颜色类型和错误类型。其他每个 crate 都从这里导入，但此 crate 仅从 `scenekit-math` 导入。

**依赖于：** `scenekit-math`

#### `src/ids.rs`

```rust
// 所有 ID 都是 u64 上的 Copy 新类型 — 零成本、哈希友好。
// 由 scenekit-scene / scenekit-renderer 中的 SlotMap 生成；用户从不直接构造。

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MeshId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MaterialId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LightId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CameraId(u64);
```

#### `src/traits.rs`

```rust
pub trait Bounded {
    fn aabb(&self) -> Aabb;
    fn bounding_sphere(&self) -> (Vec3, f32);    // 中心、半径
}

// 仅在 "gpu" 功能下可用（bytemuck 是 no_std 但可选）
#[cfg(feature = "gpu")]
pub trait GpuUpload {
    type GpuData: bytemuck::Pod;
    fn to_gpu(&self) -> Self::GpuData;
}

pub trait Named {
    fn name(&self) -> &str;
    fn set_name(&mut self, name: impl Into<String>);
}
```

#### `src/color.rs`

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,    // 0.0..=1.0
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Self;
    pub const BLACK: Self;
    pub const TRANSPARENT: Self;
    pub const RED: Self;
    pub const GREEN: Self;
    pub const BLUE: Self;

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self;
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self;
    pub fn from_hex(hex: u32) -> Self;              // 例如 0xFF8800FF
    pub fn from_srgb_u8(r: u8, g: u8, b: u8) -> Self;

    pub fn to_linear(self) -> Self;                 // sRGB → 线性 (用于 PBR)
    pub fn to_srgb(self) -> Self;
    pub fn lerp(self, rhs: Self, t: f32) -> Self;
    pub fn to_array(self) -> [f32; 4];
}
```

---

### 4.3 `scenekit-scene`

**职责：** 场景图。拥有节点层次结构、其变换以及附加的资源（网格、灯光、相机）。零 GPU 依赖。

**依赖于：** `scenekit-math`, `scenekit-core`

#### `src/graph.rs`

```rust
pub struct SceneGraph {
    nodes:       SlotMap<PrivateSceneKey, NodeRecord>,
    roots:       Vec<NodeId>,                  // 顶级节点（无父节点）
    id_to_key:   Vec<Option<PrivateSceneKey>>, // 图本地公共句柄
    next_id:     u64,                          // 在图内永不重用
    dirty_roots: Vec<NodeId>,                  // 脏子树入口点
    fog:         Option<Fog>,
}

impl SceneGraph {
    pub fn new() -> Self;
    pub fn with_capacity(capacity: usize) -> Self;

    // 节点管理
    pub fn add(&mut self, node: SceneNode) -> NodeId;
    pub fn add_child(&mut self, parent: NodeId, node: SceneNode) -> Result<NodeId, ValidationError>;
    pub fn remove(&mut self, id: NodeId) -> Result<(), ValidationError>;
    pub fn get(&self, id: NodeId) -> Option<&SceneNode>;
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut SceneNode>;

    // 变换查询 — 通过显式脏子树传播更新
    pub fn update_world_transforms(&mut self);
    pub fn world_transform(&self, id: NodeId) -> Option<Transform>;
    pub fn world_matrix(&self, id: NodeId) -> Option<Mat4>;
    pub fn set_local_transform(&mut self, id: NodeId, t: Transform) -> Result<(), ValidationError>;

    // 层次结构
    pub fn parent(&self, id: NodeId) -> Option<NodeId>;
    pub fn children(&self, id: NodeId) -> Option<&[NodeId]>;
    pub fn roots(&self) -> &[NodeId];
    pub fn reparent(&mut self, node: NodeId, new_parent: Option<NodeId>) -> Result<(), ValidationError>;

    // 遍历
    pub fn iter_depth_first(&self) -> DepthFirstIter<'_>;
    pub fn iter_breadth_first(&self) -> BreadthFirstIter<'_>;

    // 查询
    pub fn find_by_name(&self, name: &str) -> Option<NodeId>;
}
```

`NodeId` 仍然是来自 `scenekit-core` 的公共 `u64` 句柄。`scenekit-scene` 在内部使用私有 SlotMap 键，并维护一个图本地句柄表，因此 SlotMap 键布局永远不会成为公共 API。修改层次结构操作对缺失 ID 返回 `ValidationError::InvalidId`，对创建循环的重新父化返回 `ValidationError::InvalidState`。

#### `src/node.rs`

```rust
pub struct SceneNode {
    pub name:      String,
    pub transform: Transform,              // 本地变换
    pub visible:   bool,
    pub layer:     u32,                    // 用于相机剔除层的位掩码
    pub kind:      NodeKind,
}

pub enum NodeKind {
    Empty,
    Group,    // 逻辑分组，无渲染数据
    Mesh   { mesh_id: MeshId, material_id: MaterialId },
    Light  { light_id: LightId },
    Camera { camera_id: CameraId },
    Sprite(Sprite),
}

// 构建器模式，用于符合人体工程学的构造：
let node = SceneNode::new("Sword")
    .transform(Transform::from_translation(Vec3::new(1.0, 0.0, 0.0)))
    .kind(NodeKind::Mesh { mesh_id, material_id })
    .visible(true)
    .layer(0b0001);
```

---

### 4.4 `scenekit-camera`

**职责：** 相机类型、投影矩阵、视锥体剔除和可选的交互式控制器。

**依赖于：** `scenekit-math`, `scenekit-core`

#### `src/perspective.rs`

```rust
pub struct PerspectiveCamera {
    pub fov_y:  f32,     // 垂直视野，弧度
    pub aspect: f32,     // 宽度 / 高度
    pub near:   f32,
    pub far:    f32,
    pub position: Vec3,
    pub target:   Vec3,
    pub up:       Vec3,
}

impl PerspectiveCamera {
    pub fn new(fov_y_deg: f32, aspect: f32, near: f32, far: f32) -> Self;
    pub fn projection_matrix(&self) -> Mat4;
    pub fn view_matrix(&self) -> Mat4;
    pub fn view_projection(&self) -> Mat4;
    pub fn frustum(&self) -> Frustum;
    pub fn screen_to_ray(&self, ndc: Vec2) -> Ray3;    // 用于从鼠标位置进行光线投射
}
```

#### `src/frustum.rs`

```rust
pub struct Frustum {
    planes: [Vec4; 6],    // [左, 右, 下, 上, 近, 远] — 法线 + 偏移
}

impl Frustum {
    pub fn from_view_projection(vp: Mat4) -> Self;    // Gribb/Hartmann 提取

    pub fn contains_point(&self, p: Vec3) -> bool;
    pub fn contains_aabb(&self, aabb: &Aabb) -> Visibility;
    pub fn contains_sphere(&self, center: Vec3, radius: f32) -> Visibility;
}

pub enum Visibility { Outside, Intersects, Inside }
```

#### `src/controller.rs` (std 功能)

```rust
pub struct OrbitController {
    pub target:       Vec3,
    pub radius:       f32,
    pub theta:        f32,    // 方位角
    pub phi:          f32,    // 极角
    pub min_radius:   f32,
    pub max_radius:   f32,
    pub damping:      f32,    // 0.0 = 瞬时, 1.0 = 冻结
}

impl OrbitController {
    pub fn on_drag(&mut self, delta: Vec2, dt: f32);
    pub fn on_scroll(&mut self, delta: f32, dt: f32);
    pub fn on_pan(&mut self, delta: Vec2, dt: f32);
    pub fn update(&mut self, dt: f32);
    pub fn camera_transform(&self) -> Transform;
}
```

---

### 4.5 `scenekit-mesh`

**职责：** CPU 端几何缓冲区和图元生成器。此 crate 不了解 GPU。

**依赖于：** `scenekit-math`, `scenekit-core`

#### `src/geometry.rs`

```rust
pub struct Geometry {
    pub positions:  Vec<Vec3>,       // 始终需要
    pub normals:    Vec<Vec3>,       // 可选 — 如果缺失则自动生成
    pub tangents:   Vec<Vec4>,       // 可选 — 用于法线贴图
    pub uvs:        Vec<Vec2>,       // UV 通道 0
    pub uvs2:       Vec<Vec2>,       // UV 通道 1 (光照贴图)
    pub colors:     Vec<Color>,      // 逐顶点颜色
    pub indices:    Option<Vec<u32>>,
    pub topology:   PrimitiveTopology,
}

impl Geometry {
    pub fn compute_normals(&mut self);       // 基于索引/非索引的平滑或平面
    pub fn compute_tangents(&mut self);      // MikkTSpace 算法
    pub fn center(&self) -> Vec3;
    pub fn aabb(&self) -> Aabb;
    pub fn merge(&self, other: &Self) -> Self;
    pub fn vertex_count(&self) -> usize;
    pub fn triangle_count(&self) -> usize;
}
```

#### 图元生成器

```rust
// 所有构造函数都返回带有位置、法线和 UV 的完全有效的 Geometry。

pub fn box_geometry(width: f32, height: f32, depth: f32,
                    width_segs: u32, height_segs: u32, depth_segs: u32) -> Geometry;

pub fn sphere_geometry(radius: f32, width_segs: u32, height_segs: u32) -> Geometry;

pub fn plane_geometry(width: f32, height: f32, width_segs: u32, height_segs: u32) -> Geometry;

pub fn cylinder_geometry(top_radius: f32, bottom_radius: f32, height: f32,
                         radial_segs: u32, height_segs: u32, open_ended: bool) -> Geometry;

pub fn torus_geometry(radius: f32, tube: f32,
                      radial_segs: u32, tubular_segs: u32) -> Geometry;

pub fn icosphere_geometry(radius: f32, subdivisions: u32) -> Geometry;

pub fn capsule_geometry(radius: f32, height: f32, rings: u32, segments: u32) -> Geometry;
```

#### `src/instanced.rs`

```rust
pub struct InstancedMesh {
    pub mesh_id:     MeshId,
    pub material_id: MaterialId,
    pub transforms:  Vec<Mat4>,    // 每个实例一个 — 作为存储缓冲区上传到 GPU
    pub count:       u32,
}

impl InstancedMesh {
    pub fn new(mesh_id: MeshId, material_id: MaterialId, capacity: u32) -> Self;
    pub fn set_transform_at(&mut self, index: u32, t: Transform);
    pub fn push(&mut self, t: Transform);
    pub fn clear(&mut self);
}
```

---

### 4.6 `scenekit-material`

**职责：** Material trait 和所有内置材质类型。定义渲染器用于缓存编译管线的 `PipelineKey`。

**依赖于：** `scenekit-math`, `scenekit-core`

> **设计决策：** `Material` trait 没有 wgpu 依赖。
> GPU 特定方法（`bind_group_layout`、`to_uniform_bytes`）位于
> `GpuMaterial` — 定义在 `scenekit-renderer` 中的 trait 扩展。这保持了
> `scenekit-material` 无 GPU 依赖，可以在没有图形上下文的情况下进行测试。

#### `src/traits.rs`

```rust
/// CPU 端材质描述 — 零 GPU 依赖。
pub trait Material: Send + Sync + 'static {
    fn pipeline_key(&self) -> PipelineKey;     // 确定使用哪个 WGSL 管线
    fn is_transparent(&self) -> bool;          // 影响渲染顺序和混合
    fn double_sided(&self) -> bool;
    fn alpha_cutoff(&self) -> Option<f32>;     // 用于 AlphaMode::Mask
}
```

#### `src/pbr.rs`

```rust
pub struct PbrMaterial {
    pub name:                 String,
    pub albedo:               Color,           // 基础颜色（线性）
    pub albedo_texture:       Option<TextureId>,
    pub metallic:             f32,             // 0.0 = 电介质, 1.0 = 金属
    pub roughness:            f32,             // 0.0 = 镜面, 1.0 = 哑光
    pub metallic_roughness_texture: Option<TextureId>,
    pub normal_texture:       Option<TextureId>,
    pub occlusion_texture:    Option<TextureId>,
    pub emissive:             Vec3,            // 自发光颜色（线性）
    pub emissive_texture:     Option<TextureId>,
    pub alpha_mode:           AlphaMode,       // Opaque / Mask(f32) / Blend
    pub double_sided:         bool,
}

pub enum AlphaMode {
    Opaque,
    Mask(f32),     // 截止阈值
    Blend,
}
```

#### `src/shader.rs`

```rust
pub struct ShaderMaterial {
    pub name:         String,
    pub vertex_wgsl:  String,       // 自定义顶点着色器源
    pub fragment_wgsl: String,      // 自定义片段着色器源
    pub uniforms:     Vec<u8>,      // 原始 uniform 缓冲区字节
    pub textures:     Vec<TextureId>,
    pub transparent:  bool,
    pub double_sided: bool,
}
```

#### `src/physical.rs`

```rust
/// 带有高级表面效果的基于物理的材质。
/// 等效于 Three.js MeshPhysicalMaterial。
pub struct PhysicalMaterial {
    // 继承所有 PbrMaterial 字段，加上：
    pub base:            PbrMaterial,
    pub clearcoat:       f32,            // 0.0..=1.0, 清漆层强度
    pub clearcoat_roughness: f32,        // 清漆层的粗糙度
    pub clearcoat_normal_texture: Option<TextureId>,
    pub sheen:           f32,            // 0.0..=1.0, 织物状光泽
    pub sheen_color:     Color,
    pub sheen_roughness: f32,
    pub transmission:    f32,            // 0.0..=1.0, 玻璃状透明度
    pub thickness:       f32,            // 透射的体积厚度
    pub ior:             f32,            // 折射率 (默认: 1.5)
    pub iridescence:     f32,            // 薄膜干涉 (肥皂泡)
    pub iridescence_ior: f32,
}
```

#### `src/toon.rs`

```rust
/// 带有离散着色带的卡通着色材质。
pub struct ToonMaterial {
    pub name:           String,
    pub color:          Color,
    pub color_texture:  Option<TextureId>,
    pub gradient_map:   Option<TextureId>,  // 用于着色步骤的 1D 渐变纹理
}
```

---

*文档版本: 0.1.0 — 基于 scenix*
*项目: scenekit — github.com/scenekit/scenekit*
*配套库: animato*
