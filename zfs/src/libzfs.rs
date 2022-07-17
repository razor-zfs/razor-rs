use std::borrow::Cow;
use std::ffi;

use razor_libzfs as libzfs;

pub use libzfs::zfs_prop_t;

use super::*;

pub use self::collector::DatasetCollectorBuilder;
pub use self::error::ZfsError;

mod collector;
mod error;

#[derive(Debug)]
pub struct ZfsHandle {
    handle: *mut libzfs::zfs_handle_t,
}

unsafe impl ::std::marker::Send for ZfsHandle {}

impl ZfsHandle {
    pub fn new(name: ffi::CString) -> Result<Self> {
        let handle = unsafe { libzfs::zfs_open(name.as_ptr()) };

        if !handle.is_null() {
            Ok(Self { handle })
        } else {
            Err(ZfsError::from_libzfs_errno())?
        }
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

    pub fn set_properties(&mut self, nvl: nvpair::NvList) -> Result<()> {
        let rc = unsafe { libzfs::zfs_prop_set_list(self.handle, *nvl) };
        value_or_err((), rc)?;
        unsafe { libzfs::zfs_refresh_properties(self.handle) };
        Ok(())
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

pub fn libzfs_error_description() -> Cow<'static, str> {
    unsafe {
        let cstr = libzfs::libzfs_error_description();
        ffi::CStr::from_ptr(cstr).to_string_lossy()
    }
}

pub fn libzfs_errno() -> i32 {
    unsafe { libzfs::libzfs_errno() }
}
