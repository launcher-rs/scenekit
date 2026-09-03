# 术语表

## SceneGraph

节点、变换、可见性、图层和对象 ID 的 CPU 层次结构。

## Geometry

CPU 顶点和索引数据。在注册到渲染器之前，它不是 GPU 缓冲区。

## MaterialId

场景节点和渲染器材质存储使用的稳定 ID。

## BVH

`scenekit-raycaster` 使用的包围体层次结构，用于减少候选网格测试。

## LineGeometry

`scenekit-helpers` 生成的调试线数据，用于网格、轴、边界、相机、灯光、箭头和骨骼。

## PostStack

一系列 GPU 全屏后处理效果。

## Facade

`scenekit` crate，在稳定的功能标志后面重新导出专注的 crate。
