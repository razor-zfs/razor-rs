use std::mem;

use once_cell::sync::Lazy;
use razor_zfscore_sys as sys;

static LIB_ZFS_HANDLE: Lazy<LibZfsHandle> = Lazy::new(LibZfsHandle::init);

#[derive(Debug)]
struct LibZfsHandle {
    libzfs_handle: *mut sys::libzfs_handle_t,
}

unsafe impl Send for LibZfsHandle {}
unsafe impl Sync for LibZfsHandle {}

impl LibZfsHandle {
    fn init() -> Self {
        Self {
            libzfs_handle: unsafe { sys::libzfs_init() },
        }
    }
}

pub(crate) unsafe fn make_dataset_handle(name: *const libc::c_char) -> *mut sys::zfs_handle_t {
    sys::make_dataset_handle(LIB_ZFS_HANDLE.libzfs_handle, name)
}

pub(crate) unsafe fn libzfs_mnttab_find(name: *const libc::c_char) -> Option<sys::mnttab> {
    let mut entry = mem::MaybeUninit::uninit();
    if sys::libzfs_mnttab_find(LIB_ZFS_HANDLE.libzfs_handle, name, entry.as_mut_ptr()) == 0 {
        Some(entry.assume_init())
    } else {
        None
    }
}
