pub use dataset::Bookmark;
pub use dataset::Filesystem;
pub use dataset::Snapshot;
pub use dataset::Volume;

use std::ffi::CString;

use super::libnvpair;
use super::sys;
use super::Result;
use crate::error::DatasetError;

use dataset::FileSystemBuilder;
use dataset::VolumeBuilder;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub mod property;

mod dataset;
mod zfs_handler;

pub static ZFS: Lazy<Mutex<Zfs>> = Lazy::new(|| {
    let zfs = Zfs::init();
    Mutex::new(zfs)
});

#[derive(Debug)]
pub struct Zfs {}

impl Zfs {
    fn init() -> Self {
        let guard = zfs_handler::ZFS_HANDLER.lock().unwrap();
        drop(guard);
        Self {}
    }

    pub fn filesystem(name: impl AsRef<str>) -> FileSystemBuilder {
        ZFS.lock().unwrap().filesystem_builder(name)
    }

    fn filesystem_builder(&self, name: impl AsRef<str>) -> FileSystemBuilder {
        FileSystemBuilder::new(name)
    }

    pub fn volume(name: impl AsRef<str>) -> VolumeBuilder {
        ZFS.lock().unwrap().volume_builder(name)
    }

    fn volume_builder(&self, name: impl AsRef<str>) -> VolumeBuilder {
        VolumeBuilder::new(name)
    }

    pub fn destroy_dataset(name: impl AsRef<str>) -> Result<()> {
        ZFS.lock().unwrap().destroy(name)
    }

    fn destroy(&self, name: impl AsRef<str>) -> Result<()> {
        if unsafe { sys::lzc_destroy(CString::new(name.as_ref())?.as_ptr()) } != 0 {
            return Err(DatasetError::DatasetDeleteError);
        }

        Ok(())
    }
}
