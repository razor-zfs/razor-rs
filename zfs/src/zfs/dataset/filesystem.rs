use std::ffi::CString;

use razor_nvpair as nvpair;
use razor_zfscore::lzc;

use nvpair::NvListAccess;

use crate::error::DatasetError;

use super::property;
use super::Result;
use super::ZfsDatasetHandler;

#[derive(Debug)]
pub struct Filesystem {
    dataset_handler: ZfsDatasetHandler,
}

impl Filesystem {
    pub fn destroy(self) -> Result<()> {
        lzc::destroy_dataset(self.name()).map_err(|err| err.into())
    }

    pub fn name(&self) -> String {
        self.dataset_handler.get_name()
    }

    pub fn available(&self) -> u64 {
        self.dataset_handler
            .numeric_property("available", lzc::zfs_prop_t::ZFS_PROP_AVAILABLE)
    }

    pub fn atime(&self) -> property::OnOff {
        self.dataset_handler
            .numeric_property("atime", lzc::zfs_prop_t::ZFS_PROP_ATIME)
            .into()

        // let default = self.dataset_handler.get_prop_default_numeric();

        // if self.dataset_handler.check_mnt_option("atime") && default == 0 {
        //     property::OnOff::On
        // } else if self.dataset_handler.check_mnt_option("noatime") && default != 0 {
        //     property::OnOff::Off
        // } else {
        //     default.into()
        // }
    }

    pub fn logicalused(&self) -> u64 {
        self.dataset_handler
            .numeric_property("logicalused", lzc::zfs_prop_t::ZFS_PROP_LOGICALUSED)
    }

    pub fn canmount(&self) -> property::OnOffNoAuto {
        self.dataset_handler
            .numeric_property("canmount", lzc::zfs_prop_t::ZFS_PROP_CANMOUNT)
            .into()
    }

    pub fn mounted(&self) -> property::YesNo {
        self.dataset_handler.is_mounted().into()
    }

    pub fn checksum(&self) -> property::CheckSumAlgo {
        self.dataset_handler
            .numeric_property("checksum", lzc::zfs_prop_t::ZFS_PROP_CHECKSUM)
            .into()
    }

    pub fn compression(&self) -> property::CompressionAlgo {
        self.dataset_handler
            .numeric_property("compression", lzc::zfs_prop_t::ZFS_PROP_COMPRESSION)
            .into()
    }

    pub fn guid(&self) -> u64 {
        self.dataset_handler
            .numeric_property("guid", lzc::zfs_prop_t::ZFS_PROP_GUID)
    }

    pub fn creation(&self) -> u64 {
        self.dataset_handler
            .numeric_property("creation", lzc::zfs_prop_t::ZFS_PROP_CREATION)
    }

    pub fn createtxg(&self) -> u64 {
        self.dataset_handler
            .numeric_property("createtxg", lzc::zfs_prop_t::ZFS_PROP_CREATETXG)
    }

    pub fn compressratio(&self) -> u64 {
        self.dataset_handler
            .numeric_property("compressratio", lzc::zfs_prop_t::ZFS_PROP_COMPRESSRATIO)
    }

    pub fn used(&self) -> u64 {
        self.dataset_handler
            .numeric_property("used", lzc::zfs_prop_t::ZFS_PROP_USED)
    }

    pub fn referenced(&self) -> u64 {
        self.dataset_handler
            .numeric_property("referenced", lzc::zfs_prop_t::ZFS_PROP_REFERENCED)
    }

    pub fn logicalreferenced(&self) -> u64 {
        self.dataset_handler.numeric_property(
            "logicalreferenced",
            lzc::zfs_prop_t::ZFS_PROP_LOGICALREFERENCED,
        )
    }

    pub fn objsetid(&self) -> u64 {
        self.dataset_handler
            .numeric_property("objsetid", lzc::zfs_prop_t::ZFS_PROP_OBJSETID)
    }

    pub fn get_filesystem(name: impl AsRef<str>) -> Result<Self> {
        let cname = CString::new(name.as_ref())?;
        let dataset_handler = ZfsDatasetHandler::new(cname)?;

        Ok(Self { dataset_handler })
    }
}

#[derive(Debug)]
pub struct FileSystemBuilder {
    nvlist: nvpair::NvList,
    name: String,
    err: Option<DatasetError>,
}

impl FileSystemBuilder {
    pub fn new(name: impl AsRef<str>) -> Self {
        let nvlist = nvpair::NvList::new(nvpair::NvFlag::UniqueName);
        Self {
            nvlist,
            name: name.as_ref().to_string(),
            err: None,
        }
    }

    pub fn atime(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();
        if let Err(err) = self.nvlist.add_string("atime", value.as_str()) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn canmount(mut self, v: impl Into<property::OnOffNoAuto>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_string("canmount", value.as_str()) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn checksum(mut self, v: impl Into<property::CheckSumAlgo>) -> Self {
        let value = v.into();
        if let Err(err) = self.nvlist.add_string("checksum", value.as_str()) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn devices(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_string("devices", value.as_str()) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn nbmand(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();
        if let Err(err) = self.nvlist.add_string("nbmand", value.as_str()) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn overlay(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_string("overlay", value.as_str()) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn readonly(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_string("readonly", value.as_str()) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn relatime(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_string("relatime", value.as_str()) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn setuid(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_string("setuid", value.as_str()) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn vscan(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_string("vscan", value.as_str()) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn zoned(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_string("zoned", value.as_str()) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn compression(mut self, v: impl Into<property::CompressionAlgo>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_string("compression", value.as_str()) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn exec(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_string("exec", value.as_str()) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn create(self) -> Result<Filesystem> {
        let cname = CString::new(self.name.as_bytes())?;
        match self.err {
            Some(err) => Err(err),
            None => {
                lzc::create_filesystem(&self.name, &self.nvlist)?;
                let dataset_handler = ZfsDatasetHandler::new(cname)?;
                let filesystem: Filesystem = Filesystem { dataset_handler };

                Ok(filesystem)
            }
        }
    }
}
