use super::libnvpair;
use super::zfs_property;
use super::Result;
use serde_nvpair::from_nvlist;

use serde::Deserialize;

mod bookmark;
mod filesystem;
mod snapshot;
mod volume;

/*#[derive(Debug)]
pub enum DatasetType {
    Filesystem(filesystem::Filesystem),
    Volume(volume::Volume),
    Snapshot(snapshot::Snapshot),
    Bookmark(bookmark::Bookmark),
}*/

#[derive(Debug)]
pub struct Dataset {}

impl Dataset {
    pub fn builder() -> Result<DatasetBuilder> {
        DatasetBuilder::default()
    }
}

pub struct DatasetBuilder {
    nvlist: libnvpair::NvList,
}

impl DatasetBuilder {
    pub fn default() -> Result<Self> {
        Ok(DatasetBuilder {
            nvlist: libnvpair::NvList::nvlist_alloc(libnvpair::NvFlag::UniqueName)?,
        })
    }

    pub fn canmount(mut self, v: zfs_property::OnOffNoAuto) -> Result<Self> {
        Ok(self)
    }

    pub fn atime(mut self, v: zfs_property::OnOff) -> Result<Self> {
        Ok(self)
    }

    pub fn checksum(mut self, v: zfs_property::CheckSumAlgo) -> Result<Self> {
        Ok(self)
    }

    pub fn compression_ratio(mut self, v: zfs_property::CompressionAlgo) -> Result<Self> {
        Ok(self)
    }

    pub fn copies(mut self, v: u8) -> Result<Self> {
        Ok(self)
    }

    pub fn exec(mut self, v: zfs_property::OnOff) -> Result<Self> {
        Ok(self)
    }

    pub fn create_filesystem(mut self) -> Result<filesystem::Filesystem> {
        Ok(from_nvlist(&mut self.nvlist)?)
    }

    pub fn create_snapshot(mut self) -> Result<snapshot::Snapshot> {
        Ok(from_nvlist(&mut self.nvlist)?)
    }

    pub fn create_bookmark(mut self) -> Result<bookmark::Bookmark> {
        Ok(from_nvlist(&mut self.nvlist)?)
    }

    pub fn create_volume(mut self, size: u64) -> Result<volume::Volume> {
        Ok(from_nvlist(&mut self.nvlist)?)
    }
}

#[derive(Debug, Deserialize)]
struct CommonProperties {
    guid: zfs_property::Guid,
    creation: zfs_property::Creation,
    createtxg: zfs_property::CreateTxg,
    compressratio: zfs_property::CompressRatio,
    used: zfs_property::Used,
    referenced: zfs_property::Referenced,
    logicalreferenced: zfs_property::LogicalReferenced,
    objsetid: zfs_property::ObjSetId,
}
