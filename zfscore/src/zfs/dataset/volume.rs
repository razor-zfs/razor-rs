use super::*;
use crate::zfs::zfs_handler::ZFS_HANDLER;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Volume {
    #[serde(default)]
    name: property::Name,
    available: property::Available,
    volsize: property::Volsize,
    volblocksize: property::VolBlockSize,
    logicalused: Option<property::LogicalUsed>,
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

impl Volume {
    pub fn destroy(self) -> Result<()> {
        if unsafe { sys::lzc_destroy(self.name.value().as_ptr()) } != 0 {
            return Err(DatasetError::DatasetDeleteError);
        }

        Ok(())
    }

    pub fn available(&self) -> property::Available {
        self.available
    }

    pub fn logicalused(&self) -> property::LogicalUsed {
        match self.logicalused {
            Some(logicalused) => logicalused,
            None => property::LogicalUsed::default(),
        }
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
pub struct VolumeBuilder {
    nvlist: Result<libnvpair::NvList>,
    name: String,
    volblocksize: u64,
    //err: Option<DatasetError>,
}

impl VolumeBuilder {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            nvlist: libnvpair::NvList::nvlist_alloc(libnvpair::NvFlag::UniqueName)
                .map_err(|nvlist_err| nvlist_err.into()),
            name: name.as_ref().to_string(),
            volblocksize: Self::calculate_default_volblocksize(),
        }
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

    pub fn compression(mut self, v: impl Into<property::CompressionAlgo>) -> Self {
        let value = v.into();
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_string("compression", value.as_str()) {
                self.nvlist = Err(err.into());
            }
        }

        self
    }

    pub fn blocksize(mut self, v: u64) -> Result<Self> {
        self.volblocksize = v;
        Ok(self)
    }

    // TODO: implement calculation algorithm
    fn calculate_default_volblocksize() -> u64 {
        8192
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

        let cname = CString::new(self.name.as_bytes())?;
        match self.nvlist.as_mut() {
            Ok(nvlist) => {
                nvlist.add_uint64("volsize", size)?;

                nvlist.add_uint64("volmode", 3)?;

                // TODO: check if volblocksize is power of 2 and between 512 and 128000
                nvlist.add_uint64("volblocksize", self.volblocksize)?;

                let rc = unsafe {
                    sys::lzc_create(
                        CString::new(self.name.clone())?.as_ptr(),
                        sys::lzc_dataset_type::LZC_DATSET_TYPE_ZVOL,
                        nvlist.raw,
                        std::ptr::null_mut(),
                        0,
                    )
                };
                if rc != 0 {
                    dbg!("error ", rc);
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

                let volume: Volume = from_nvlist(&mut nvl).map(|fs| Volume {
                    name: property::Name::new(cname),
                    ..fs
                })?;

                Ok(volume)
            }
            Err(err) => Err(err.clone()), // TODO: check this line because it clones here
        }
    }
}
