use std::mem;
use std::ptr;

use once_cell::sync::Lazy;
use razor_zfscore_sys as sys;

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

    // unsafe fn make_dataset_handle(&self, name: *const libc::c_char) -> *mut sys::zfs_handle_t {
    //     sys::make_dataset_handle(self.libzfs_handle, name)
    // }

    unsafe fn zfs_open(&self, name: *const libc::c_char) -> *mut sys::zfs_handle_t {
        let types = sys::zfs_type_t::ZFS_TYPE_FILESYSTEM
            | sys::zfs_type_t::ZFS_TYPE_VOLUME
            | sys::zfs_type_t::ZFS_TYPE_SNAPSHOT;
        let types = types.0 as i32;
        sys::zfs_open(self.libzfs_handle, name, types)
    }
}

// pub(crate) unsafe fn make_dataset_handle(name: *const libc::c_char) -> *mut sys::zfs_handle_t {
//     LIBZFS_HANDLE.make_dataset_handle(name)
// }

pub(crate) unsafe fn libzfs_mnttab_find(name: *const libc::c_char) -> Option<sys::mnttab> {
    let mut entry = mem::MaybeUninit::uninit();
    if sys::libzfs_mnttab_find(LIBZFS_HANDLE.libzfs_handle, name, entry.as_mut_ptr()) == 0 {
        Some(entry.assume_init())
    } else {
        None
    }
}

pub(crate) unsafe fn zfs_open(name: *const libc::c_char) -> *mut sys::zfs_handle_t {
    LIBZFS_HANDLE.zfs_open(name)
}

pub(crate) unsafe fn zfs_close(handle: *mut sys::zfs_handle_t) {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_close(handle);
}

pub(crate) unsafe fn zfs_get_all_props(handle: *mut sys::zfs_handle_t) -> *mut sys::nvlist_t {
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_get_all_props(handle)
}

pub(crate) unsafe fn zfs_prop_get_numeric(
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
