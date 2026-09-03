use core::fmt;

macro_rules! id_type {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name(u64);

        impl $name {
            /// 从原始值创建 ID。
            #[inline]
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            /// 返回原始 ID 值。
            #[inline]
            pub const fn get(self) -> u64 {
                self.0
            }

            /// 返回此 ID 是否为默认的零哨兵值。
            #[inline]
            pub const fn is_null(self) -> bool {
                self.0 == 0
            }
        }

        impl fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }

        impl From<u64> for $name {
            #[inline]
            fn from(value: u64) -> Self {
                Self::new(value)
            }
        }

        impl From<$name> for u64 {
            #[inline]
            fn from(value: $name) -> Self {
                value.get()
            }
        }
    };
}

id_type!(NodeId, "场景节点的类型化标识符。");
id_type!(MeshId, "网格资源的类型化标识符。");
id_type!(MaterialId, "材质资源的类型化标识符。");
id_type!(TextureId, "纹理资源的类型化标识符。");
id_type!(LightId, "光源资源的类型化标识符。");
id_type!(CameraId, "摄像机资源的类型化标识符。");
id_type!(AssetId, "导入资产包的类型化标识符。");
id_type!(SkinId, "导入蒙皮元数据的类型化标识符。");
id_type!(
    AnimationClipId,
    "导入动画片段的类型化标识符。"
);

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    #[test]
    fn id_newtypes_are_copy_hashable_and_debuggable() {
        let id = NodeId::new(42);
        let copied = id;
        assert_eq!(copied.get(), 42);
        assert_eq!(format!("{id:?}"), "NodeId(42)");
    }
}
