use razor_nvpair as libnvpair;
use razor_nvpair::NvListError;
use razor_zfscore_sys as sys;
pub use zfs::zfs_property;
pub(crate) use zfs::InvalidProperty;
pub use zfs::ZFS;

mod error;
mod zfs;

pub type Result<T> = std::result::Result<T, error::DatasetError>;
