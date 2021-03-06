#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

//! High level safe wrappers on top of libzfs
//!

use std::borrow::Cow;
use std::ffi;

use razor_libzfs as libzfs;
use razor_nvpair as nvpair;

pub use libzfs::zfs_canmount_type_t;
pub use libzfs::zfs_prop_t;

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
    pub fn new(name: ffi::CString) -> Result<Self, ZfsError> {
        let handle = unsafe { libzfs::zfs_open(name.as_ptr()) };

        if !handle.is_null() {
            Ok(Self { handle })
        } else {
            Err(ZfsError::from_libzfs_errno())
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

    pub fn zpool_handle(&self) -> *mut libzfs::zpool_handle_t {
        unsafe { libzfs::zfs_get_pool_handle(self.handle) }
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

    pub fn set_properties(&mut self, nvl: impl Into<nvpair::NvList>) -> Result<(), ZfsError> {
        let nvl = nvl.into();
        let rc = unsafe { libzfs::zfs_prop_set_list(self.handle, *nvl) };
        ZfsError::from_rc(rc)
            .result(())
            .map(|_| unsafe { libzfs::zfs_refresh_properties(self.handle) })
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

pub fn zfs_list() -> DatasetCollectorBuilder {
    DatasetCollectorBuilder::new()
}

pub fn zfs_list_from(name: impl AsRef<str>) -> DatasetCollectorBuilder {
    DatasetCollectorBuilder::from(name)
}

pub fn create_filesystem(
    name: impl AsRef<str>,
    props: impl Into<nvpair::NvList>,
) -> Result<(), ZfsError> {
    let name = cstring(name)?;
    let props = props.into();
    let rc = unsafe {
        libzfs::zfs_create(
            name.as_ptr(),
            libzfs::zfs_type_t::ZFS_TYPE_FILESYSTEM,
            *props,
        )
    };
    ZfsError::from_rc(rc).result(())
}

#[inline]
fn cstring(text: impl AsRef<str>) -> Result<ffi::CString, ffi::NulError> {
    ffi::CString::new(text.as_ref())
}
