use std::borrow::Cow;
use std::ffi;

use razor_libzfs as libzfs;

pub use libzfs::zfs_prop_t;
pub use libzfs::ZfsError;

use super::*;

pub use self::collector::DatasetCollectorBuilder;

mod collector;

#[derive(Debug)]
pub struct ZfsHandle {
    handle: *mut libzfs::zfs_handle_t,
}

unsafe impl ::std::marker::Send for ZfsHandle {}

impl ZfsHandle {
    pub fn new(name: ffi::CString) -> Result<Self> {
        let handle = unsafe { libzfs::zfs_open(name.as_ptr()) };

        if handle.is_null() {
            let error = unsafe { libzfs::libzfs_errno() };
            Err(ZfsError::from(libzfs::translate_zfs_error(error)))?;
        }

        Ok(Self { handle })
    }

    pub fn name(&self) -> Cow<'_, str> {
        unsafe {
            let cstr = libzfs::zfs_get_name(self.handle);
            ffi::CStr::from_ptr(cstr).to_string_lossy()
        }
    }

    pub fn r#type(&self) -> libzfs::zfs_type_t {
        unsafe { libzfs::zfs_get_type(self.handle) }
    }

    pub fn is_volume(&self) -> bool {
        self.r#type() == libzfs::zfs_type_t::ZFS_TYPE_VOLUME
    }

    pub fn is_filesystem(&self) -> bool {
        self.r#type() == libzfs::zfs_type_t::ZFS_TYPE_FILESYSTEM
    }

    pub fn numeric_property_old(&self, name: &str, property: zfs_prop_t) -> u64 {
        let nvl = unsafe { libzfs::zfs_get_all_props(self.handle) };
        let nvl = nvpair::NvListRef::from_raw(nvl, self);

        if let Ok(Some(nvp)) = nvl.lookup_nvpair(name) {
            nvp.uint64()
        } else {
            zfs_prop_default_numeric(property)
        }
    }

    pub fn numeric_property(&self, property: zfs_prop_t) -> u64 {
        unsafe { libzfs::zfs_prop_get_int(self.handle, property) }
    }

    pub fn string_property(&self, name: &str, property: zfs_prop_t) -> String {
        let nvl = unsafe { libzfs::zfs_get_all_props(self.handle) };
        let nvl = nvpair::NvListRef::from_raw(nvl, self);

        if let Ok(Some(nvp)) = nvl.lookup_nvpair(name) {
            nvp.string().to_string()
        } else {
            zfs_prop_default_string(property).to_string()
        }
    }

    pub fn set_properties(&self, nvl: nvpair::NvList) -> Result<()> {
        value_or_err((), unsafe { libzfs::zfs_prop_set_list(self.handle, *nvl) })
    }
}

impl From<*mut libzfs::zfs_handle_t> for ZfsHandle {
    fn from(handle: *mut libzfs::zfs_handle_t) -> Self {
        Self { handle }
    }
}

impl Drop for ZfsHandle {
    fn drop(&mut self) {
        unsafe { libzfs::zfs_close(self.handle) };
    }
}

pub fn zfs_prop_default_string(property: zfs_prop_t) -> Cow<'static, str> {
    unsafe {
        let cstr = libzfs::zfs_prop_default_string(property);
        ffi::CStr::from_ptr(cstr).to_string_lossy()
    }
}

pub fn zfs_prop_default_numeric(property: zfs_prop_t) -> u64 {
    unsafe { libzfs::zfs_prop_default_numeric(property) }
}

pub fn zfs_prop_to_name(property: zfs_prop_t) -> Cow<'static, str> {
    unsafe {
        let cstr = libzfs::zfs_prop_to_name(property);
        ffi::CStr::from_ptr(cstr).to_string_lossy()
    }
}
