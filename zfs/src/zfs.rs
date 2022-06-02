use std::os::unix::io::AsRawFd;

use razor_libzfscore::lzc;
use razor_property as property;

pub use dataset::Bookmark;
pub use dataset::Filesystem;
pub use dataset::FilesystemBuilder;
pub use dataset::Snapshot;
pub use dataset::SnapshotBuilder;
pub use dataset::Volume;
pub use dataset::VolumeBuilder;

use super::*;

#[cfg(feature = "cmd")]
mod cmd;
mod dataset;

#[derive(Debug)]
pub struct Zfs {}

impl Zfs {
    pub fn filesystem() -> FilesystemBuilder {
        FilesystemBuilder::new()
    }

    pub fn volume() -> VolumeBuilder {
        VolumeBuilder::new()
    }

    pub fn snapshot() -> SnapshotBuilder {
        SnapshotBuilder::new()
    }

    pub fn create_bookmark(
        snapshot: impl AsRef<str>,
        bookmark: impl AsRef<str>,
    ) -> Result<Bookmark> {
        lzc::bookmark(snapshot, &bookmark)?;
        Bookmark::get(bookmark)
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
        Filesystem::get(name)
    }

    pub fn get_volume(name: impl AsRef<str>) -> Result<Volume> {
        Volume::get(name)
    }

    pub fn get_snapshot(name: impl AsRef<str>) -> Result<Snapshot> {
        Snapshot::get(name)
    }

    pub fn get_bookmark(name: impl AsRef<str>) -> Result<Bookmark> {
        Bookmark::get(name)
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

    pub fn receive<S, O, U>(snapname: S, origin: Option<O>, force: bool, file: U) -> Result<()>
    where
        S: AsRef<str>,
        O: AsRef<str>,
        U: AsRawFd,
    {
        let raw = false;
        lzc::receive(snapname, origin, force, raw, file)?;
        Ok(())
    }
}
