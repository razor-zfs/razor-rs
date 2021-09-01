use super::sys;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

pub(crate) static ZFS_HANDLER: Lazy<Mutex<LibZfsHandler>> = Lazy::new(|| {
    let handler = LibZfsHandler::init();
    Mutex::new(handler)
});

#[derive(Debug)]
pub(crate) struct LibZfsHandler {
    raw_libzfs_handle: *mut sys::libzfs_handle_t,
}

// TODO: check this!!
unsafe impl Send for LibZfsHandler {}
unsafe impl Sync for LibZfsHandler {}

impl LibZfsHandler {
    fn init() -> Self {
        dbg!("initializing zfs handler");
        Self {
            raw_libzfs_handle: unsafe { sys::libzfs_init() },
        }
    }

    pub(crate) fn handler(&self) -> *mut sys::libzfs_handle_t {
        self.raw_libzfs_handle
    }
}

// TODO: check how to free zfs_handle_t
impl Drop for LibZfsHandler {
    fn drop(&mut self) {
        unsafe { sys::libzfs_fini(self.raw_libzfs_handle) }
    }
}
