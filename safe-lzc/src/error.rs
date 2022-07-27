use std::borrow::Cow;
use std::ffi;
use std::fmt;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl fmt::Display for LzcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LZC error: {} ({})", libc_strerror(self.code), self.code)
    }
}

impl ::std::error::Error for LzcError {}

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

fn libc_strerror(code: i32) -> Cow<'static, str> {
    unsafe {
        let cstr = libc::strerror(code);
        ffi::CStr::from_ptr(cstr).to_string_lossy()
    }
}
