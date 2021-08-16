use super::libnvpair;
use super::sys;
use super::Dataset;
use super::FilesystemIntermediate;
use super::Result;

use filesystem_builder::FileSystemBuilder;
use volume_builder::VolumeBuilder;

mod filesystem_builder;
mod volume_builder;
pub struct DatasetBuilder {}

impl DatasetBuilder {
    pub fn create_filesystem(name: impl AsRef<str>) -> FileSystemBuilder {
        FileSystemBuilder::new(name)
    }
    pub fn create_volume(name: impl AsRef<str>) -> VolumeBuilder {
        VolumeBuilder::new(name)
    }
}
