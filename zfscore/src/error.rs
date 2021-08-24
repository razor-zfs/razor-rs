use thiserror::Error;

use super::NvListError;

#[derive(Debug, Error)]
pub enum InvalidProperty {
    #[error("No such property ({0})")]
    NoSuchProperty(String),
    #[error("Invalid source ({0})")]
    InvalidSource(String),
    #[error("Invalid value ({0})")]
    InvalidValue(String),
}

impl InvalidProperty {
    pub(crate) fn _no_such_property(prop: impl ToString) -> Self {
        Self::NoSuchProperty(prop.to_string())
    }

    pub(crate) fn invalid_value(value: impl ToString) -> Self {
        Self::InvalidValue(value.to_string())
    }
}

#[derive(Error, Debug)]
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
    #[error("failed to get dataset")]
    DatasetGetError,
    #[error("failed to delete dataset")]
    DatasetDeleteError,
    #[error("unknown builder error")]
    Unknown,
}

impl From<i32> for DatasetError {
    fn from(rc: i32) -> Self {
        match rc {
            libc::EPERM => DatasetError::OperationNotPermited,
            libc::ENOENT => DatasetError::NoSuchFileOrDirectory,
            libc::ESRCH => DatasetError::NoSuchProccess,
            libc::EINTR => DatasetError::InterruptedSysCall,
            libc::EIO => DatasetError::IOError,
            libc::ENXIO => DatasetError::NoSuchDeviceOrAddress,
            libc::E2BIG => DatasetError::ArgumentListTooLong,
            libc::ENOEXEC => DatasetError::ExecFormatError,
            libc::EBADF => DatasetError::BadFileNumber,
            libc::ECHILD => DatasetError::NoChildProcess,
            libc::EAGAIN => DatasetError::TryAgain,
            libc::ENOMEM => DatasetError::OutOfMemory,
            libc::EACCES => DatasetError::PermissionDenied,
            libc::EFAULT => DatasetError::BadAddress,
            libc::ENOTBLK => DatasetError::BlockDeviceRequired,
            libc::EBUSY => DatasetError::DeviceOrResourceBusy,
            libc::EEXIST => DatasetError::FileExists,
            libc::EXDEV => DatasetError::CrossDeviceLink,
            libc::ENODEV => DatasetError::NoSuchDevice,
            libc::ENOTDIR => DatasetError::NotADirectory,
            libc::EISDIR => DatasetError::IsADirectory,
            libc::EINVAL => DatasetError::InvalidArgument,
            libc::ENFILE => DatasetError::FileTableOverflow,
            libc::EMFILE => DatasetError::TooManyOpenFiles,
            libc::ENOTTY => DatasetError::NotATypeWriter,
            libc::ETXTBSY => DatasetError::TextFileBusy,
            libc::EFBIG => DatasetError::FileTooLarge,
            libc::ENOSPC => DatasetError::NoSpaceLeft,
            libc::ESPIPE => DatasetError::IllegalSeek,
            libc::EROFS => DatasetError::ReadOnlyFileSystem,
            libc::EMLINK => DatasetError::TooManyLinks,
            libc::EPIPE => DatasetError::BrokenPipe,
            libc::EDOM => DatasetError::MathArgOutOfFunc,
            libc::ERANGE => DatasetError::MathResultNotRepresentable,
            _ => DatasetError::Unknown,
        }
    }
}
