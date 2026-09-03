# 创建 3D 作品集部分

## 使用场景

当你需要此行为在应用程序中并想要支持它的最小 scenekit 子系统集时。

## 方法

使用 WASM 包装器或网站模式创建具有干净回退 UI 的生成浏览器场景。

## 示例

```sh
cd website
trunk build --release --public-url /scenekit/
```

## 验证

围绕上述状态更改或命令添加专注测试。对于浏览器或 GPU 路径，保持测试门控以便正常 CPU CI 保持快速。
