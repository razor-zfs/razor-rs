use razor_zfscore::lzc;

pub use dataset::Bookmark;
pub use dataset::Filesystem;
pub use dataset::Snapshot;
pub use dataset::Volume;

use super::Result;
use super::ZfsDatasetHandle;

pub use dataset::FileSystemBuilder;
pub use dataset::VolumeBuilder;

pub mod property;

mod dataset;

#[derive(Debug)]
pub struct Zfs {}

impl Zfs {
    pub fn filesystem(name: impl AsRef<str>) -> FileSystemBuilder {
        FileSystemBuilder::new(name)
    }

    pub fn volume() -> VolumeBuilder {
        VolumeBuilder::new()
    }

    pub fn destroy_dataset(name: impl AsRef<str>) -> Result<()> {
        lzc::destroy_dataset(name).map_err(|err| err.into())
    }

    pub fn get_filesystem(name: impl AsRef<str>) -> Result<Filesystem> {
        Filesystem::get_filesystem(name)
    }

    pub fn get_volume(name: impl AsRef<str>) -> Result<Volume> {
        Volume::get_volume(name)
    }
}
