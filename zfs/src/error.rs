use std::io;

use razor_zfscore::error;

use thiserror::Error;

use super::InvalidProperty;
use super::NvListError;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DatasetError {
    #[error("failed to convert string to C string")]
    StringConversionError(#[from] std::ffi::NulError),
    #[error(transparent)]
    InvalidProperty(#[from] InvalidProperty),
    #[error(transparent)]
    NvListError(#[from] NvListError),
    #[error("Snapshot name must contain @ ({0})")]
    InvalidSnapshotName(String),
    #[error(transparent)]
    CoreErr(#[from] error::CoreError),
    #[error("unknown builder error, error code: ({0})")]
    Unknown(i32),
}

impl DatasetError {
    pub fn missing_value() -> Self {
        Self::InvalidProperty(InvalidProperty::InvalidValue(
            "Value is missing".to_string(),
        ))
    }

    pub fn invalid_snapshot_name(name: impl AsRef<str>) -> Self {
        Self::InvalidSnapshotName(name.as_ref().to_string())
    }
}

impl From<io::Error> for DatasetError {
    fn from(error: io::Error) -> Self {
        let code = error.raw_os_error().unwrap_or_default();
        Self::Unknown(code)
    }
}
