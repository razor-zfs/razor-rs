pub use dataset::Bookmark;
pub use dataset::Filesystem;
pub use dataset::Snapshot;
pub use dataset::Volume;

use std::ffi::CString;

use super::core;
use super::libnvpair;
use super::sys;
use super::Result;
use crate::error::DatasetError;

use dataset::FileSystemBuilder;
use dataset::VolumeBuilder;
use serde_nvpair::from_nvlist;

pub mod property;

mod dataset;
mod zfs_handler;

#[derive(Debug)]
pub struct Zfs {}

impl Zfs {
    pub fn filesystem(name: impl AsRef<str>) -> FileSystemBuilder {
        FileSystemBuilder::new(name)
    }

    pub fn volume(name: impl AsRef<str>) -> VolumeBuilder {
        VolumeBuilder::new(name)
    }

    pub fn destroy_dataset(name: impl AsRef<str>) -> Result<()> {
        let cname = CString::new(name.as_ref())?;
        core::destroy_dataset(cname).map_err(|err| err.into())
    }

    pub fn get_filesystem(name: impl AsRef<str>) -> Result<Filesystem> {
        let cname = CString::new(name.as_ref())?;

        let mut nvl = core::get_dataset_nvlist(name)?;

        from_nvlist(&mut nvl)
            .map(|fs| Filesystem {
                name: property::Name::new(cname),
                ..fs
            })
            .map_err(|err| err.into())
    }

    pub fn get_volume(name: impl AsRef<str>) -> Result<Volume> {
        let cname = CString::new(name.as_ref())?;
        let mut nvl = core::get_dataset_nvlist(name)?;

        from_nvlist(&mut nvl)
            .map(|vol| Volume {
                name: property::Name::new(cname),
                ..vol
            })
            .map_err(|err| err.into())
    }
}
