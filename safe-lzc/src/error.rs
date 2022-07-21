use thiserror::Error;

use super::*;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[error("LZC error: {code}")]
pub struct LzcError {
    pub code: libc::c_int,
}

impl LzcError {
    pub(crate) fn err(code: libc::c_int) -> Result<(), Self> {
        match code {
            0 => Ok(()),
            code => Err(Self { code }),
        }
    }
}

impl From<ffi::NulError> for LzcError {
    fn from(_: ffi::NulError) -> Self {
        Self { code: libc::EINVAL }
    }
}

impl From<nvpair::NvListError> for LzcError {
    fn from(e: nvpair::NvListError) -> Self {
        match e {
            nvpair::NvListError::InvalidArgument => Self { code: libc::EINVAL },
            nvpair::NvListError::OutOfMemory => Self { code: libc::ENOMEM },
            nvpair::NvListError::NotFound => Self { code: libc::ENOENT },
        }
    }
}
