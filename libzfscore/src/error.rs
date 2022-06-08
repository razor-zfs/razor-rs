use std::fmt;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZfsError {
    error: libzfs::zfs_error_t,
}

impl fmt::Display for ZfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZFS error {:?}", self.error)
    }
}

impl std::error::Error for ZfsError {}

impl From<libzfs::zfs_error_t> for ZfsError {
    fn from(error: libzfs::zfs_error_t) -> Self {
        Self { error }
    }
}

impl ZfsError {
    pub fn is_success(&self) -> bool {
        self.error == libzfs::zfs_error::EZFS_SUCCESS
    }
}
