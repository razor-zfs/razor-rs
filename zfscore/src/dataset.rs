use std::borrow::Cow;
use std::ffi;

use razor_nvpair as nvpair;

use nvpair::NvListAccess;

use crate::libzfs;
use crate::lzc;
pub use collector::DatasetCollectorBuilder;
pub use zfs_dataset_handle::ZfsDatasetHandle;

use super::error::CoreError;
use super::Result;

mod collector;
mod zfs_dataset_handle;

impl ZfsDatasetHandle {
    pub fn new(name: ffi::CString) -> Result<Self> {
        let handle = unsafe { libzfs::zfs_open(name.as_ptr()) };

        if handle.is_null() {
            return Err(CoreError::DatasetNotExist);
        }

        Ok(Self { handle })
    }

    pub fn name(&self) -> Cow<'_, str> {
        unsafe {
            let cstr = libzfs::zfs_get_name(self.handle);
            ffi::CStr::from_ptr(cstr).to_string_lossy()
        }
    }

    pub fn r#type(&self) -> lzc::zfs_type_t {
        unsafe { libzfs::zfs_get_type(self.handle) }
    }

    pub fn is_volume(&self) -> bool {
        self.r#type() == libzfs::zfs_type_t::ZFS_TYPE_VOLUME
    }

    pub fn is_filesystem(&self) -> bool {
        self.r#type() == libzfs::zfs_type_t::ZFS_TYPE_FILESYSTEM
    }

    pub fn numeric_property_old(&self, name: &str, property: lzc::zfs_prop_t) -> u64 {
        let nvl = unsafe { libzfs::zfs_get_all_props(self.handle) };
        let nvl = nvpair::NvListRef::from_raw(nvl, self);

        if let Ok(nvp) = nvl.lookup_nvpair(name) {
            nvp.uint64()
        } else {
            lzc::zfs_prop_default_numeric(property)
        }
    }

    pub fn numeric_property(&self, property: lzc::zfs_prop_t) -> u64 {
        unsafe { libzfs::zfs_prop_get_int(self.handle, property) }
    }

    pub fn string_property(&self, name: &str, property: lzc::zfs_prop_t) -> String {
        let nvl = unsafe { libzfs::zfs_get_all_props(self.handle) };
        let nvl = nvpair::NvListRef::from_raw(nvl, self);

        if let Ok(nvp) = nvl.lookup_nvpair(name) {
            nvp.string().to_string()
        } else {
            lzc::zfs_prop_default_string(property).to_string()
        }
    }

    pub fn set_properties(&self, nvl: nvpair::NvList) -> libc::c_int {
        let nvl = nvl.nvl();
        unsafe { libzfs::zfs_prop_set_list(self.handle, nvl) }
    }
}

impl From<*mut razor_zfscore_sys::zfs_handle_t> for ZfsDatasetHandle {
    fn from(handle: *mut razor_zfscore_sys::zfs_handle_t) -> Self {
        Self { handle }
    }
}

impl Drop for ZfsDatasetHandle {
    fn drop(&mut self) {
        unsafe { libzfs::zfs_close(self.handle) };
    }
}
