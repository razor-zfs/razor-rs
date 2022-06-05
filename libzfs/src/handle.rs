use super::*;

pub(crate) static LIBZFS_HANDLE: Lazy<LibZfsHandle> = Lazy::new(LibZfsHandle::init);

#[derive(Debug)]
pub(crate) struct LibZfsHandle {
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
        sys::libzfs_print_on_error(libzfs_handle, libnvpair::boolean_t::B_FALSE);
        let version = Version::new();

        Self {
            libzfs_handle,
            version,
        }
    }

    pub(crate) fn handle(&self) -> *mut sys::libzfs_handle_t {
        self.libzfs_handle
    }

    pub(crate) fn version(&self) -> &Version {
        &self.version
    }
}
