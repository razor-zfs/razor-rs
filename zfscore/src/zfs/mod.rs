use std::ffi::CString;

use super::error::DatasetError;
use super::libnvpair;
use super::sys;
use super::Result;
use dataset::FileSystemBuilder;
use dataset::VolumeBuilder;
use once_cell::sync::Lazy;
pub(crate) use property::InvalidProperty;
use std::sync::Mutex;

mod dataset;
mod property;
mod zfs_handler;
pub mod zfs_property;
pub mod zpool_property;

pub static ZFS: Lazy<Mutex<Zfs>> = Lazy::new(|| {
    let zfs = Zfs::init();
    Mutex::new(zfs)
});

pub struct Zfs {}

impl Zfs {
    fn init() -> Zfs {
        let guard = zfs_handler::ZFS_HANDLER.lock().unwrap();
        drop(guard);
        Zfs {}
    }

    pub fn new_filesystem(&self, name: impl AsRef<str>) -> FileSystemBuilder {
        FileSystemBuilder::new(name)
    }

    pub fn new_volume(&self, name: impl AsRef<str>) -> VolumeBuilder {
        VolumeBuilder::new(name)
    }

    pub fn destroy_dataset(&self, name: impl AsRef<str>) -> Result<()> {
        if unsafe { sys::lzc_destroy(CString::new(name.as_ref())?.as_ptr()) } != 0 {
            return Err(DatasetError::DatasetDeleteError);
        }

        Ok(())
    }
}
