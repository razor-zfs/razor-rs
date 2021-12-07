use tonic::{Code, Status};

#[derive(Debug)]
pub enum ZfsError {
    InternalError(Status),
    AlreadyExists(Status),
}

impl From<Status> for ZfsError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::AlreadyExists => Self::AlreadyExists(status),
            _ => Self::InternalError(status),
        }
    }
}

impl From<ZfsError> for anyhow::Error {
    fn from(error: ZfsError) -> Self {
        anyhow::anyhow!(error)
    }
}
