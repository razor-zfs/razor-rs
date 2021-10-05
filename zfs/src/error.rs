use razor_zfscore::error;
use razor_zfscore::zfs_error_t;

use thiserror::Error;

use super::InvalidProperty;
use super::NvListError;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum DatasetError {
    #[error("operation not permited")]
    OperationNotPermited,
    #[error("No such file or directory")]
    NoSuchFileOrDirectory,
    #[error("No such process")]
    NoSuchProccess,
    #[error("Interrupted system call")]
    InterruptedSysCall,
    #[error("I/O error")]
    IOError,
    #[error("No such device or address")]
    NoSuchDeviceOrAddress,
    #[error("Argument list too long")]
    ArgumentListTooLong,
    #[error("Exec format error")]
    ExecFormatError,
    #[error("Bad file number")]
    BadFileNumber,
    #[error("No child processes")]
    NoChildProcess,
    #[error("Try again")]
    TryAgain,
    #[error("Out of memory")]
    OutOfMemory,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Bad address")]
    BadAddress,
    #[error("Block device required")]
    BlockDeviceRequired,
    #[error("Device or resource busy")]
    DeviceOrResourceBusy,
    #[error("File exists")]
    FileExists,
    #[error("Cross-device link")]
    CrossDeviceLink,
    #[error("No such device")]
    NoSuchDevice,
    #[error("Not a directory")]
    NotADirectory,
    #[error("Is a directory")]
    IsADirectory,
    #[error("Invalid argument")]
    InvalidArgument,
    #[error("File table overflow")]
    FileTableOverflow,
    #[error("Too many open files")]
    TooManyOpenFiles,
    #[error("Not a typewriter")]
    NotATypeWriter,
    #[error("Text file busy")]
    TextFileBusy,
    #[error("File too large")]
    FileTooLarge,
    #[error("No space left on device")]
    NoSpaceLeft,
    #[error("Illegal seek")]
    IllegalSeek,
    #[error("Read-only file system")]
    ReadOnlyFileSystem,
    #[error("Too many links")]
    TooManyLinks,
    #[error("Broken pipe")]
    BrokenPipe,
    #[error("Math argument out of domain of func")]
    MathArgOutOfFunc,
    #[error("Math result not representable")]
    MathResultNotRepresentable,
    #[error("block size out of range or does not match")]
    BadVolumeBlockSize,
    #[error("failed to convert string to C string")]
    StringConversionError(#[from] std::ffi::NulError),
    #[error("failed to create dataset")]
    DatasetCreationFailure,
    #[error("failed to load zfs module")]
    ZfsInitFailure,
    #[error(transparent)]
    InvalidProperty(#[from] InvalidProperty),
    #[error(transparent)]
    NvListError(#[from] NvListError),
    #[error(transparent)]
    CoreErr(#[from] error::CoreError),
    #[error("failed to get dataset")]
    DatasetGetError,
    #[error("failed to delete dataset")]
    DatasetDeleteError,
    #[error("property may be set but unable to remount filesystem")]
    ZfsMountFailed,
    #[error("property may be set but unable to reshare filesystem")]
    ZfsShareNfsFailed,
    #[error("unknown builder error")]
    Unknown,
}

// impl From<i32> for DatasetError {
//     fn from(rc: i32) -> Self {
//         const MOUNT_FAILED: i32 = zfs_error_t::EZFS_MOUNTFAILED as i32;
//         const SHARE_FAILED: i32 = zfs_error_t::EZFS_SHARENFSFAILED as i32;
//         match rc {
//             libc::EPERM => Self::OperationNotPermited,
//             libc::ENOENT => Self::NoSuchFileOrDirectory,
//             libc::ESRCH => Self::NoSuchProccess,
//             libc::EINTR => Self::InterruptedSysCall,
//             libc::EIO => Self::IOError,
//             libc::ENXIO => Self::NoSuchDeviceOrAddress,
//             libc::E2BIG => Self::ArgumentListTooLong,
//             libc::ENOEXEC => Self::ExecFormatError,
//             libc::EBADF => Self::BadFileNumber,
//             libc::ECHILD => Self::NoChildProcess,
//             libc::EAGAIN => Self::TryAgain,
//             libc::ENOMEM => Self::OutOfMemory,
//             libc::EACCES => Self::PermissionDenied,
//             libc::EFAULT => Self::BadAddress,
//             libc::ENOTBLK => Self::BlockDeviceRequired,
//             libc::EBUSY => Self::DeviceOrResourceBusy,
//             libc::EEXIST => Self::FileExists,
//             libc::EXDEV => Self::CrossDeviceLink,
//             libc::ENODEV => Self::NoSuchDevice,
//             libc::ENOTDIR => Self::NotADirectory,
//             libc::EISDIR => Self::IsADirectory,
//             libc::EINVAL => Self::InvalidArgument,
//             libc::ENFILE => Self::FileTableOverflow,
//             libc::EMFILE => Self::TooManyOpenFiles,
//             libc::ENOTTY => Self::NotATypeWriter,
//             libc::ETXTBSY => Self::TextFileBusy,
//             libc::EFBIG => Self::FileTooLarge,
//             libc::ENOSPC => Self::NoSpaceLeft,
//             libc::ESPIPE => Self::IllegalSeek,
//             libc::EROFS => Self::ReadOnlyFileSystem,
//             libc::EMLINK => Self::TooManyLinks,
//             libc::EPIPE => Self::BrokenPipe,
//             libc::EDOM => Self::MathArgOutOfFunc,
//             libc::ERANGE => Self::MathResultNotRepresentable,
//             MOUNT_FAILED => Self::ZfsMountFailed,
//             SHARE_FAILED => Self::ZfsShareNfsFailed,
//             _ => Self::Unknown,
//         }
//     }
// }

pub(crate) fn value_or_err<T>(val: T, rc: i32) -> Result<T, DatasetError> {
    const MOUNT_FAILED: i32 = zfs_error_t::EZFS_MOUNTFAILED as i32;
    const SHARE_FAILED: i32 = zfs_error_t::EZFS_SHARENFSFAILED as i32;
    match rc {
        0 => Ok(val),
        libc::EPERM => Err(DatasetError::OperationNotPermited),
        libc::ENOENT => Err(DatasetError::NoSuchFileOrDirectory),
        libc::ESRCH => Err(DatasetError::NoSuchProccess),
        libc::EINTR => Err(DatasetError::InterruptedSysCall),
        libc::EIO => Err(DatasetError::IOError),
        libc::ENXIO => Err(DatasetError::NoSuchDeviceOrAddress),
        libc::E2BIG => Err(DatasetError::ArgumentListTooLong),
        libc::ENOEXEC => Err(DatasetError::ExecFormatError),
        libc::EBADF => Err(DatasetError::BadFileNumber),
        libc::ECHILD => Err(DatasetError::NoChildProcess),
        libc::EAGAIN => Err(DatasetError::TryAgain),
        libc::ENOMEM => Err(DatasetError::OutOfMemory),
        libc::EACCES => Err(DatasetError::PermissionDenied),
        libc::EFAULT => Err(DatasetError::BadAddress),
        libc::ENOTBLK => Err(DatasetError::BlockDeviceRequired),
        libc::EBUSY => Err(DatasetError::DeviceOrResourceBusy),
        libc::EEXIST => Err(DatasetError::FileExists),
        libc::EXDEV => Err(DatasetError::CrossDeviceLink),
        libc::ENODEV => Err(DatasetError::NoSuchDevice),
        libc::ENOTDIR => Err(DatasetError::NotADirectory),
        libc::EISDIR => Err(DatasetError::IsADirectory),
        libc::EINVAL => Err(DatasetError::InvalidArgument),
        libc::ENFILE => Err(DatasetError::FileTableOverflow),
        libc::EMFILE => Err(DatasetError::TooManyOpenFiles),
        libc::ENOTTY => Err(DatasetError::NotATypeWriter),
        libc::ETXTBSY => Err(DatasetError::TextFileBusy),
        libc::EFBIG => Err(DatasetError::FileTooLarge),
        libc::ENOSPC => Err(DatasetError::NoSpaceLeft),
        libc::ESPIPE => Err(DatasetError::IllegalSeek),
        libc::EROFS => Err(DatasetError::ReadOnlyFileSystem),
        libc::EMLINK => Err(DatasetError::TooManyLinks),
        libc::EPIPE => Err(DatasetError::BrokenPipe),
        libc::EDOM => Err(DatasetError::MathArgOutOfFunc),
        libc::ERANGE => Err(DatasetError::MathResultNotRepresentable),
        MOUNT_FAILED => Err(DatasetError::ZfsMountFailed),
        SHARE_FAILED => Err(DatasetError::ZfsShareNfsFailed),
        _ => Err(DatasetError::Unknown),
    }
}
