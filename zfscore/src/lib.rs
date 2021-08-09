use razorzfsnvpair::NvListError;
pub use zfs::dataset;
pub use zfs::dataset::Filesystem;
pub use zfs::zfs_property;
pub(crate) use zfs::InvalidProperty;

mod error;
mod zfs;

pub type Result<T> = std::result::Result<T, error::DatasetError>;
