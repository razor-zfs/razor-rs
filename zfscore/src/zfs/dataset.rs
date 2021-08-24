use std::ffi::CString;

pub(crate) use filesystem::FileSystemBuilder;
pub(crate) use volume::VolumeBuilder;

use super::libnvpair;
use super::sys;
use super::zfs_property;
use super::DatasetError;
use super::Result;
use serde_nvpair::from_nvlist;

use serde::{Deserialize, Serialize};

mod bookmark;
mod filesystem;
mod snapshot;
mod volume;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum DatasetType {
    Filesystem(filesystem::Filesystem),
    Volume(volume::Volume),
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Dataset {
    name: String,
    dataset: DatasetType,
}

impl Dataset {
    pub fn destroy(self) -> Result<()> {
        if unsafe { sys::lzc_destroy(CString::new(self.name)?.as_ptr()) } != 0 {
            return Err(DatasetError::DatasetDeleteError);
        }

        Ok(())
    }
}
