use super::lzc;

#[derive(Debug)]
pub struct ZfsDatasetHandle {
    pub(super) handle: *mut lzc::zfs_handle_t,
}
