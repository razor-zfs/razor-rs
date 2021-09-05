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
pub struct Volume {
    dataset_handler: ZfsDatasetHandler,
}

impl Volume {
    pub fn destroy(self) -> Result<()> {
        core::destroy_dataset(self.name()).map_err(|err| err.into())
    }

    pub fn name(&self) -> String {
        self.dataset_handler.get_name()
    }

    pub fn get_volume(name: impl AsRef<str>) -> Result<Self> {
        let cname = CString::new(name.as_ref())?;
        let dataset_handler = ZfsDatasetHandler::new(cname)?;

        Ok(Self { dataset_handler })
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

    pub fn volsize(&self) -> u64 {
        let prop = self.dataset_handler.search_property("volsize");
        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val,
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_VOLSIZE)
        }
    }

    pub fn volblocksize(&self) -> u64 {
        let prop = self.dataset_handler.search_property("volblocksize");
        if let Ok(prop) = prop {
            match prop {
                libnvpair::Value::U64(val) => val,
                _ => todo!(),
            }
        } else {
            self.dataset_handler
                .get_prop_default_numeric(zfs_prop_t::ZFS_PROP_VOLBLOCKSIZE)
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
}

#[derive(Debug)]
pub struct VolumeBuilder {
    nvlist: libnvpair::NvList,
    name: String,
    volblocksize: u64,
    err: Option<DatasetError>,
}

impl VolumeBuilder {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            nvlist: libnvpair::NvList::new(libnvpair::NvFlag::UniqueName),
            name: name.as_ref().to_string(),
            volblocksize: Self::calculate_default_volblocksize(),
            err: None,
        }
    }

    pub fn checksum(mut self, v: impl Into<property::CheckSumAlgo>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_string("checksum", value.as_str()) {
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

        if let Err(err) = self.nvlist.add_uint64("volmode", value.into()) {
            self.err = Some(err.into());
        }

        self
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
        match self.err {
            Some(err) => Err(err),
            None => {
                self.nvlist.add_uint64("volsize", size)?;

                // TODO: check if volblocksize is power of 2 and between 512 and 128000
                self.nvlist.add_uint64("volblocksize", self.volblocksize)?;

                core::create_volume(&self.name, &self.nvlist)?;
                let dataset_handler = ZfsDatasetHandler::new(cname)?;

                let volume: Volume = Volume { dataset_handler };

                Ok(volume)
            }
        }
    }
}
