use crate::zfsrpc_proto::PropErr;

use razor_zfs::error::DatasetError;
use razor_zfscore::error::CoreError;

use thiserror::Error;
use tonic::{Code, Status};

#[derive(Error, Debug)]
pub(crate) enum ZfsError {
    #[error("({0})")]
    Internal(PropErr),
    #[error("({0})")]
    AlreadyExists(PropErr),
}

const C_EEXIST: i32 = 17;

impl From<PropErr> for ZfsError {
    fn from(err: PropErr) -> Self {
        match err {
            PropErr::ZfsError(err) => err.into(),
            _ => Self::Internal(err),
        }
    }
}

impl From<DatasetError> for ZfsError {
    fn from(err: DatasetError) -> Self {
        match err {
            DatasetError::CoreErr(CoreError::LibcError(rc, _)) if rc == C_EEXIST => {
                Self::AlreadyExists(err)
            }
            _ => Self::Internal(PropErr::ZfsError(err)),
        }
    }
}

impl From<CoreError> for ZfsError {
    fn from(err: CoreError) -> Self {
        Self::Internal(PropErr::ZfsError(err.into()))
    }
}

impl From<ZfsError> for Status {
    fn from(err: ZfsError) -> Self {
        match err {
            ZfsError::Internal(err) => Self::new(Code::Internal, err.to_string()),
            ZfsError::AlreadyExists(err) => Self::new(Code::AlreadyExists, err.to_string()),
        }
    }
}
