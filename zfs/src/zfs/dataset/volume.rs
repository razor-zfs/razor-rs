use std::ffi::CString;

use razor_nvpair as nvpair;
use razor_zfscore::lzc;

use nvpair::NvListAccess;

use crate::error::DatasetError;

use super::property;
use super::Result;
use super::ZfsDatasetHandle;

use lzc::zfs_prop_t::*;

#[derive(Debug)]
pub struct Volume {
    dataset: ZfsDatasetHandle,
}

impl Volume {
    pub fn destroy(self) -> Result<()> {
        lzc::destroy_dataset(self.name()).map_err(|err| err.into())
    }

    pub fn name(&self) -> String {
        self.dataset.name().to_string()
    }

    pub fn get_volume(name: impl AsRef<str>) -> Result<Self> {
        let cname = CString::new(name.as_ref())?;
        let dataset = ZfsDatasetHandle::new(cname)?;

        Ok(Self { dataset })
    }

    #[inline]
    pub fn available(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_AVAILABLE)
    }

    #[inline]
    pub fn volsize(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_VOLSIZE)
    }

    #[inline]
    pub fn volblocksize(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_VOLBLOCKSIZE)
    }

    #[inline]
    pub fn logicalused(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_LOGICALUSED)
    }

    #[inline]
    pub fn checksum(&self) -> property::CheckSumAlgo {
        self.dataset.numeric_property(ZFS_PROP_CHECKSUM).into()
    }

    #[inline]
    pub fn compression(&self) -> property::CompressionAlgo {
        self.dataset.numeric_property(ZFS_PROP_COMPRESSION).into()
    }

    #[inline]
    pub fn guid(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_GUID)
    }

    #[inline]
    pub fn creation(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_CREATION)
    }

    #[inline]
    pub fn createtxg(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_CREATETXG)
    }

    #[inline]
    pub fn compressratio(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_COMPRESSRATIO)
    }

    #[inline]
    pub fn used(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_USED)
    }

    #[inline]
    pub fn referenced(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_REFERENCED)
    }

    #[inline]
    pub fn logicalreferenced(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_LOGICALREFERENCED)
    }

    #[inline]
    pub fn objsetid(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_OBJSETID)
    }
}

#[derive(Debug)]
pub struct VolumeBuilder {
    nvlist: nvpair::NvList,
    name: String,
    volblocksize: u64,
    err: Option<DatasetError>,
}

impl VolumeBuilder {
    pub fn new(name: impl AsRef<str>) -> Self {
        let nvlist = nvpair::NvList::new(nvpair::NvFlag::UniqueName);
        let name = name.as_ref().to_string();
        let volblocksize = Self::calculate_default_volblocksize();

        Self {
            nvlist,
            name,
            volblocksize,
            err: None,
        }
    }

    // TODO: 1. default block size should be calculated
    //       2. volsize should be multiple of volblocksize and rounded to nearest 128k bytes
    //       3. add noreserve functionality
    //       4. add parents creation if needed
    //       5. add zfs_mount_and_share functionality
    pub fn create(mut self, size: u64) -> Result<Volume> {
        #[inline]
        fn _is_power_of_two(num: u64) -> bool {
            (num != 0) && ((num & (num - 1)) == 0)
        }
        dbg!("creating volume");
        let cname = CString::new(self.name.as_bytes())?;

        if let Some(err) = self.err {
            return Err(err);
        }

        self.nvlist
            .add_uint64(lzc::zfs_prop_to_name(ZFS_PROP_VOLSIZE), size)?;
        // TODO: check if volblocksize is power of 2 and between 512 and 128000
        self.nvlist.add_uint64(
            lzc::zfs_prop_to_name(ZFS_PROP_VOLBLOCKSIZE),
            self.volblocksize,
        )?;
        lzc::create_volume(&self.name, &self.nvlist)?;

        let dataset = ZfsDatasetHandle::new(cname)?;
        let volume = Volume { dataset };

        Ok(volume)
    }

    pub fn checksum(mut self, v: impl Into<property::CheckSumAlgo>) -> Self {
        let value = v.into();

        if let Err(err) = self
            .nvlist
            .add_string(lzc::zfs_prop_to_name(ZFS_PROP_CHECKSUM), value.as_str())
        {
            self.err = Some(err.into());
        }

        self
    }

    pub fn compression(mut self, v: impl Into<property::CompressionAlgo>) -> Self {
        let value = v.into();

        if let Err(err) = self
            .nvlist
            .add_string(lzc::zfs_prop_to_name(ZFS_PROP_COMPRESSION), value.as_str())
        {
            self.err = Some(err.into());
        }

        self
    }

    pub fn blocksize(mut self, v: u64) -> Self {
        self.volblocksize = v;
        self
    }

    // TODO: implement calculation algorithm
    fn calculate_default_volblocksize() -> u64 {
        8192
    }

    pub fn volmode(mut self, v: impl Into<property::VolModeId>) -> Self {
        let value = v.into();

        if let Err(err) = self
            .nvlist
            .add_uint64(lzc::zfs_prop_to_name(ZFS_PROP_VOLMODE), value.into())
        {
            self.err = Some(err.into());
        }

        self
    }
}
