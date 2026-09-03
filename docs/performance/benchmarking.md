# 基准测试

## 目标

在 CI 中编译基准测试，仅在显式启用时运行昂贵的 GPU 基准测试。

## 先测量

使用专注的命令，一次比较一个更改。当只有一个二进制文件或示例需要重型功能时，避免全局启用它们。

## 命令或模式

```sh
cargo bench --workspace --no-run
SCENIX_RUN_GPU_BENCHES=1 cargo bench -p scenekit-post
```

## 实际检查

- 保持仅 CPU crate 轻量级。
- 除非输入发生变化，否则避免每帧重建数据结构。
- 分析时分离加载、注册、更新和渲染成本。