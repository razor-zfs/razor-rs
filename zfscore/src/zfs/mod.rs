use super::error::DatasetError;
use super::libnvpair;
use super::sys;
use super::Result;
use dataset::Dataset;
use dataset::FilesystemIntermediate;
use dataset::Filesystem;
use property::InvalidProperty;

mod dataset;
mod dataset_builder;
mod property;
mod zfs_handler;
pub mod zfs_property;
pub mod zpool_property;

pub struct Zfs {}

impl Zfs {
    pub fn create_dataset(name: impl AsRef<str>) {}
    pub fn delete_dataset(name: impl AsRef<str>) {}
}
