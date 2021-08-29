pub use dataset::Bookmark;
pub use dataset::Filesystem;
pub use dataset::Snapshot;
pub use dataset::Volume;
use libnvpair::NvList;

use std::ffi::CString;

use super::libnvpair;
use super::sys;
use super::Result;
use crate::error::DatasetError;

use crate::zfs::zfs_handler::ZFS_HANDLER;
use dataset::FileSystemBuilder;
use dataset::VolumeBuilder;
use once_cell::sync::Lazy;
use serde_nvpair::from_nvlist;
use std::sync::Mutex;

pub mod property;

mod dataset;
mod zfs_handler;

pub(crate) static ZFS: Lazy<Mutex<Zfs>> = Lazy::new(|| {
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

    pub fn get_filesystem(name: impl AsRef<str>) -> Result<Filesystem> {
        let mut nvl = ZFS.lock().unwrap().get(name.as_ref())?;
        let cname = CString::new(name.as_ref())?;

        from_nvlist(&mut nvl)
            .map(|fs| Filesystem {
                name: property::Name::new(cname),
                ..fs
            })
            .map_err(|err| err.into())
    }

    pub fn get_volume(name: impl AsRef<str>) -> Result<Volume> {
        let mut nvl = ZFS.lock().unwrap().get(name.as_ref())?;
        let cname = CString::new(name.as_ref())?;

        from_nvlist(&mut nvl)
            .map(|vol| Volume {
                name: property::Name::new(cname),
                ..vol
            })
            .map_err(|err| err.into())
    }

    fn get(&self, name: impl AsRef<str>) -> Result<NvList> {
        let cname = CString::new(name.as_ref())?;
        let zfs_handle = unsafe {
            sys::make_dataset_handle(ZFS_HANDLER.lock().unwrap().handler(), cname.as_ptr())
        };

        let nvl = unsafe {
            libnvpair::NvList {
                raw: (*zfs_handle).zfs_props,
            }
        };

        Ok(nvl)
    }

    fn destroy(&self, name: impl AsRef<str>) -> Result<()> {
        if unsafe { sys::lzc_destroy(CString::new(name.as_ref())?.as_ptr()) } != 0 {
            return Err(DatasetError::DatasetDeleteError);
        }

        Ok(())
    }
}
