use serde::{de, ser};
use std::ffi;
use std::fmt::{self, Display};
use std::str;

#[derive(Clone, Debug, PartialEq)]
enum NvListErrorInternal {
    Ok,
    InvalidArgument,
    InsufficientMemory,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NvListError {
    Message(String),
    InvalidArgument,
    InsufficientMemory,
    UnmatchingVariables,
    RestrictedOperation,
    NameTypeError,
    ConversionError,
    NvPairTypeError,
    NullPointer,
    NvListNullPointer,
    NvPairDontExist,
    NvListDontExist,
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

impl From<str::Utf8Error> for NvListError {
    fn from(_e: str::Utf8Error) -> Self {
        Self::ConversionError
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

impl fmt::Display for NvListError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message(msg) => formatter.write_str(msg),
            Self::InvalidArgument => todo!(),
            Self::InsufficientMemory => todo!(),
            Self::UnmatchingVariables => todo!(),
            Self::RestrictedOperation => todo!(),
            Self::NameTypeError => todo!(),
            Self::ConversionError => todo!(),
            Self::NvPairTypeError => todo!(),
            Self::NullPointer => todo!(),
            Self::NvListNullPointer => todo!(),
            Self::NvPairDontExist => todo!(),
            Self::NvListDontExist => todo!(),
            /* and so forth */
        }
    }
}

impl std::error::Error for NvListError {}

/*impl TryFrom<i32> for () {
    type Error = NvListError;

    fn try_from(rc: i32) -> Result<Self, Self::Error> {
        match rc {
            0 => Ok(()),
            libc::EINVAL => Err(Self::InvalidArgument),
            libc::ENOMEM => Err(Self::InsufficientMemory),
            _ => unreachable!("invalid return code"),
        }
    }
}*/

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
