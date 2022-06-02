use std::ffi;
use std::mem;
use std::ptr;

use once_cell::sync::Lazy;

use super::*;

static LIBZFS_HANDLE: Lazy<LibZfsHandle> = Lazy::new(LibZfsHandle::init);

#[derive(Debug)]
struct LibZfsHandle {
    libzfs_handle: *mut sys::libzfs_handle_t,
    version: Version,
}

unsafe impl Send for LibZfsHandle {}
unsafe impl Sync for LibZfsHandle {}

impl LibZfsHandle {
    fn init() -> Self {
        unsafe { Self::init_impl() }
    }

    unsafe fn init_impl() -> Self {
        let libzfs_handle = sys::libzfs_init();
        if libzfs_handle.is_null() {
            panic!("libzfs_init failed");
        }
        sys::libzfs_print_on_error(libzfs_handle, sys::boolean_t::B_FALSE);
        let version = Version::new();

        Self {
            libzfs_handle,
            version,
        }
    }
}

pub(crate) unsafe fn zfs_open(name: *const libc::c_char) -> *mut sys::zfs_handle_t {
    let types = sys::zfs_type_t::ZFS_TYPE_FILESYSTEM
        | sys::zfs_type_t::ZFS_TYPE_VOLUME
        | sys::zfs_type_t::ZFS_TYPE_SNAPSHOT
        | sys::zfs_type_t::ZFS_TYPE_POOL
        | sys::zfs_type_t::ZFS_TYPE_BOOKMARK;
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
    Lazy::force(&LIBZFS_HANDLE);
    sys::zfs_iter_filesystems(handle, Some(f), ptr);
}

pub(crate) unsafe fn zfs_iter_snapshots(
    handle: *mut sys::zfs_handle_t,
    simple: bool,
    f: unsafe extern "C" fn(*mut sys::zfs_handle_t, *mut libc::c_void) -> libc::c_int,
    data: *mut libc::c_void,
    min_txg: u64,
    max_txg: u64,
) {
    Lazy::force(&LIBZFS_HANDLE);
    let simple = match simple {
        false => sys::boolean_t::B_TRUE,
        true => sys::boolean_t::B_FALSE,
    };
    sys::zfs_iter_snapshots(handle, simple, Some(f), data, min_txg, max_txg);
}

const MAX_VERSION_LEN: usize = 128;
unsafe fn zfs_version_kernel() -> String {
    let mut version = [0; MAX_VERSION_LEN];
    sys::zfs_version_kernel(version.as_mut_ptr(), MAX_VERSION_LEN as libc::c_int);
    ffi::CStr::from_ptr(version.as_ptr())
        .to_string_lossy()
        .into_owned()
}

unsafe fn zfs_version_userland() -> String {
    let mut version = [0; MAX_VERSION_LEN];
    sys::zfs_version_userland(version.as_mut_ptr(), MAX_VERSION_LEN as libc::c_int);
    ffi::CStr::from_ptr(version.as_ptr())
        .to_string_lossy()
        .into_owned()
}

pub(crate) fn zfs_version() -> Version {
    LIBZFS_HANDLE.version.clone()
}

#[derive(Clone, Debug)]
pub struct Version {
    kernel: String,
    userland: String,
}

impl Version {
    const ZFS_VERSION: &'static str = "zfs-2.1.2";

    unsafe fn new() -> Self {
        let kernel = zfs_version_kernel();
        let userland = zfs_version_userland();
        Self { kernel, userland }
    }

    pub fn ensure_compatible(&self) {
        if !self.userland.starts_with(Self::ZFS_VERSION) {
            panic!(
                "libzfs version is not compatible (I was compiled against {}, but {} is found",
                Self::ZFS_VERSION,
                self.userland
            );
        }
    }

    pub fn kernel(&self) -> &str {
        &self.kernel
    }

    pub fn userland(&self) -> &str {
        &self.userland
    }
}
