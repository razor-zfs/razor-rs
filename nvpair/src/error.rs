use std::ffi;
use std::str;

use thiserror::Error;

#[derive(Copy, Clone, Debug, PartialEq)]
enum NvListRcInternal {
    Ok,
    InvalidArgument,
    InsufficientMemory,
    UnknownError(i32),
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

impl From<i32> for NvListRcInternal {
    fn from(rc: i32) -> Self {
        match rc {
            0 => Self::Ok,
            libc::EINVAL => Self::InvalidArgument,
            libc::ENOMEM => Self::InsufficientMemory,
            rc => Self::UnknownError(rc),
        }
    }
}

impl NvListRcInternal {
    fn value_or_err<T>(self, val: T) -> Result<T, NvListError> {
        match self {
            Self::Ok => Ok(val),
            Self::UnknownError(rc) => Err(NvListError::Message(format!("unknown error {}", rc))),
            Self::InsufficientMemory => Err(NvListError::InsufficientMemory),
            Self::InvalidArgument => Err(NvListError::InvalidArgument),
        }
    }
}

pub(crate) fn value_or_err<T>(val: T, rc: i32) -> Result<T, NvListError> {
    NvListRcInternal::from(rc).value_or_err(val)
}
