use std::ffi::CString;

use libnvpair::NvListAccess;

use crate::error::DatasetError;

use super::core;
use super::libnvpair;
use super::property;
use super::zfs_prop_t;
use super::Result;
use super::ZfsDatasetHandler;

#[derive(Debug)]
pub struct Filesystem {
    dataset_handler: ZfsDatasetHandler,
}

impl Filesystem {
    pub fn destroy(self) -> Result<()> {
        core::destroy_dataset(self.name()).map_err(|err| err.into())
    }

    pub fn name(&self) -> String {
        self.dataset_handler.get_name()
    }

    pub fn available(&self) -> u64 {
        let prop = self.dataset_handler.search_property("available");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val,
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_AVAILABLE)
        }
    }

    pub fn atime(&self) -> property::OnOff {
        let prop = self.dataset_handler.search_property("atime");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val.into(),
                _ => todo!(),
            }
        } else {
            let default = self
                .dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_ATIME);

            if self.dataset_handler.check_mnt_option("atime") && default == 0 {
                property::OnOff::On
            } else if self.dataset_handler.check_mnt_option("noatime") && default != 0 {
                property::OnOff::Off
            } else {
                default.into()
            }
        }
    }

    pub fn logicalused(&self) -> u64 {
        let prop = self.dataset_handler.search_property("logicalused");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val,
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_LOGICALUSED)
        }
    }

    pub fn canmount(&self) -> property::OnOffNoAuto {
        let prop = self.dataset_handler.search_property("canmount");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val.into(),
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_CANMOUNT)
                .into()
        }
    }

    pub fn mounted(&self) -> property::YesNo {
        let prop = self.dataset_handler.search_property("mounted");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val.into(),
                _ => todo!(),
            }
        } else {
            self.dataset_handler.is_mounted().into()
        }
    }

    pub fn checksum(&self) -> property::CheckSumAlgo {
        let prop = self.dataset_handler.search_property("checksum");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val.into(),
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_CHECKSUM)
                .into()
        }
    }

    pub fn compression(&self) -> property::CompressionAlgo {
        let prop = self.dataset_handler.search_property("compression");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val.into(),
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_COMPRESSION)
                .into()
        }
    }

    pub fn guid(&self) -> u64 {
        let prop = self.dataset_handler.search_property("guid");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val,
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_GUID)
        }
    }

    pub fn creation(&self) -> u64 {
        let prop = self.dataset_handler.search_property("creation");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val,
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_CREATION)
        }
    }

    pub fn createtxg(&self) -> u64 {
        let prop = self.dataset_handler.search_property("createtxg");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val,
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_CREATETXG)
        }
    }

    pub fn compressratio(&self) -> u64 {
        let prop = self.dataset_handler.search_property("compressratio");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val,
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_COMPRESSRATIO)
        }
    }

    pub fn used(&self) -> u64 {
        let prop = self.dataset_handler.search_property("used");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val,
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_USED)
        }
    }

    pub fn referenced(&self) -> u64 {
        let prop = self.dataset_handler.search_property("referenced");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val,
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_REFERENCED)
        }
    }

    pub fn logicalreferenced(&self) -> u64 {
        let prop = self.dataset_handler.search_property("logicalreferenced");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val,
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_LOGICALREFERENCED)
        }
    }

    pub fn objsetid(&self) -> u64 {
        let prop = self.dataset_handler.search_property("objsetid");

        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val,
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_OBJSETID)
        }
    }

    pub fn get_filesystem(name: impl AsRef<str>) -> Result<Self> {
        let cname = CString::new(name.as_ref())?;
        let dataset_handler = ZfsDatasetHandler::new(cname)?;

        Ok(Self { dataset_handler })
    }
}

#[derive(Debug)]
pub struct FileSystemBuilder {
    nvlist: libnvpair::NvList,
    name: String,
    err: Option<DatasetError>,
}

impl FileSystemBuilder {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            nvlist: libnvpair::NvList::new(libnvpair::NvFlag::UniqueName),
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
                core::create_filesystem(&self.name, &self.nvlist)?;
                let dataset_handler = ZfsDatasetHandler::new(cname)?;
                let filesystem: Filesystem = Filesystem { dataset_handler };

                Ok(filesystem)
            }
        }
    }
}
