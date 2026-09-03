# 贡献 scenekit

感谢您抽出时间参与贡献。每个 Bug 报告、功能建议、文档改进和拉取请求都让 scenekit 变得更好。

---

## 目录

1. [行为准则](#行为准则)
2. [贡献方式](#贡献方式)
3. [设置工作空间](#设置工作空间)
4. [项目结构](#项目结构)
5. [进行更改](#进行更改)
6. [提交消息](#提交消息)
7. [测试要求](#测试要求)
8. [文档要求](#文档要求)
9. [拉取请求流程](#拉取请求流程)
10. [报告 Bug](#报告-bug)
11. [建议功能](#建议功能)
12. [开发 GPU Crate](#开发-gpu-crate)
13. [Crate 版本管理](#crate-版本管理)

---

## 行为准则

请保持尊重。欢迎对代码和想法提出建设性批评；但不接受人身攻击。参与敌对行为的贡献者将被要求停止，并可能被移出项目。

---

## 贡献方式

您不必编写代码也能参与贡献：

- **报告 Bug** — 提交包含最小复现的问题
- **建议功能** — 提交描述使用场景的问题
- **改进文档** — 修正拼写错误、添加示例、澄清令人困惑的内容
- **编写示例** — 展示 scenekit 在真实场景中的用法
- **编写基准测试** — 帮助识别性能回归
- **审查拉取请求** — 阅读他人的更改并留下深思熟虑的反馈
- **编写测试** — 提高现有代码的覆盖率

---

## 设置工作空间

### 前置条件

- Rust stable 1.89 或更高版本（`rustup update stable`）
- 用于 GPU crate 测试的 GPU 或软件渲染器：
  - Linux：Mesa/lavapipe 用于无头 GPU 测试（`sudo apt install mesa-vulkan-drivers`）
  - macOS/Windows：原生 GPU 开箱即用
- `wasm-pack`（可选 — 仅用于 WASM 开发）：`cargo install wasm-pack`
- `cargo-llvm-cov`（可选 — 用于覆盖率）：`cargo install cargo-llvm-cov`

### 克隆与构建

```sh
git clone https://github.com/launcher-rs/scenekit.git
cd scenekit

# 构建所有 crate：
cargo build --workspace

# 运行所有测试：
cargo test --workspace

# 运行所有 feature 的测试：
cargo test --workspace --all-features

# 验证无 GPU 依赖的 crate 的 no_std 兼容性：
cargo test -p scenekit-math -p scenekit-core -p scenekit-input -p scenekit-scene -p scenekit-camera -p scenekit-mesh -p scenekit-material -p scenekit-light -p scenekit-texture -p scenekit-raycaster -p scenekit-helpers -p scenekit-animato --no-default-features

# 代码检查：
cargo clippy --workspace --all-features -- -D warnings

# 格式检查：
cargo fmt --check
```

### IDE 设置

打开根目录 `scenekit/` 文件夹。`rust-analyzer` 会自动检测工作空间，无需额外配置。

---

## 项目结构

```
scenekit/
├── crates/
│   ├── scenekit-math/        ← Vec2/3/4、Mat4、Quat、Transform、Ray3、AABB — 从这里开始
│   ├── scenekit-core/        ← Trait、ID、Color、错误类型
│   ├── scenekit-input/       ← PointerState、KeyboardState、按键/按钮状态
│   ├── scenekit-scene/       ← 场景图、节点、遍历、雾、精灵、LOD
│   ├── scenekit-mesh/        ← 几何缓冲区、图元、实例化、批处理
│   └── scenekit/             ← 门面 crate
├── ARCHITECTURE.md        ← 未来 crate 的长期设计
└── ROADMAP.md             ← 版本发布计划
```

版本 `0.3.0` 包含 Foundation、Scene Graph 和 Geometry 层。
未来的 crate 如 `scenekit-material`、`scenekit-light`、`scenekit-renderer` 和 `scenekit-wasm` 已在路线图中记录但尚未实现。

---

## 进行更改

### 1. 检查是否已有相关 Issue

开始之前请搜索[已打开的 Issue](https://github.com/launcher-rs/scenekit/issues)。如果没有与您的更改相关的 Issue，请先创建一个——特别是对于比拼写修正更大的改动。

### 2. Fork 并创建分支

```sh
git clone https://github.com/YOUR_USERNAME/scenekit.git
cd scenekit
git checkout -b fix/pbr-roughness-clamp
```

分支命名规范：

| 类型 | 前缀 | 示例 |
|------|------|------|
| Bug 修复 | `fix/` | `fix/shadow-acne-bias` |
| 新功能 | `feat/` | `feat/toon-material` |
| 文档 | `docs/` | `docs/pbr-guide` |
| 重构 | `refactor/` | `refactor/pipeline-cache` |
| 性能优化 | `perf/` | `perf/frustum-culling` |
| 测试 | `test/` | `test/bvh-correctness` |

### 3. 做尽可能小的改动

不要在一个 PR 中混入不相关的更改。添加新图元的 PR 不应同时修复材质 Bug。

### 4. 格式化代码

```sh
cargo fmt
```

CI 会拒绝未格式化的代码。

---

## 提交消息

使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>(<scope>): <简短描述>
```

scope 是不带 `scenekit-` 前缀的 crate 名称：

```
feat(mesh): add TorusKnotGeometry
fix(renderer): clamp shadow bias to prevent acne
perf(scene): skip dirty-flag propagation for static nodes
docs(material): add PBR roughness examples
test(math): add Quat::slerp edge case at t=0 and t=1
```

规则：
- 使用祈使语气："add"、"fix"、"update" — 而不是 "added"、"fixed"
- 第一行不超过 72 个字符
- 在脚注中引用 Issue：`Closes #42`
- 破坏性变更：在脚注中添加 `BREAKING CHANGE:`

---

## 测试要求

每个 PR 都必须包含测试，没有例外。

### 测试什么

- **新功能：** 一个在更改前失败、更改后通过的测试。
- **Bug 修复：** 一个复现该 Bug 的测试，然后修复它。
- **边界情况：** 零尺寸几何体、空场景、零强度灯光、退化三角形。

### 测试放置位置

- 单元测试 → 相关 `.rs` 文件底部的 `#[cfg(test)]` 块
- 集成测试 → 工作空间根目录的 `tests/`
- 示例 → `examples/`

### 运行测试

```sh
# 所有测试：
cargo test --workspace

# 特定 crate：
cargo test -p scenekit-mesh

# 特定测试：
cargo test -p scenekit-math quat_slerp_midpoint

# 所有 feature：
cargo test --workspace --all-features

# no_std 兼容 crate：
cargo test -p scenekit-math -p scenekit-core -p scenekit-input -p scenekit-scene -p scenekit-mesh --no-default-features
```

---

## 文档要求

每个 `pub` 项都必须有 `///` 文档注释，至少包含一句话描述。非平凡的 API 需要 `# Examples` 部分，包含可运行的代码块。

```rust
/// 计算透视投影矩阵。
///
/// `fov_y_rad` 是垂直视场角（弧度）。
/// `aspect` 是宽度除以高度。
///
/// # Examples
///
/// ```rust
/// use scenekit_math::Mat4;
/// use std::f32::consts::PI;
///
/// let proj = Mat4::perspective(PI / 3.0, 16.0 / 9.0, 0.1, 1000.0);
/// ```
pub fn perspective(fov_y_rad: f32, aspect: f32, near: f32, far: f32) -> Self { ... }
```

检查文档是否正确渲染：

```sh
cargo doc --workspace --all-features --open
```

---

## 拉取请求流程

### 开始前

执行以下检查清单：

```sh
cargo fmt --check
cargo clippy --workspace --all-features -- -D warnings
cargo test --workspace --all-features
cargo test -p scenekit-math -p scenekit-core -p scenekit-input --no-default-features
cargo doc --workspace --all-features
```

所有项目必须通过 — 零警告、零失败。

### 创建 PR

- 完整填写 PR 模板
- 关联相关 Issue：`Closes #42`
- 标题使用 Conventional Commits 格式
- 欢迎提交 Draft PR 以获取早期反馈

### 代码审查

- 合并前至少需要一位维护者批准
- 处理所有审查意见 — 在讨论中解释分歧
- 通过 rebase 保持与 `main` 同步
- 维护者将使用 squash 合并

---

## 报告 Bug

使用 **Bug Report** 模板提交 Issue。请包含：

1. 您期望的行为
2. 实际发生的情况 — 完整的错误消息或 panic 输出
3. 最小复现 — 展示该 Bug 的最小代码
4. 环境信息：Rust 版本、操作系统、GPU/驱动、scenekit 版本、启用的 feature

---

## 建议功能

使用 **Feature Request** 模板提交 Issue。请包含：

1. 使用场景 — 您要解决什么问题？
2. 建议的 API — 展示代码
3. 考虑过的替代方案 — 为什么现有 API 无法解决？

---

## 开发 GPU Crate

GPU crate（`scenekit-renderer`、`scenekit-post`、`scenekit-loader`）有额外的要求：

### 无头测试

GPU 测试需要支持 Vulkan 的设备或软件渲染器：

```sh
# Linux 使用 lavapipe 无头运行（Mesa 软件 Vulkan）：
WGPU_BACKEND=vulkan cargo test -p scenekit-renderer

# macOS — 原生 Metal 可用：
cargo test -p scenekit-renderer

# Windows — DX12 或 Vulkan 可用：
cargo test -p scenekit-renderer
```

### WGSL 着色器

所有着色器位于其 crate 的 `src/shaders/` 目录下。规则：
- 每个渲染通道一个 `.wgsl` 文件
- 所有常量定义在文件顶部
- 着色器结构体必须与 Rust `bytemuck::Pod` 对应物逐字节匹配
- 测试 Rust 结构体大小等于 WGSL 结构体大小

### 性能变更

如果您的更改影响渲染性能，请包含基准测试对比：

```sh
cargo bench -p scenekit-renderer -- --save-baseline before
# 进行您的更改
cargo bench -p scenekit-renderer -- --baseline before
```

---

## Crate 版本管理

scenekit 遵循 [Semantic Versioning](https://semver.org/)。

- **补丁**（`0.1.x`）— Bug 修复，无 API 变更
- **次版本**（`0.x.0`）— 新功能，向后兼容
- **主版本**（`x.0.0`）— 破坏性变更（不在 `v1.0.0` 之前发布）

每个子 crate 独立管理版本。门面 crate 跟踪最高的子 crate 版本。

### 发布顺序

```
scenekit-math → scenekit-core → scenekit-input → scenekit
```

---

## 有问题？

提交 Issue 或加入 [launcher-rs Discord](https://discord.gg/aarambhdevhub) — 查找 `#scenekit` 频道。
