use std::io;

use thiserror::Error;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DatasetError {
    #[error("failed to convert string to C string")]
    StringConversionError(#[from] std::ffi::NulError),
    #[error(transparent)]
    InvalidProperty(#[from] zfs::property::InvalidProperty),
    #[error(transparent)]
    NvListError(#[from] NvListError),
    #[error("Snapshot name must contain @ ({0})")]
    InvalidSnapshotName(String),
    #[error(transparent)]
    CoreErr(#[from] libzfs::ZfsError),
    #[error("unknown builder error, error code: ({0})")]
    Unknown(i32),
}

impl DatasetError {
    pub fn missing_value() -> Self {
        zfs::property::InvalidProperty::invalid_value("Value is missing").into()
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

impl From<lzc::LzcError> for DatasetError {
    fn from(e: lzc::LzcError) -> Self {
        libzfs::ZfsError::from_rc(e.code).into()
    }
}

pub(crate) fn value_or_err<T>(value: T, rc: i32) -> Result<T, DatasetError> {
    libzfs::ZfsError::from_rc(rc).result(value)
}
