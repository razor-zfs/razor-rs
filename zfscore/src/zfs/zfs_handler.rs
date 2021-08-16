use super::sys;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub(crate) static ZFS_HANDLER: Lazy<Mutex<ZfsHandler>> = Lazy::new(|| {
    let mut handler = ZfsHandler::init();
    Mutex::new(handler)
});

#[derive(Debug)]
pub struct ZfsHandler {
    pub raw_libzfs_handle: *mut sys::libzfs_handle_t,
}

impl ZfsHandler {
    pub fn init() -> Self {
        ZfsHandler {
            raw_libzfs_handle: unsafe { sys::libzfs_init() },
        }
    }

    pub fn handler(&self) -> *mut sys::libzfs_handle_t {
        self.raw_libzfs_handle
    }
}

// TODO: check how to free zfs_handle_t
impl Drop for ZfsHandler {
    fn drop(&mut self) {
        unsafe { sys::libzfs_fini(self.raw_libzfs_handle) }
    }
}
