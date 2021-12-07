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
