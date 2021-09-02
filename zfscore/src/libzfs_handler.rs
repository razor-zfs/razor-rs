use once_cell::sync::Lazy;
use parking_lot::Mutex;

use super::sys;

static LIB_ZFS_HANDLER: Lazy<Mutex<LibZfsHandler>> = Lazy::new(|| {
    let handler = LibZfsHandler::init();
    Mutex::new(handler)
});

#[derive(Debug)]
pub(crate) struct LibZfsHandler {
    raw_libzfs_handle: *mut sys::libzfs_handle_t,
}

unsafe impl Send for LibZfsHandler {}
unsafe impl Sync for LibZfsHandler {}

impl LibZfsHandler {
    fn init() -> Self {
        dbg!("initializing zfs handler");
        Self {
            raw_libzfs_handle: unsafe { sys::libzfs_init() },
        }
    }

    pub(crate) fn handler() -> *mut sys::libzfs_handle_t {
        LIB_ZFS_HANDLER.lock().raw_libzfs_handle
    }
}
