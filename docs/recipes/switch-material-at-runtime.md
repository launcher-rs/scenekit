# 运行时切换材质

## 使用场景

当你需要此行为在应用程序中并想要支持它的最小 Scenix 子系统集时。

## 方法

更改节点材质 ID 或更新同一 `MaterialId` 的渲染器材质注册。

## 示例

```rust
use scenekit::{MaterialId, PbrMaterial};
let material_id = MaterialId::new(1);
let material = PbrMaterial::new();
# let _ = (material_id, material);
```

## 验证

围绕上述状态更改或命令添加专注测试。对于浏览器或 GPU 路径，保持测试门控以便正常 CPU CI 保持快速。