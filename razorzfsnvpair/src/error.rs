use std::ffi;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NvListError {
    #[error("Invalid argument")]
    InvalidArgument,
    #[error("Insufficient memory")]
    InsufficientMemory,
}

impl NvListError {
    pub(crate) fn from_nvlist_rc(rc: i32) -> Result<(), Self> {
        match rc {
            0 => Ok(()),
            libc::EINVAL => Err(Self::InvalidArgument),
            libc::ENOMEM => Err(Self::InsufficientMemory),
            _ => unreachable!("invalid return code"),
        }
    }
}

impl From<ffi::NulError> for NvListError {
    fn from(_e: ffi::NulError) -> Self {
        Self::InvalidArgument
    }
}
