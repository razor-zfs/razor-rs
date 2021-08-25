pub use bookmark::Bookmark;
pub use filesystem::Filesystem;
pub use snapshot::Snapshot;
pub use volume::Volume;

pub(crate) use filesystem::FileSystemBuilder;
pub(crate) use volume::VolumeBuilder;

use std::ffi::CString;

use serde::{Deserialize, Serialize};

use super::libnvpair;
use super::property;
use super::sys;
use super::DatasetError;
use super::Result;
use serde_nvpair::from_nvlist;

mod bookmark;
mod filesystem;
mod snapshot;
mod volume;

#[derive(Debug, Deserialize, PartialEq, Clone)]
enum DatasetType {
    Filesystem(filesystem::Filesystem),
    Volume(volume::Volume),
}

/*#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Dataset {
    dataset: DatasetType,
}*/
