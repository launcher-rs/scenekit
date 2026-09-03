# 动画运行时

## 目的

通过 `AnimationMixer` 播放、循环和交叉淡入淡出基于剪辑的动画，这是过程式 `scenekitAnimationDriver` 的基于剪辑的对应物。

## 源码

`examples/animation_runtime.rs`

## 相关功能标志

`animato`、`scene`

## 运行或检查

```sh
cargo run -p scenekit --example animation_runtime --features animato,scene
```

## 查看内容

- 剪辑播放沿关键帧位置推进节点平移。
- 循环模式在每次迭代中无缝重复剪辑。
- 第二个剪辑可以通过添加另一个操作并调用两个操作上的 `fade_to` 来交叉淡入淡出。

## 相关文档

- [示例索引](README.md)
- [功能标志](../concepts/feature-flags.md)
