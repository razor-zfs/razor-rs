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

use razor_nvpair as nvpair;

use razor_property::InvalidProperty;

pub use nvpair::NvListError;

pub use error::DatasetError;
pub use zfs::Bookmark;
pub use zfs::Filesystem;
pub use zfs::FilesystemBuilder;
pub use zfs::Snapshot;
pub use zfs::SnapshotBuilder;
pub use zfs::Volume;
pub use zfs::VolumeBuilder;
pub use zfs::Zfs;

use error::value_or_err;

mod error;
pub mod libzfs;
pub mod lzc;
pub mod zfs;

pub type Result<T, E = DatasetError> = std::result::Result<T, E>;
