use super::*;

#[derive(Clone, Debug)]
pub struct Version;

impl Version {
    const ZFS_VERSION: &'static str = env!("ZFS_VERSION");

    pub(crate) unsafe fn new() -> Self {
        Self
    }

    pub fn ensure_compatible(&self) {
        let userland = self.userland();
        if !userland.starts_with(Self::ZFS_VERSION) {
            panic!(
                "libzfs version is not compatible (compiled against {}, but {} is found",
                Self::ZFS_VERSION,
                userland
            );
        }
    }

    pub fn kernel(&self) -> String {
        unsafe { zfs_version_kernel() }
    }

    pub fn userland(&self) -> String {
        unsafe { zfs_version_userland() }
    }

    pub fn compiled(&self) -> String {
        Self::ZFS_VERSION.to_string()
    }
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
