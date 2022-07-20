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
#![allow(clippy::missing_safety_doc)]
#![deny(warnings)]

use std::ffi;
use std::mem;
use std::ptr;

use once_cell::sync::Lazy;
use razor_libnvpair as libnvpair;
use razor_libzfs_sys as sys;

pub use sys::zfs_canmount_type_t;
pub use sys::zfs_error;
pub use sys::zfs_error_t;
pub use sys::zfs_handle_t;
pub use sys::zfs_prop_t;
pub use sys::zfs_type_t;
pub use sys::zfs_userquota_prop_t;
pub use sys::zpool_handle_t;
pub use sys::zpool_prop_t;

pub use version::Version;

use handle::LIBZFS_HANDLE;

mod handle;
mod version;

pub unsafe fn libzfs_errno() -> libc::c_int {
    sys::libzfs_errno(LIBZFS_HANDLE.handle())
}

pub unsafe fn libzfs_error_action() -> *const libc::c_char {
    sys::libzfs_error_action(LIBZFS_HANDLE.handle())
}

pub unsafe fn libzfs_error_description() -> *const libc::c_char {
    sys::libzfs_error_description(LIBZFS_HANDLE.handle())
}

pub unsafe fn zfs_open(name: *const libc::c_char) -> *mut sys::zfs_handle_t {
    let types = zfs_type_t::ZFS_TYPE_FILESYSTEM
        | zfs_type_t::ZFS_TYPE_VOLUME
        | zfs_type_t::ZFS_TYPE_SNAPSHOT
        | zfs_type_t::ZFS_TYPE_POOL
        | zfs_type_t::ZFS_TYPE_BOOKMARK;
    let types = types.0 as i32;
    sys::zfs_open(LIBZFS_HANDLE.handle(), name, types)
}

pub unsafe fn zfs_close(handle: *mut sys::zfs_handle_t) {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_close(handle);
}

pub unsafe fn zfs_get_name(handle: *mut sys::zfs_handle_t) -> *const libc::c_char {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_get_name(handle)
}

pub unsafe fn zfs_get_type(handle: *mut sys::zfs_handle_t) -> zfs_type_t {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_get_type(handle)
}

pub unsafe fn zfs_get_all_props(handle: *mut sys::zfs_handle_t) -> *mut libnvpair::nvlist_t {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_get_all_props(handle)
}

pub unsafe fn zfs_prop_get_numeric(
    handle: *mut sys::zfs_handle_t,
    property: sys::zfs_prop_t,
) -> Result<u64, i32> {
    Lazy::force(&LIBZFS_HANDLE);
    let mut value = 0;
    let mut src = mem::MaybeUninit::uninit();
    let statbuf = ptr::null_mut();
    let statlen = 0;
    let rc = sys::zfs_prop_get_numeric(
        handle,
        property,
        &mut value,
        src.as_mut_ptr(),
        statbuf,
        statlen,
    );
    if rc == 0 {
        Ok(value)
    } else {
        Err(rc)
    }
}

pub unsafe fn zfs_prop_get_int(handle: *mut sys::zfs_handle_t, property: sys::zfs_prop_t) -> u64 {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_prop_get_int(handle, property)
}

pub unsafe fn zfs_prop_set_list(
    dataset_handle: *mut sys::zfs_handle_t,
    nvl: *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_prop_set_list(dataset_handle, nvl)
}

pub unsafe fn zfs_prop_to_name(property: sys::zfs_prop_t) -> *const libc::c_char {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_prop_to_name(property)
}

pub unsafe fn zfs_prop_default_string(property: sys::zfs_prop_t) -> *const libc::c_char {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_prop_default_string(property)
}

pub unsafe fn zfs_prop_default_numeric(property: sys::zfs_prop_t) -> u64 {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_prop_default_numeric(property)
}

pub unsafe fn zfs_refresh_properties(dataset_handle: *mut sys::zfs_handle_t) {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_refresh_properties(dataset_handle)
}

pub unsafe fn zfs_valid_proplist(
    r#type: zfs_type_t,
    nvl: *mut libnvpair::nvlist_t,
    zoned: bool,
    dataset_handle: *mut sys::zfs_handle_t,
    zpool_handle: *mut zpool_handle_t,
    key_params_ok: bool,
    err_buf: *const libc::c_char,
) -> *mut libnvpair::nvlist_t {
    let zoned = zoned.into();
    let key_params_ok = key_params_ok.into();
    sys::zfs_valid_proplist(
        LIBZFS_HANDLE.handle(),
        r#type,
        nvl,
        zoned,
        dataset_handle,
        zpool_handle,
        key_params_ok,
        err_buf,
    )
}

pub unsafe fn zfs_get_pool_handle(dataset_handle: *mut sys::zfs_handle_t) -> *mut zpool_handle_t {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_get_pool_handle(dataset_handle)
}

pub unsafe fn zvol_volsize_to_reservation(
    zpool_handle: *mut zpool_handle_t,
    volsize: u64,
    props: *mut libnvpair::nvlist_t,
) -> u64 {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zvol_volsize_to_reservation(zpool_handle, volsize, props)
}

pub unsafe fn zfs_iter_root(callback: sys::zfs_iter_f, ptr: *mut libc::c_void) {
    sys::zfs_iter_root(LIBZFS_HANDLE.handle(), callback, ptr);
}

pub unsafe fn zfs_iter_filesystems(
    handle: *mut sys::zfs_handle_t,
    callback: sys::zfs_iter_f,
    ptr: *mut libc::c_void,
) {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_iter_filesystems(handle, callback, ptr);
}

pub unsafe fn zfs_iter_snapshots(
    handle: *mut sys::zfs_handle_t,
    simple: bool,
    callback: sys::zfs_iter_f,
    data: *mut libc::c_void,
    min_txg: u64,
    max_txg: u64,
) {
    Lazy::force(&LIBZFS_HANDLE);
    let simple = simple.into();
    sys::zfs_iter_snapshots(handle, simple, callback, data, min_txg, max_txg);
}

pub fn zfs_version() -> Version {
    LIBZFS_HANDLE.version().clone()
}
