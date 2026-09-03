# 从 0.9 迁移到 1.0

Scenix 1.0.0 稳定了 0.x 里程碑期间引入的模块化 API。大多数 0.9 代码在版本升级后应继续编译，因为 v1 优先选择累加 API 和弃用而不是静默移除。

## Cargo 更改

```toml
[dependencies]
scenekit = "1"
```

保持可选功能显式：

```toml
scenekit = { version = "1", features = ["renderer", "post"] }
```

## 审查清单

- 将 `0.9` 依赖要求替换为 `1`。
- 确认渲染器、加载器、后处理、Animato 和 WASM 功能仅在需要时启用。
- 如果你支持 `no_std`，请为 CPU crate 重新运行无默认检查。
- 在 `../release-v1.2.0.md` 中查看当前渲染器行为和限制。

## 验证

```sh
cargo test --workspace --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
```