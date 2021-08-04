use super::libnvpair;
use super::zfs_property;
use super::Result;
use serde_nvpair::from_nvlist;

use serde::Deserialize;

mod bookmark;
mod filesystem;
mod snapshot;
mod volume;

#[derive(Debug)]
pub struct Dataset {
    nvlist: libnvpair::NvList,
    name: String,
}

impl Dataset {
    pub fn new<T>(name: T) -> Result<Self>
    where
        T: AsRef<str>,
    {
        Ok(Dataset {
            nvlist: libnvpair::NvList::nvlist_alloc(libnvpair::NvFlag::UniqueName)?,
            name: name.as_ref().to_string(),
        })
    }

    pub fn canmount(mut self, v: zfs_property::OnOffNoAuto) -> Result<Self> {
        self.nvlist.add_string("canmount", v.as_str())?;
        Ok(self)
    }

    pub fn atime(mut self, v: zfs_property::OnOff) -> Result<Self> {
        self.nvlist.add_string("atime", v.as_str())?;
        Ok(self)
    }

    pub fn checksum(mut self, v: zfs_property::CheckSumAlgo) -> Result<Self> {
        self.nvlist.add_string("checksum", v.as_str())?;
        Ok(self)
    }

    pub fn compression(mut self, v: zfs_property::CompressionAlgo) -> Result<Self> {
        self.nvlist.add_string("compression", v.as_str())?;
        Ok(self)
    }

    pub fn copies(mut self, v: u8) -> Result<Self> {
        self.nvlist.add_string("copies", v.to_string().as_str())?;
        Ok(self)
    }

    pub fn exec(mut self, v: zfs_property::OnOff) -> Result<Self> {
        self.nvlist.add_string("exec", v.as_str())?;
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
        self.nvlist
            .add_string("volblocksize", size.to_string().as_str())?;
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
