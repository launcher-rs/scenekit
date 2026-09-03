# Crate 依赖关系图

scenekit 保持依赖方向分层和显式。

```text
scenekit facade
  -> CPU authoring crates: math, core, input, scene, camera, mesh, material, light, texture, raycaster, helpers
  -> optional loader
  -> optional renderer
  -> optional post
  -> optional animato
  -> optional wasm
```

## 实际规则

- CPU crate 不应依赖 GPU crate。
- `scenekit-post` 保持独立于 `scenekit-renderer`；渲染器可选地集成后处理支持。
- `scenekit-loader` 输出 CPU 数据，不上传到 GPU。
- `scenekit-wasm` 面向浏览器且可选。
