use std::mem;
use std::ptr;

use once_cell::sync::Lazy;
use razor_zfscore_sys as sys;

pub(crate) use sys::zfs_type_t;

static LIBZFS_HANDLE: Lazy<LibZfsHandle> = Lazy::new(LibZfsHandle::init);

#[derive(Debug)]
struct LibZfsHandle {
    libzfs_handle: *mut sys::libzfs_handle_t,
}

unsafe impl Send for LibZfsHandle {}
unsafe impl Sync for LibZfsHandle {}

impl LibZfsHandle {
    fn init() -> Self {
        Self {
            libzfs_handle: unsafe { Self::init_impl() },
        }
    }

    unsafe fn init_impl() -> *mut sys::libzfs_handle_t {
        let handle = sys::libzfs_init();
        if handle.is_null() {
            panic!("libzfs_init failed");
        }
        sys::libzfs_print_on_error(handle, sys::boolean_t::B_FALSE);
        handle
    }
}

pub(crate) unsafe fn zfs_open(name: *const libc::c_char) -> *mut sys::zfs_handle_t {
    let types = sys::zfs_type_t::ZFS_TYPE_FILESYSTEM
        | sys::zfs_type_t::ZFS_TYPE_VOLUME
        | sys::zfs_type_t::ZFS_TYPE_SNAPSHOT;
    let types = types.0 as i32;
    sys::zfs_open(LIBZFS_HANDLE.libzfs_handle, name, types)
}

pub(crate) unsafe fn zfs_close(handle: *mut sys::zfs_handle_t) {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_close(handle);
}

pub(crate) unsafe fn zfs_get_name(handle: *mut sys::zfs_handle_t) -> *const libc::c_char {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_get_name(handle)
}

pub(crate) unsafe fn zfs_get_type(handle: *mut sys::zfs_handle_t) -> sys::zfs_type_t {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_get_type(handle)
}

pub(crate) unsafe fn zfs_get_all_props(handle: *mut sys::zfs_handle_t) -> *mut sys::nvlist_t {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_get_all_props(handle)
}

pub(crate) unsafe fn _zfs_prop_get_numeric(
    handle: *mut sys::zfs_handle_t,
    property: sys::zfs_prop_t,
) -> u64 {
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
        value
    } else {
        panic!("zfs_prop_get_numeric failed");
    }
}

pub(crate) unsafe fn zfs_prop_get_int(
    handle: *mut sys::zfs_handle_t,
    property: sys::zfs_prop_t,
) -> u64 {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_prop_get_int(handle, property)
}

pub(crate) unsafe fn zfs_prop_set_list(
    dataset_handle: *mut sys::zfs_handle_t,
    nvl: *mut sys::nvlist_t,
) -> libc::c_int {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_prop_set_list(dataset_handle, nvl)
}

pub(crate) unsafe fn zfs_prop_to_name(property: sys::zfs_prop_t) -> *const libc::c_char {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_prop_to_name(property)
}

pub(crate) unsafe fn zfs_prop_default_string(property: sys::zfs_prop_t) -> *const libc::c_char {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_prop_default_string(property)
}

pub(crate) unsafe fn zfs_prop_default_numeric(property: sys::zfs_prop_t) -> u64 {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_prop_default_numeric(property)
}

pub(crate) unsafe fn zfs_iter_root(
    f: unsafe extern "C" fn(*mut sys::zfs_handle_t, *mut libc::c_void) -> libc::c_int,
    ptr: *mut libc::c_void,
) {
    sys::zfs_iter_root(LIBZFS_HANDLE.libzfs_handle, Some(f), ptr);
}

pub(crate) unsafe fn zfs_iter_filesystems(
    handle: *mut sys::zfs_handle_t,
    f: unsafe extern "C" fn(*mut sys::zfs_handle_t, *mut libc::c_void) -> libc::c_int,
    ptr: *mut libc::c_void,
) {
    sys::zfs_iter_filesystems(handle, Some(f), ptr);
}
