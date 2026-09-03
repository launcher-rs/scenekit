# 运行无头渲染测试

## 使用场景

当你需要此行为在应用程序中并想要支持它的最小 scenekit 子系统集时。

## 方法

使用环境变量门控 GPU 测试，以便 CI 可以选择 lavapipe 或跳过 GPU 工作。

## 示例

```sh
scenekit_RUN_GPU_TESTS=1 WGPU_BACKEND=vulkan cargo test -p scenekit-renderer --all-features
```

## 验证

围绕上述状态更改或命令添加专注测试。对于浏览器或 GPU 路径，保持测试门控以便正常 CPU CI 保持快速。
