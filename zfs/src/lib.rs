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

pub use zfs::property;
pub use zfs::Bookmark;
pub use zfs::Filesystem;
pub use zfs::Snapshot;
pub use zfs::Volume;
pub use zfs::Zfs;

use razor_nvpair as libnvpair;
use razor_nvpair::NvListError;
use razor_zfscore_sys as sys;

mod error;
mod zfs;

pub type Result<T> = std::result::Result<T, error::DatasetError>;
