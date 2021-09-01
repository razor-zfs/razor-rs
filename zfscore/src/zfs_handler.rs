use super::sys;

#[derive(Debug)]
pub(crate) struct ZfsHandler {
    raw: *mut sys::zfs_handle_t,
}

impl ZfsHandler {}
