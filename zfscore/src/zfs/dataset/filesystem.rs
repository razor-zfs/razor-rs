use super::sys;
use super::*;
use crate::zfs::zfs_handler::ZFS_HANDLER;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Filesystem {
    #[serde(default)]
    name: property::Name,
    available: property::Available,
    atime: Option<property::Atime>,
    logicalused: property::LogicalUsed,
    canmount: Option<property::CanMount>,
    mounted: Option<property::Mounted>,
    checksum: Option<property::CheckSum>,
    compression: Option<property::Compression>,
    guid: property::Guid,
    creation: property::Creation,
    createtxg: property::CreateTxg,
    compressratio: property::CompressRatio,
    used: property::Used,
    referenced: property::Referenced,
    logicalreferenced: property::LogicalReferenced,
    objsetid: property::ObjSetId,
}

impl Filesystem {
    pub fn destroy(self) -> Result<()> {
        if unsafe { sys::lzc_destroy(self.name.value().as_ptr()) } != 0 {
            return Err(DatasetError::DatasetDeleteError);
        }

        Ok(())
    }

    pub fn available(&self) -> u64 {
        self.available.value()
    }

    pub fn atime(&self) -> property::OnOff {
        self.atime.map_or_else(
            || property::Atime::default(self.name.value()).value(),
            |atime| atime.value(),
        )
    }

    pub fn logicalused(&self) -> u64 {
        self.logicalused.value()
    }

    pub fn canmount(&self) -> property::OnOffNoAuto {
        match self.canmount {
            Some(canmount) => canmount.value(),
            None => property::CanMount::default().value(),
        }
    }

    pub fn mounted(&self) -> property::YesNo {
        self.mounted.map_or_else(
            || property::Mounted::default(self.name.value()).value(),
            |mounted| mounted.value(),
        )
    }

    pub fn checksum(&self) -> property::CheckSumAlgo {
        self.checksum.map_or_else(
            || property::CheckSum::default().value(),
            |checksum| checksum.value(),
        )
    }

    pub fn compression(&self) -> property::CompressionAlgo {
        self.compression.map_or_else(
            || property::Compression::default().value(),
            |compression| compression.value(),
        )
    }

    pub fn guid(&self) -> u64 {
        self.guid.value()
    }

    pub fn creation(&self) -> u64 {
        self.creation.value()
    }

    pub fn createtxg(&self) -> u64 {
        self.createtxg.value()
    }

    pub fn compressratio(&self) -> u64 {
        self.compressratio.value()
    }

    pub fn used(&self) -> u64 {
        self.used.value()
    }

    pub fn referenced(&self) -> u64 {
        self.referenced.value()
    }

    pub fn logicalreferenced(&self) -> u64 {
        self.logicalreferenced.value()
    }

    pub fn objsetid(&self) -> u64 {
        self.objsetid.value()
    }
}

#[derive(Debug)]
pub struct FileSystemBuilder {
    nvlist: Result<libnvpair::NvList>,
    name: String,
    //err: Option<DatasetError>,
}

impl FileSystemBuilder {
    pub fn new(name: impl AsRef<str>) -> Self {
        FileSystemBuilder {
            nvlist: libnvpair::NvList::nvlist_alloc(libnvpair::NvFlag::UniqueName)
                .map_err(|err| err.into()),
            name: name.as_ref().to_string(),
        }
    }

    pub fn atime(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_string("atime", value.as_str()) {
                self.nvlist = Err(err.into());
            }
        }

        self
    }

    pub fn canmount(mut self, v: impl Into<property::OnOffNoAuto>) -> Self {
        let value = v.into();
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_string("canmount", value.as_str()) {
                self.nvlist = Err(err.into());
            }
        }

        self
    }

    pub fn checksum(mut self, v: impl Into<property::CheckSumAlgo>) -> Self {
        let value = v.into();
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_string("checksum", value.as_str()) {
                self.nvlist = Err(err.into());
            }
        }

        self
    }

    pub fn devices(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_string("devices", value.as_str()) {
                self.nvlist = Err(err.into());
            }
        }

        self
    }

    pub fn nbmand(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_string("nbmand", value.as_str()) {
                self.nvlist = Err(err.into());
            }
        }

        self
    }

    pub fn overlay(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_string("overlay", value.as_str()) {
                self.nvlist = Err(err.into());
            }
        }

        self
    }

    pub fn readonly(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_string("readonly", value.as_str()) {
                self.nvlist = Err(err.into());
            }
        }

        self
    }

    pub fn relatime(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_string("relatime", value.as_str()) {
                self.nvlist = Err(err.into());
            }
        }

        self
    }

    pub fn setuid(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_string("setuid", value.as_str()) {
                self.nvlist = Err(err.into());
            }
        }

        self
    }

    pub fn vscan(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_string("vscan", value.as_str()) {
                self.nvlist = Err(err.into());
            }
        }

        self
    }

    pub fn zoned(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_string("zoned", value.as_str()) {
                self.nvlist = Err(err.into());
            }
        }

        self
    }

    pub fn compression(mut self, v: impl Into<property::CompressionAlgo>) -> Self {
        let value = v.into();
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_string("compression", value.as_str()) {
                self.nvlist = Err(err.into());
            }
        }

        self
    }

    pub fn exec(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_string("exec", value.as_str()) {
                self.nvlist = Err(err.into());
            }
        }

        self
    }

    pub fn create(mut self) -> Result<Filesystem> {
        let cname = CString::new(self.name.as_bytes())?;
        match self.nvlist.as_mut() {
            Ok(nvlist) => {
                let ret = unsafe {
                    sys::lzc_create(
                        cname.as_ptr(),
                        sys::lzc_dataset_type::LZC_DATSET_TYPE_ZFS,
                        nvlist.raw,
                        std::ptr::null_mut(),
                        0,
                    )
                };
                dbg!(ret);

                if ret != 0 {
                    return Err(DatasetError::DatasetCreationFailure);
                }

                let zfs_handle = unsafe {
                    sys::make_dataset_handle(ZFS_HANDLER.lock().unwrap().handler(), cname.as_ptr())
                };

                let mut nvl = unsafe {
                    libnvpair::NvList {
                        raw: (*zfs_handle).zfs_props,
                    }
                };

                let filesystem: Filesystem = from_nvlist(&mut nvl).map(|fs| Filesystem {
                    name: property::Name::new(cname),
                    ..fs
                })?;

                Ok(filesystem)
            }
            Err(err) => Err(err.clone()), // TODO: check this line because it clones here
        }
    }
}
