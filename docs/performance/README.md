# 性能

当应用程序足够大以至于编译时间、资产加载、场景遍历、光线投射、渲染或浏览器有效负载大小很重要时，请使用这些页面。

对于 v1.3 资产密集型应用程序，优先使用 `AssetManager`，以便重复加载共享缓存的 `AssetPackage` 句柄，导入许多大文件时设置内存预算，并在将资源上传到渲染器之前检查包诊断。

- [编译时间](compile-time.md)
- [Crate 大小](crate-size.md)
- [BVH 光线投射](bvh-raycasting.md)
- [场景图优化](scene-graph-optimization.md)
- [渲染器性能](renderer-performance.md)
- [WASM 性能](wasm-performance.md)
- [基准测试](benchmarking.md)