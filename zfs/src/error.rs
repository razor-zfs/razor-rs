use std::io;

use razor_zfscore::error;

use thiserror::Error;

use super::InvalidProperty;
use super::NvListError;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum DatasetError {
    #[error("failed to convert string to C string")]
    StringConversionError(#[from] std::ffi::NulError),
    #[error(transparent)]
    InvalidProperty(#[from] InvalidProperty),
    #[error(transparent)]
    NvListError(#[from] NvListError),
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
}

impl From<io::Error> for DatasetError {
    fn from(error: io::Error) -> Self {
        let code = error.raw_os_error().unwrap_or_default();
        Self::Unknown(code)
    }
}
