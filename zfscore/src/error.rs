use thiserror::Error;

use super::NvListError;

#[derive(Error, Debug, Clone)]
pub enum CoreError {
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
    NvListError(#[from] NvListError),
    #[error("failed to get dataset")]
    DatasetGetError,
    #[error("failed to delete dataset")]
    DatasetDeleteError,
    #[error("unknown builder error")]
    Unknown,
}

pub(crate) fn value_or_err<T>(val: T, rc: i32) -> Result<T, CoreError> {
    match rc {
        0 => Ok(val),
        libc::EPERM => Err(CoreError::OperationNotPermited),
        libc::ENOENT => Err(CoreError::NoSuchFileOrDirectory),
        libc::ESRCH => Err(CoreError::NoSuchProccess),
        libc::EINTR => Err(CoreError::InterruptedSysCall),
        libc::EIO => Err(CoreError::IOError),
        libc::ENXIO => Err(CoreError::NoSuchDeviceOrAddress),
        libc::E2BIG => Err(CoreError::ArgumentListTooLong),
        libc::ENOEXEC => Err(CoreError::ExecFormatError),
        libc::EBADF => Err(CoreError::BadFileNumber),
        libc::ECHILD => Err(CoreError::NoChildProcess),
        libc::EAGAIN => Err(CoreError::TryAgain),
        libc::ENOMEM => Err(CoreError::OutOfMemory),
        libc::EACCES => Err(CoreError::PermissionDenied),
        libc::EFAULT => Err(CoreError::BadAddress),
        libc::ENOTBLK => Err(CoreError::BlockDeviceRequired),
        libc::EBUSY => Err(CoreError::DeviceOrResourceBusy),
        libc::EEXIST => Err(CoreError::FileExists),
        libc::EXDEV => Err(CoreError::CrossDeviceLink),
        libc::ENODEV => Err(CoreError::NoSuchDevice),
        libc::ENOTDIR => Err(CoreError::NotADirectory),
        libc::EISDIR => Err(CoreError::IsADirectory),
        libc::EINVAL => Err(CoreError::InvalidArgument),
        libc::ENFILE => Err(CoreError::FileTableOverflow),
        libc::EMFILE => Err(CoreError::TooManyOpenFiles),
        libc::ENOTTY => Err(CoreError::NotATypeWriter),
        libc::ETXTBSY => Err(CoreError::TextFileBusy),
        libc::EFBIG => Err(CoreError::FileTooLarge),
        libc::ENOSPC => Err(CoreError::NoSpaceLeft),
        libc::ESPIPE => Err(CoreError::IllegalSeek),
        libc::EROFS => Err(CoreError::ReadOnlyFileSystem),
        libc::EMLINK => Err(CoreError::TooManyLinks),
        libc::EPIPE => Err(CoreError::BrokenPipe),
        libc::EDOM => Err(CoreError::MathArgOutOfFunc),
        libc::ERANGE => Err(CoreError::MathResultNotRepresentable),
        _ => Err(CoreError::Unknown),
    }
}
