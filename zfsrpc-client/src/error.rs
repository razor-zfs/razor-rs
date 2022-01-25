use thiserror::Error;
use tonic::{Code, Status};

#[derive(Debug, Error)]
pub enum Fixme {
    #[error("dataset properties is not supported yet")]
    Mango2456,
}

#[derive(Debug, Error)]
pub enum ZfsError {
    #[error("{0:?}")]
    InternalError(Status),
    #[error("{0:?}")]
    AlreadyExists(Status),
    #[error("{0}")]
    NotImplemented(Fixme),
}

impl From<Status> for ZfsError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::AlreadyExists => Self::AlreadyExists(status),
            _ => Self::InternalError(status),
        }
    }
}
