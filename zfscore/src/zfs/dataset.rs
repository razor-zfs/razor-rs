use std::ffi::CString;

use crate::dataset::filesystem::FilesystemIntermediate;

use super::libnvpair;
pub(crate) use super::sys;
use super::zfs_property;
use super::DatasetError;
use super::Result;
use super::Zfs;
use serde_nvpair::from_nvlist;

pub use filesystem::Filesystem;
pub use volume::Volume;

use serde::{Deserialize, Serialize};

mod bookmark;
mod filesystem;
mod snapshot;
mod volume;

#[derive(Debug)]
pub struct Dataset {
    zfs_handle: Zfs,
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
            zfs_handle: Zfs::init(),
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

    // TODO: add zfs_mount_and_share functionality
    pub fn create_filesystem(mut self) -> Result<filesystem::Filesystem> {
        let ret = unsafe {
            sys::lzc_create(
                CString::new(self.name.clone())?.as_ptr(),
                sys::lzc_dataset_type::LZC_DATSET_TYPE_ZFS,
                self.nvlist.raw,
                std::ptr::null_mut(),
                0,
            )
        };

        dbg!(ret);

        if ret != 0 {
            return Err(DatasetError::DatasetCreationFailure);
        }

        self.zfs_handle.create_dataset_handle(&self.name)?;

        let interfs: FilesystemIntermediate =
            from_nvlist(&mut self.zfs_handle.get_dataset_nvlist()?)?;

        Ok(interfs.convert_to_valid(&self.zfs_handle, &self.name)?)
    }

    pub fn create_snapshot(mut self) -> Result<snapshot::Snapshot> {
        Ok(from_nvlist(&mut self.nvlist)?)
    }

    pub fn create_bookmark(mut self) -> Result<bookmark::Bookmark> {
        Ok(from_nvlist(&mut self.nvlist)?)
    }

    // TODO: 1. default block size should be calculated
    //       2. volsize should be multiple of volblocksize and rounded to nearest 128k bytes
    //       3. add noreserve functionality
    //       4. add parents creation if needed
    //       5. add zfs_mount_and_share functionality
    pub fn create_volume(mut self, size: u64) -> Result<volume::Volume> {
        #[inline]
        fn is_power_of_two(num: u64) -> bool {
            (num != 0) && ((num & (num - 1)) == 0)
        }

        self.nvlist.add_uint64("volsize", size)?;

        if let Some(block_size) = self.volblocksize {
            if (block_size > 512 || block_size < 128000) && is_power_of_two(block_size) {
                self.nvlist.add_uint64("volblocksize", block_size)?;
            } else {
                return Err(DatasetError::BadVolumeBlockSize);
            }
        } else {
            self.nvlist.add_uint64("volblocksize", 8192)?;
        }

        if unsafe {
            sys::lzc_create(
                CString::new(self.name)?.as_ptr(),
                sys::lzc_dataset_type::LZC_DATSET_TYPE_ZVOL,
                self.nvlist.raw,
                std::ptr::null_mut(),
                0,
            )
        } != 0
        {
            return Err(DatasetError::DatasetCreationFailure);
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
