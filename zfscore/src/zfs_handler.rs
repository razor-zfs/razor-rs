use super::sys;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

pub(crate) static ZFS_HANDLER: Lazy<Mutex<ZfsHandler>> = Lazy::new(|| {
    let handler = ZfsHandler::init();
    Mutex::new(handler)
});

#[derive(Debug)]
pub(crate) struct ZfsHandler {
    raw_libzfs_handle: *mut sys::libzfs_handle_t,
}

// TODO: check this!!
unsafe impl Send for ZfsHandler {}
unsafe impl Sync for ZfsHandler {}

impl ZfsHandler {
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
impl Drop for ZfsHandler {
    fn drop(&mut self) {
        unsafe { sys::libzfs_fini(self.raw_libzfs_handle) }
    }
}
