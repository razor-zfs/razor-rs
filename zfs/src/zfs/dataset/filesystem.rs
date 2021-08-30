use super::core;
use super::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Filesystem {
    #[serde(default)]
    pub(crate) name: property::Name,
    pub(crate) available: property::Available,
    pub(crate) atime: Option<property::Atime>,
    pub(crate) logicalused: property::LogicalUsed,
    pub(crate) canmount: Option<property::CanMount>,
    pub(crate) mounted: Option<property::Mounted>,
    pub(crate) checksum: Option<property::CheckSum>,
    pub(crate) compression: Option<property::Compression>,
    pub(crate) guid: property::Guid,
    pub(crate) creation: property::Creation,
    pub(crate) createtxg: property::CreateTxg,
    pub(crate) compressratio: property::CompressRatio,
    pub(crate) used: property::Used,
    pub(crate) referenced: property::Referenced,
    pub(crate) logicalreferenced: property::LogicalReferenced,
    pub(crate) objsetid: property::ObjSetId,
}

impl Filesystem {
    pub fn destroy(self) -> Result<()> {
        core::destroy_dataset(self.name.value().to_string_lossy()).map_err(|err| err.into())
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
        Self {
            nvlist: libnvpair::NvList::new(libnvpair::NvFlag::UniqueName).map_err(|err| err.into()),
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
                let mut nvl = core::create_filesystem(self.name, nvlist)?;

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
