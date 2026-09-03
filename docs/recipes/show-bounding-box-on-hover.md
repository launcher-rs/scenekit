# 悬停时显示边界框

## 使用场景

当你需要此行为在应用程序中并想要支持它的最小 scenekit 子系统集时。

## 方法

使用光线投射器选择来选择节点，然后为该节点边界生成 `BoundingBoxHelper` 线几何体。

## 示例

```rust
use scenekit::{Aabb, BoundingBoxHelper, Vec3};
let bounds = Aabb::new(Vec3::splat(-1.0), Vec3::splat(1.0));
let lines = BoundingBoxHelper::new(bounds).geometry();
# let _ = lines;
```

## 验证

围绕上述状态更改或命令添加专注测试。对于浏览器或 GPU 路径，保持测试门控以便正常 CPU CI 保持快速。
