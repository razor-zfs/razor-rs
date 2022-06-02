#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

use razor_libzfscore::DatasetCollectorBuilder;
use razor_property::InvalidProperty;

pub use razor_libzfscore::zfs_type_t;
pub use razor_libzfscore::ZfsDatasetHandle;
pub use razor_nvpair::NvListError;

pub use error::DatasetError;
pub use zfs::Bookmark;
pub use zfs::Filesystem;
pub use zfs::FilesystemBuilder;
pub use zfs::Snapshot;
pub use zfs::SnapshotBuilder;
pub use zfs::Volume;
pub use zfs::VolumeBuilder;
pub use zfs::Zfs;

mod error;
mod zfs;

pub type Result<T> = std::result::Result<T, DatasetError>;
