use scenekit_core::ValidationError;

use crate::Texture2D;

/// 逐帧更新的可变 CPU 端纹理。
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VideoTexture {
    /// 当前纹理帧。
    pub texture: Texture2D,
    /// 已成功上传的帧数。
    pub frame_index: u64,
    /// 自上次上传以来纹理是否已更改。
    pub dirty: bool,
}

impl VideoTexture {
    /// 从初始帧创建视频纹理。
    #[inline]
    pub fn new(texture: Texture2D) -> Self {
        Self {
            texture,
            frame_index: 0,
            dirty: true,
        }
    }

    /// 替换当前帧并将纹理标记为脏。
    pub fn update_frame(&mut self, data: &[u8]) -> Result<(), ValidationError> {
        if data.len() != self.texture.base_level_len()? {
            return Err(ValidationError::OutOfRange);
        }
        self.texture.data.clear();
        self.texture.data.extend_from_slice(data);
        self.frame_index = self.frame_index.saturating_add(1);
        self.dirty = true;
        Ok(())
    }

    /// 清除脏标记。
    #[inline]
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}
