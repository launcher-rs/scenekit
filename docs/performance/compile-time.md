# 编译时间

## 目标

保持默认功能为仅 CPU，避免在不需要它们的 crate 中启用 loader/renderer/post/wasm，并为库优先使用专注的 crate。

## 先测量

使用专注的命令，一次比较一个更改。当只有一个二进制文件或示例需要重型功能时，避免全局启用它们。

## 命令或模式

```sh
cargo check -p scenekit --no-default-features
```

## 实际检查

- 保持仅 CPU crate 轻量级。
- 除非输入发生变化，否则避免每帧重建数据结构。
- 分析时分离加载、注册、更新和渲染成本。
