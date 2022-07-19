#![allow(non_camel_case_types)]
#![allow(deref_nullptr)]

use razor_libnvpair::*;

include!(concat!(env!("OUT_DIR"), "/zfs.rs"));

impl zfs_type_t {
    /// Returns true if the type is a filesystem.
    ///
    pub fn is_filesystem(&self) -> bool {
        *self & zfs_type_t::ZFS_TYPE_FILESYSTEM != zfs_type_t(0)
    }

    /// Returns true if the type is a snapshot.
    ///
    pub fn is_snapshot(&self) -> bool {
        *self & zfs_type_t::ZFS_TYPE_SNAPSHOT != zfs_type_t(0)
    }

    /// Returns true if the type is a volume.
    ///
    pub fn is_volume(&self) -> bool {
        *self & zfs_type_t::ZFS_TYPE_VOLUME != zfs_type_t(0)
    }

    /// Returns true if the type is a bookmark.
    ///
    pub fn is_bookmark(&self) -> bool {
        *self & zfs_type_t::ZFS_TYPE_BOOKMARK != zfs_type_t(0)
    }

    /// Returns true if the type is a pool.
    ///
    pub fn is_pool(&self) -> bool {
        *self & zfs_type_t::ZFS_TYPE_POOL != zfs_type_t(0)
    }

    pub fn contains(&self, other: zfs_type_t) -> bool {
        *self & other != zfs_type_t(0)
    }
}

impl From<zfs_prop_t> for ::std::borrow::Cow<'static, str> {
    fn from(property: zfs_prop_t) -> Self {
        unsafe {
            let cstr = zfs_prop_to_name(property);
            ::std::ffi::CStr::from_ptr(cstr).to_string_lossy()
        }
    }
}
