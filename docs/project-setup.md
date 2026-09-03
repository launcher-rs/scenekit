# 项目设置

当你开始新的 Scenix 应用程序或示例时，请使用此页面。推荐的布局将 CPU 场景数据与 GPU 注册分离，并使可选系统显而易见。

## 推荐布局

```text
my-app/
  Cargo.toml
  src/
    main.rs
    scene.rs       # SceneGraph、变换、ID
    assets.rs      # 几何体/材质/纹理存储
    render.rs      # 渲染器设置和 GPU 注册
    input.rs       # 指针/键盘状态和相机控件
```

## Cargo 功能

```toml
[dependencies]
scenekit = { version = "1", features = ["renderer", "post"] }
```

仅在运行时解码资产时使用 `loader`。仅在应用需要动画轨道时使用 `animato`。

## 开发命令

```sh
cargo fmt --check
cargo clippy --all-features -- -D warnings
cargo test --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
```

## 运行时模式

在 CPU 上创建场景资源，变换编辑后调用 `scene.update_world_transforms()`，将更改的资源注册到渲染器，然后使用活动相机进行渲染。这使场景创作具有确定性，GPU 所有权显式。