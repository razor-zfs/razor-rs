//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//
/*
use serde::{de, ser};
use std::ffi;
use std::fmt::{self, Display};
use std::str;

#[derive(Clone, Debug, PartialEq)]
pub enum NvListError {
    Message(String),
    InvalidArgument,
    InsufficientMemory,
    UnmatchingVariables,
    RestrictedOperation,
    NameTypeError,
    ConversionError,
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

impl Display for NvListError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NvListError::Message(msg) => formatter.write_str(msg),
            NvListError::InvalidArgument => todo!(),
            NvListError::InsufficientMemory => todo!(),
            NvListError::UnmatchingVariables => todo!(),
            NvListError::RestrictedOperation => todo!(),
            NvListError::NameTypeError => todo!(),
            NvListError::ConversionError => todo!(),
            /* and so forth */
        }
    }
}

impl std::error::Error for NvListError {}
*/
