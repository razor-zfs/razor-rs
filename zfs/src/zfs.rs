use std::os::unix::io::AsRawFd;

use razor_property as property;
use razor_zfscore::lzc;

pub use dataset::Bookmark;
pub use dataset::FileSystemBuilder;
pub use dataset::Filesystem;
pub use dataset::Snapshot;
pub use dataset::Volume;
pub use dataset::VolumeBuilder;

use super::DatasetCollectorBuilder;
use super::Result;
use super::ZfsDatasetHandle;

mod dataset;

#[derive(Debug)]
pub struct Zfs {}

impl Zfs {
    pub fn filesystem() -> FileSystemBuilder {
        FileSystemBuilder::new()
    }

    pub fn volume() -> VolumeBuilder {
        VolumeBuilder::new()
    }

    pub fn destroy_dataset(name: impl AsRef<str>) -> Result<()> {
        lzc::destroy_dataset(name).map_err(|err| err.into())
    }

    pub fn dataset_exists(dataset: impl AsRef<str>) -> Result<()> {
        lzc::dataset_exists(dataset).map_err(|err| err.into())
    }

    pub fn list() -> DatasetCollectorBuilder {
        lzc::zfs_list()
    }

    pub fn list_from(name: impl AsRef<str>) -> DatasetCollectorBuilder {
        lzc::zfs_list_from(name)
    }

    pub fn get_filesystem(name: impl AsRef<str>) -> Result<Filesystem> {
        Filesystem::get_filesystem(name)
    }

    pub fn get_volume(name: impl AsRef<str>) -> Result<Volume> {
        Volume::get_volume(name)
    }

    pub fn get_snapshot(name: impl AsRef<str>) -> Result<Snapshot> {
        Snapshot::get(name)
    }

    pub fn send<S, F, U>(source: S, from: Option<F>, file: U) -> Result<()>
    where
        S: AsRef<str>,
        F: AsRef<str>,
        U: AsRawFd,
    {
        lzc::send(source, from, file)?;
        Ok(())
    }
}
