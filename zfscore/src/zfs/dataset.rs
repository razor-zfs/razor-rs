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
    volblocksize: Option<u64>,
}

impl Dataset {
    pub fn new<T>(name: T) -> Result<Self>
    where
        T: AsRef<str>,
    {
        Ok(Dataset {
            nvlist: libnvpair::NvList::nvlist_alloc(libnvpair::NvFlag::UniqueName)?,
            name: name.as_ref().to_string(),
            volblocksize: None,
        })
    }

    pub fn canmount(mut self, v: impl Into<zfs_property::OnOffNoAuto>) -> Result<Self> {
        self.nvlist.add_string("canmount", v.into().as_str())?;
        Ok(self)
    }

    pub fn atime(mut self, v: impl Into<zfs_property::OnOff>) -> Result<Self> {
        self.nvlist.add_string("atime", v.into().as_str())?;
        Ok(self)
    }

    pub fn checksum(mut self, v: impl Into<zfs_property::CheckSumAlgo>) -> Result<Self> {
        self.nvlist.add_string("checksum", v.into().as_str())?;
        Ok(self)
    }

    pub fn devices(mut self, v: impl Into<zfs_property::OnOff>) -> Result<Self> {
        self.nvlist.add_string("devices", v.into().as_str())?;
        Ok(self)
    }

    pub fn nbmand(mut self, v: impl Into<zfs_property::OnOff>) -> Result<Self> {
        self.nvlist.add_string("nbmand", v.into().as_str())?;
        Ok(self)
    }

    pub fn overlay(mut self, v: impl Into<zfs_property::OnOff>) -> Result<Self> {
        self.nvlist.add_string("overlay", v.into().as_str())?;
        Ok(self)
    }

    pub fn readonly(mut self, v: impl Into<zfs_property::OnOff>) -> Result<Self> {
        self.nvlist.add_string("readonly", v.into().as_str())?;
        Ok(self)
    }

    pub fn realtime(mut self, v: impl Into<zfs_property::OnOff>) -> Result<Self> {
        self.nvlist.add_string("realtime", v.into().as_str())?;
        Ok(self)
    }

    pub fn setuid(mut self, v: impl Into<zfs_property::OnOff>) -> Result<Self> {
        self.nvlist.add_string("setuid", v.into().as_str())?;
        Ok(self)
    }

    pub fn utf8only(mut self, v: impl Into<zfs_property::OnOff>) -> Result<Self> {
        self.nvlist.add_string("utf8only", v.into().as_str())?;
        Ok(self)
    }

    pub fn vscan(mut self, v: impl Into<zfs_property::OnOff>) -> Result<Self> {
        self.nvlist.add_string("vscan", v.into().as_str())?;
        Ok(self)
    }

    pub fn zoned(mut self, v: impl Into<zfs_property::OnOff>) -> Result<Self> {
        self.nvlist.add_string("zoned", v.into().as_str())?;
        Ok(self)
    }

    pub fn compression(mut self, v: impl Into<zfs_property::CompressionAlgo>) -> Result<Self> {
        self.nvlist.add_string("compression", v.into().as_str())?;
        Ok(self)
    }

    pub fn exec(mut self, v: impl Into<zfs_property::OnOff>) -> Result<Self> {
        self.nvlist.add_string("exec", v.into().as_str())?;
        Ok(self)
    }

    pub fn blocksize(mut self, v: u64) -> Result<Self> {
        self.volblocksize = Some(v);
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
        self.nvlist.add_uint64("volsize", size)?;

        if let Some(block_size) = self.volblocksize {
            self.nvlist.add_uint64("volblocksize", block_size)?;
        }

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
