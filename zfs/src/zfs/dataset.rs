use std::ffi;

use razor_nvpair as nvpair;
use razor_zfscore::lzc;

use lzc::zfs_prop_t::*;

use super::property;
use super::Result;
use super::ZfsDatasetHandle;

pub use bookmark::Bookmark;
pub use filesystem::FileSystemBuilder;
pub use filesystem::Filesystem;
pub use snapshot::Snapshot;
pub use snapshot::SnapshotBuilder;
pub use volume::Volume;
pub use volume::VolumeBuilder;

mod bookmark;
mod filesystem;
mod snapshot;
mod volume;
