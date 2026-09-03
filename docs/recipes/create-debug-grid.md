# 创建调试网格

## 使用场景

当你需要此行为在应用程序中并想要支持它的最小 Scenix 子系统集时。

## 方法

生成 `LineGeometry` 网格数据，并使用支持线的调试渲染器绘制它。

## 示例

```rust
use scenekit::GridHelper;
let grid = GridHelper::new(20, 0.5).geometry();
# let _ = grid;
```

## 验证

围绕上述状态更改或命令添加专注测试。对于浏览器或 GPU 路径，保持测试门控以便正常 CPU CI 保持快速。