use razor_zfscore_sys as sys;
use thiserror::Error;

use super::NvListError;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum CoreError {
    #[error("failed to convert string to C string")]
    StringConversionError(#[from] std::ffi::NulError),
    #[error(transparent)]
    NvListError(#[from] NvListError),
    #[error("requested dataset not exist")]
    DatasetNotExist,
    #[error("zfs error ({0}): ({1})")]
    ZfsInternalError(i32, String),
    #[error("error ({0}): ({1})")]
    LibcError(i32, String),
    #[error("unknown error, error code: ({0})")]
    Unknown(i32),
}

pub(crate) fn value_or_err<T>(val: T, rc: i32) -> Result<T, CoreError> {
    const MOUNT_FAILED: i32 = sys::zfs_error::EZFS_MOUNTFAILED as i32;
    const SHARE_FAILED: i32 = sys::zfs_error::EZFS_SHARENFSFAILED as i32;

    match rc {
        0 => Ok(val),
        libc::EPERM => Err(CoreError::LibcError(
            rc,
            "operation not permited".to_string(),
        )),
        libc::ENOENT => Err(CoreError::LibcError(
            rc,
            "no such file or directory".to_string(),
        )),
        libc::ESRCH => Err(CoreError::LibcError(rc, "no such process".to_string())),
        libc::EINTR => Err(CoreError::LibcError(
            rc,
            "interrupted system call".to_string(),
        )),
        libc::EIO => Err(CoreError::LibcError(rc, "IO error".to_string())),
        libc::ENXIO => Err(CoreError::LibcError(
            rc,
            "no such device or address".to_string(),
        )),
        libc::E2BIG => Err(CoreError::LibcError(
            rc,
            "argument list too long".to_string(),
        )),
        libc::ENOEXEC => Err(CoreError::LibcError(rc, "exec format error".to_string())),
        libc::EBADF => Err(CoreError::LibcError(rc, "bad file number".to_string())),
        libc::ECHILD => Err(CoreError::LibcError(rc, "no child proccess".to_string())),
        libc::EAGAIN => Err(CoreError::LibcError(rc, "try again".to_string())),
        libc::ENOMEM => Err(CoreError::LibcError(rc, "out of memory".to_string())),
        libc::EACCES => Err(CoreError::LibcError(rc, "permission denied".to_string())),
        libc::EFAULT => Err(CoreError::LibcError(rc, "bad address".to_string())),
        libc::ENOTBLK => Err(CoreError::LibcError(
            rc,
            "block device required".to_string(),
        )),
        libc::EBUSY => Err(CoreError::LibcError(
            rc,
            "device or resource busy".to_string(),
        )),
        libc::EEXIST => Err(CoreError::LibcError(rc, "file exists".to_string())),
        libc::EXDEV => Err(CoreError::LibcError(rc, "cross device link".to_string())),
        libc::ENODEV => Err(CoreError::LibcError(rc, "no such device".to_string())),
        libc::ENOTDIR => Err(CoreError::LibcError(rc, "not a directory".to_string())),
        libc::EISDIR => Err(CoreError::LibcError(rc, "is a directory".to_string())),
        libc::EINVAL => Err(CoreError::LibcError(rc, "invalid argument".to_string())),
        libc::ENFILE => Err(CoreError::LibcError(rc, "file table overflow".to_string())),
        libc::EMFILE => Err(CoreError::LibcError(rc, "too many open files".to_string())),
        libc::ENOTTY => Err(CoreError::LibcError(rc, "not a type writter".to_string())),
        libc::ETXTBSY => Err(CoreError::LibcError(rc, "text file busy".to_string())),
        libc::EFBIG => Err(CoreError::LibcError(rc, "file too large".to_string())),
        libc::ENOSPC => Err(CoreError::LibcError(rc, "no space left".to_string())),
        libc::ESPIPE => Err(CoreError::LibcError(rc, "illegal seek".to_string())),
        libc::EROFS => Err(CoreError::LibcError(rc, "readonly file system".to_string())),
        libc::EMLINK => Err(CoreError::LibcError(rc, "too many links".to_string())),
        libc::EPIPE => Err(CoreError::LibcError(rc, "broken pipe".to_string())),
        libc::EDOM => Err(CoreError::LibcError(
            rc,
            "math argument out of func".to_string(),
        )),
        libc::ERANGE => Err(CoreError::LibcError(
            rc,
            "math result not representable".to_string(),
        )),
        MOUNT_FAILED => Err(CoreError::ZfsInternalError(rc, "mount failed".to_string())),
        SHARE_FAILED => Err(CoreError::ZfsInternalError(
            rc,
            "share nfs failed".to_string(),
        )),
        everything_else => Err(CoreError::Unknown(everything_else)),
    }
}
