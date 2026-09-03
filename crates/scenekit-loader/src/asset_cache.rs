use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use scenekit_core::{LoadError, ScenixError};

/// 按路径键控的资产缓存，复用已解码的 CPU 端资产。
#[derive(Debug)]
pub struct AssetCache<T> {
    assets: BTreeMap<PathBuf, Arc<T>>,
}

impl<T> AssetCache<T> {
    /// 创建空缓存。
    #[inline]
    pub const fn new() -> Self {
        Self {
            assets: BTreeMap::new(),
        }
    }

    /// 返回已缓存资产的数量。
    #[inline]
    pub fn len(&self) -> usize {
        self.assets.len()
    }

    /// 返回缓存是否为空。
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.assets.is_empty()
    }

    /// 经过规范路径规范化后，返回 `path` 是否已缓存。
    pub fn contains(&self, path: impl AsRef<Path>) -> bool {
        canonical_cache_key(path.as_ref())
            .ok()
            .is_some_and(|key| self.assets.contains_key(&key))
    }

    /// 加载资产一次，后续请求返回共享句柄。
    pub fn get_or_load(
        &mut self,
        path: impl AsRef<Path>,
        load: impl FnOnce(&Path) -> Result<T, ScenixError>,
    ) -> Result<Arc<T>, ScenixError> {
        let key = canonical_cache_key(path.as_ref())?;
        if let Some(asset) = self.assets.get(&key) {
            return Ok(Arc::clone(asset));
        }

        let asset = Arc::new(load(&key)?);
        self.assets.insert(key, Arc::clone(&asset));
        Ok(asset)
    }

    /// 移除已缓存的资产（如果存在）。
    pub fn invalidate(&mut self, path: impl AsRef<Path>) -> bool {
        canonical_cache_key(path.as_ref())
            .ok()
            .and_then(|key| self.assets.remove(&key))
            .is_some()
    }

    /// 清空所有缓存句柄。
    #[inline]
    pub fn clear(&mut self) {
        self.assets.clear();
    }
}

impl<T> Default for AssetCache<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

fn canonical_cache_key(path: &Path) -> Result<PathBuf, ScenixError> {
    path.canonicalize().map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            ScenixError::Load(LoadError::NotFound)
        } else {
            ScenixError::Load(LoadError::Io)
        }
    })
}
