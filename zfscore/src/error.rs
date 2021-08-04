use std::ffi::NulError;

use thiserror::Error;

use super::InvalidProperty;
use super::NvListError;

#[derive(Error, Debug)]
pub enum DatasetError {
    #[error("block size out of range or does not match")]
    BadVolumeBlockSize,
    #[error("failed to convert string to C string")]
    StringConversionError(#[from] NulError),
    #[error("failed to create dataset")]
    DatasetCreationFailure,
    #[error("failed to load zfs module")]
    ZfsInitFailure,
    #[error(transparent)]
    InvalidProperty(#[from] InvalidProperty),
    #[error("failed to add parameter")]
    BuildError(#[from] NvListError),
    #[error("unknown builder error")]
    Unknown,
}
