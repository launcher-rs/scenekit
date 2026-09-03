# 资产管道

将生成的 glTF 文件加载到 `AssetPackage` 中，并通过 `RendererAssetExt` 上传。

`examples/asset_pipeline.rs`

必需功能：

`loader`、`renderer`

```sh
cargo run -p scenekit --example asset_pipeline --features "loader renderer"
```