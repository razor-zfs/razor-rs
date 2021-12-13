use razor_zfs::error::DatasetError;
use razor_zfscore::error::CoreError;

use thiserror::Error;
use tonic::{Code, Status};

#[derive(Error, Debug)]
pub(crate) enum ZfsError {
    #[error("({0})")]
    Internal(DatasetError),
    #[error("({0})")]
    AlreadyExists(DatasetError),
    #[error("({0})")]
    MountFs(std::io::Error),
}

const C_EEXIST: i32 = 17;

impl From<DatasetError> for ZfsError {
    fn from(err: DatasetError) -> Self {
        match err {
            DatasetError::CoreErr(CoreError::LibcError(rc, _)) if rc == C_EEXIST => {
                Self::AlreadyExists(err)
            }
            _ => Self::Internal(err),
        }
    }
}

impl From<CoreError> for ZfsError {
    fn from(err: CoreError) -> Self {
        Self::Internal(DatasetError::CoreErr(err))
    }
}

impl From<ZfsError> for Status {
    fn from(err: ZfsError) -> Self {
        match err {
            ZfsError::Internal(err) => Self::new(Code::Internal, err.to_string()),
            ZfsError::AlreadyExists(err) => Self::new(Code::AlreadyExists, err.to_string()),
            ZfsError::MountFs(err) => Self::new(Code::Internal, err.to_string()),
        }
    }
}
