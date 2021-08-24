use super::sys;
use super::*;
use crate::zfs::zfs_handler::ZFS_HANDLER;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Filesystem {
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
    pub fn available(&self) -> u64 {
        self.available.value()
    }

    pub fn atime(&self, name: impl AsRef<str>) -> Result<property::OnOff> {
        self.atime.map_or_else(
            || Ok(property::Atime::default(name)?.value()),
            |atime| Ok(atime.value()),
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

    pub fn mounted(&self, name: impl AsRef<str>) -> Result<property::YesNo> {
        self.mounted.map_or_else(
            || Ok(property::Mounted::default(name)?.value()),
            |mounted| Ok(mounted.value()),
        )
    }

    pub fn checksum(&self) -> Result<property::CheckSum> {
        self.checksum
            .map_or_else(|| property::CheckSum::default(), Ok)
    }

    pub fn compression(&self) -> Result<property::Compression> {
        self.compression
            .map_or_else(|| property::Compression::default(), Ok)
    }

    pub fn guid(&self) -> property::Guid {
        self.guid
    }

    pub fn creation(&self) -> property::Creation {
        self.creation
    }

    pub fn createtxg(&self) -> property::CreateTxg {
        self.createtxg
    }

    pub fn compressratio(&self) -> property::CompressRatio {
        self.compressratio
    }

    pub fn used(&self) -> property::Used {
        self.used
    }

    pub fn referenced(&self) -> property::Referenced {
        self.referenced
    }

    pub fn logicalreferenced(&self) -> property::LogicalReferenced {
        self.logicalreferenced
    }

    pub fn objsetid(&self) -> property::ObjSetId {
        self.objsetid
    }
}

#[derive(Debug)]
pub struct FileSystemBuilder {
    nvlist: Option<libnvpair::NvList>,
    name: String,
    err: Option<DatasetError>,
}

impl FileSystemBuilder {
    pub fn new(name: impl AsRef<str>) -> Self {
        match libnvpair::NvList::nvlist_alloc(libnvpair::NvFlag::UniqueName) {
            Ok(nvlist) => FileSystemBuilder {
                nvlist: Some(nvlist),
                name: name.as_ref().to_string(),
                err: None,
            },
            Err(error) => FileSystemBuilder {
                nvlist: None,
                name: name.as_ref().to_string(),
                err: Some(error.into()),
            },
        }
    }

    pub fn atime(mut self, v: impl Into<property::OnOff>) -> Self {
        if self.err.is_some() {
            return self;
        }

        let value = v.into();
        if let Some(nvlist) = self.nvlist.as_mut() {
            self.err = nvlist
                .add_string("atime", value.as_str())
                .map_err(Into::into)
                .err();
        }

        self
    }

    pub fn canmount(mut self, v: impl Into<property::OnOffNoAuto>) -> Self {
        if self.err.is_some() {
            return self;
        }

        let value = v.into();
        if let Some(nvlist) = self.nvlist.as_mut() {
            self.err = nvlist
                .add_string("canmount", value.as_str())
                .map_err(Into::into)
                .err();
        }

        self
    }

    pub fn checksum(mut self, v: impl Into<property::CheckSumAlgo>) -> Self {
        if self.err.is_some() {
            return self;
        }

        let value = v.into();
        if let Some(nvlist) = self.nvlist.as_mut() {
            self.err = nvlist
                .add_string("checksum", value.as_str())
                .map_err(Into::into)
                .err();
        }

        self
    }

    pub fn devices(mut self, v: impl Into<property::OnOff>) -> Self {
        if self.err.is_some() {
            return self;
        }

        let value = v.into();
        if let Some(nvlist) = self.nvlist.as_mut() {
            self.err = nvlist
                .add_string("devices", value.as_str())
                .map_err(Into::into)
                .err();
        }

        self
    }

    pub fn nbmand(mut self, v: impl Into<property::OnOff>) -> Self {
        if self.err.is_some() {
            return self;
        }

        let value = v.into();
        if let Some(nvlist) = self.nvlist.as_mut() {
            self.err = nvlist
                .add_string("nbmand", value.as_str())
                .map_err(Into::into)
                .err();
        }

        self
    }

    pub fn overlay(mut self, v: impl Into<property::OnOff>) -> Self {
        if self.err.is_some() {
            return self;
        }

        let value = v.into();
        if let Some(nvlist) = self.nvlist.as_mut() {
            self.err = nvlist
                .add_string("overlay", value.as_str())
                .map_err(Into::into)
                .err();
        }

        self
    }

    pub fn readonly(mut self, v: impl Into<property::OnOff>) -> Self {
        if self.err.is_some() {
            return self;
        }

        let value = v.into();
        if let Some(nvlist) = self.nvlist.as_mut() {
            self.err = nvlist
                .add_string("readonly", value.as_str())
                .map_err(Into::into)
                .err();
        }

        self
    }

    pub fn relatime(mut self, v: impl Into<property::OnOff>) -> Self {
        if self.err.is_some() {
            return self;
        }

        let value = v.into();
        if let Some(nvlist) = self.nvlist.as_mut() {
            self.err = nvlist
                .add_string("relatime", value.as_str())
                .map_err(Into::into)
                .err();
        }

        self
    }

    pub fn setuid(mut self, v: impl Into<property::OnOff>) -> Self {
        if self.err.is_some() {
            return self;
        }

        let value = v.into();
        if let Some(nvlist) = self.nvlist.as_mut() {
            self.err = nvlist
                .add_string("setuid", value.as_str())
                .map_err(Into::into)
                .err();
        }

        self
    }

    pub fn vscan(mut self, v: impl Into<property::OnOff>) -> Self {
        if self.err.is_some() {
            return self;
        }

        let value = v.into();
        if let Some(nvlist) = self.nvlist.as_mut() {
            self.err = nvlist
                .add_string("vscan", value.as_str())
                .map_err(Into::into)
                .err();
        }

        self
    }

    pub fn zoned(mut self, v: impl Into<property::OnOff>) -> Self {
        if self.err.is_some() {
            return self;
        }

        let value = v.into();
        if let Some(nvlist) = self.nvlist.as_mut() {
            self.err = nvlist
                .add_string("zoned", value.as_str())
                .map_err(Into::into)
                .err();
        }

        self
    }

    pub fn compression(mut self, v: impl Into<property::CompressionAlgo>) -> Self {
        if self.err.is_some() {
            return self;
        }

        let value = v.into();
        if let Some(nvlist) = self.nvlist.as_mut() {
            self.err = nvlist
                .add_string("compression", value.as_str())
                .map_err(Into::into)
                .err();
        }

        self
    }

    pub fn exec(mut self, v: impl Into<property::OnOff>) -> Self {
        if self.err.is_some() {
            return self;
        }

        let value = v.into();
        if let Some(nvlist) = self.nvlist.as_mut() {
            self.err = nvlist
                .add_string("exec", value.as_str())
                .map_err(Into::into)
                .err();
        }

        self
    }

    pub fn create(self) -> Result<Dataset> {
        match self.err {
            Some(err) => Err(err),
            None => {
                if let Some(nvlist) = self.nvlist {
                    let ret = unsafe {
                        sys::lzc_create(
                            CString::new(self.name.as_bytes())?.as_ptr(),
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
                        sys::make_dataset_handle(
                            ZFS_HANDLER.lock().unwrap().handler(),
                            CString::new(self.name.as_bytes())?.as_ptr(),
                        )
                    };

                    let mut nvl = unsafe {
                        libnvpair::NvList {
                            raw: (*zfs_handle).zfs_props,
                        }
                    };

                    Ok(Dataset {
                        name: self.name,
                        dataset: DatasetType::Filesystem(from_nvlist(&mut nvl)?),
                    })
                } else {
                    Err(DatasetError::Unknown)
                }
            }
        }
    }
}
