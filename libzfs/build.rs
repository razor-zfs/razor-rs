use std::ffi;

use razor_libzfs_sys as sys;

const MAX_VERSION_LEN: usize = 128;

unsafe fn zfs_version_userland() -> String {
    let mut version = [0; MAX_VERSION_LEN];
    sys::zfs_version_userland(version.as_mut_ptr(), MAX_VERSION_LEN as libc::c_int);
    ffi::CStr::from_ptr(version.as_ptr())
        .to_string_lossy()
        .into_owned()
}

fn main() {
    let zfs_version = unsafe { zfs_version_userland() };
    println!("cargo:rustc-env=ZFS_VERSION={zfs_version}");
}
