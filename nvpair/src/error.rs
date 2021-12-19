use std::ffi;
use std::str;

use thiserror::Error;

#[derive(Clone, Debug, PartialEq)]
enum NvListErrorInternal {
    Ok,
    InvalidArgument,
    InsufficientMemory,
}

#[derive(Error, Clone, Debug, PartialEq)]
pub enum NvListError {
    #[error("({0})")]
    Message(String),
    #[error("Invalid argument")]
    InvalidArgument,
    #[error("Insufficient memory")]
    InsufficientMemory,
    #[error("Unmatching variables")]
    UnmatchingVariables,
    #[error("Restricted Operation")]
    RestrictedOperation,
    #[error("Name type error")]
    NameTypeError,
    #[error("Conversion error")]
    ConversionError,
    #[error("NvPair type error")]
    NvPairTypeError,
    #[error("Null pointer")]
    NullPointer,
    #[error("NvList Null pointer")]
    NvListNullPointer,
    #[error("NvPair doesn't exist")]
    NvPairDoesntExist,
    #[error("NvList doesn't exist")]
    NvListDoesntExist,
}

impl From<ffi::NulError> for NvListError {
    fn from(_e: ffi::NulError) -> Self {
        Self::InvalidArgument
    }
}

impl From<str::Utf8Error> for NvListError {
    fn from(_e: str::Utf8Error) -> Self {
        Self::ConversionError
    }
}

impl From<i32> for NvListErrorInternal {
    fn from(rc: i32) -> Self {
        match rc {
            0 => Self::Ok,
            libc::EINVAL => Self::InvalidArgument,
            libc::ENOMEM => Self::InsufficientMemory,
            _ => unreachable!("invalid return code"),
        }
    }
}

pub(crate) fn value_or_err<T>(val: T, rc: i32) -> Result<T, NvListError> {
    match rc {
        0 => Ok(val),
        libc::EINVAL => Err(NvListError::InvalidArgument),
        libc::ENOMEM => Err(NvListError::InsufficientMemory),
        _ => unreachable!("invalid return code"),
    }
}
