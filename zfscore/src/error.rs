use thiserror::Error;

use super::InvalidProperty;
use super::NvListError;

#[derive(Error, Debug)]
pub enum DatasetError {
    #[error(transparent)]
    InvalidProperty(#[from] InvalidProperty),
    #[error("failed to add parameter")]
    BuildError(#[from] NvListError),
    #[error("unknown builder error")]
    Unknown,
}
