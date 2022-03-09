use razor_zfs as zfs;
use razor_zfscore::error::CoreError;

use thiserror::Error;
use tonic::{Code, Status};

#[derive(Error, Debug)]
pub(crate) enum ZfsError {
    #[error("({0})")]
    Internal(zfs::DatasetError),
    #[error("({0})")]
    AlreadyExists(zfs::DatasetError),
    #[error("({0})")]
    NotFound(zfs::DatasetError),
    #[error("({0})")]
    MountFs(std::io::Error),
}

const C_ENOENT: i32 = 2;
const C_EEXIST: i32 = 17;

impl From<zfs::DatasetError> for ZfsError {
    fn from(err: zfs::DatasetError) -> Self {
        match err {
            zfs::DatasetError::CoreErr(CoreError::LibcError(rc, _)) if rc == C_EEXIST => {
                Self::AlreadyExists(err)
            }
            zfs::DatasetError::CoreErr(CoreError::LibcError(rc, _)) if rc == C_ENOENT => {
                Self::NotFound(err)
            }
            _ => Self::Internal(err),
        }
    }
}

impl From<CoreError> for ZfsError {
    fn from(err: CoreError) -> Self {
        Self::Internal(zfs::DatasetError::CoreErr(err))
    }
}

impl From<ZfsError> for Status {
    fn from(err: ZfsError) -> Self {
        match err {
            ZfsError::Internal(err) => Self::new(Code::Internal, err.to_string()),
            ZfsError::AlreadyExists(err) => Self::new(Code::AlreadyExists, err.to_string()),
            ZfsError::NotFound(err) => Self::new(Code::NotFound, err.to_string()),
            ZfsError::MountFs(err) => Self::new(Code::Internal, err.to_string()),
        }
    }
}
