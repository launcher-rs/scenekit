# 控件展示

源码：`examples/controls_showcase.rs`

此示例构建一个 DPR 感知的 `InputState`，使用指针和滚动输入驱动轨道球，推进帧边界，然后使用键盘和游戏手柄状态驱动第一人称移动。

```sh
cargo run -p scenekit --example controls_showcase
```

相同的输入快照可以驱动轨道、飞行、轨道球、轨迹球、地图、第一人称和指针锁控件。