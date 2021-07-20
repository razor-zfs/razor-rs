use std::ffi;

use serde::{de, ser};
use std::fmt::{self, Display};

#[derive(Clone, Debug, PartialEq)]
pub enum NvListError {
    Message(String),
    InvalidArgument,
    InsufficientMemory,
    UnmatchingVariables,
    RestrictedOperation,
    NameTypeError,
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

impl ser::Error for NvListError {
    fn custom<T: Display>(msg: T) -> Self {
        NvListError::Message(msg.to_string())
    }
}

impl de::Error for NvListError {
    fn custom<T: Display>(msg: T) -> Self {
        NvListError::Message(msg.to_string())
    }
}

impl Display for NvListError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NvListError::Message(msg) => formatter.write_str(msg),
            NvListError::InvalidArgument => todo!(),
            NvListError::InsufficientMemory => todo!(),
            NvListError::UnmatchingVariables => todo!(),
            NvListError::RestrictedOperation => todo!(),
            NvListError::NameTypeError => todo!(),
            /* and so forth */
        }
    }
}

impl std::error::Error for NvListError {}
