use std::borrow::Cow;
use std::fmt;

use libzfs::zfs_error;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZfsError {
    error: libzfs::zfs_error_t,
    description: Cow<'static, str>,
}

impl fmt::Display for ZfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZFS error {} ({})", self.description, self.error)
    }
}

impl std::error::Error for ZfsError {}

impl ZfsError {
    pub fn is_success(&self) -> bool {
        self.error == zfs_error::EZFS_SUCCESS
    }

    pub fn from_libzfs_errno() -> Self {
        let code = libzfs_errno();
        let error = code as u32;
        let description = libzfs_error_description();

        Self { error, description }
    }

    pub fn from_rc(code: i32) -> Self {
        let libzfs_error = libzfs_errno();
        let error = code as u32;
        let description = if error == zfs_error::EZFS_SUCCESS {
            "success".into()
        } else if error < zfs_error::EZFS_NOMEM {
            libc_strerror(code)
        } else if error < zfs_error::EZFS_UNKNOWN {
            if code == libzfs_error {
                libzfs_error_description()
            } else {
                format!(
                    "rc ({}) and library errno ({}) mismatch",
                    code, libzfs_error
                )
                .into()
            }
        } else {
            "unknown".into()
        };

        Self { error, description }
    }

    pub fn result<T, U>(self, ok: T) -> Result<T, U>
    where
        U: From<Self>,
    {
        if self.is_success() {
            Ok(ok)
        } else {
            Err(self)?
        }
    }
}

fn libc_strerror(code: i32) -> Cow<'static, str> {
    unsafe {
        let cstr = libc::strerror(code);
        ffi::CStr::from_ptr(cstr).to_string_lossy()
    }
}
