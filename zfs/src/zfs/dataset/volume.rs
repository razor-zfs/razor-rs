use super::*;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Volume {
    #[serde(default)]
    pub(crate) name: property::Name,
    pub(crate) available: property::Available,
    pub(crate) volsize: property::Volsize,
    pub(crate) volblocksize: property::VolBlockSize,
    pub(crate) logicalused: Option<property::LogicalUsed>,
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

impl Volume {
    pub fn destroy(self) -> Result<()> {
        core::destroy_dataset(self.name.value().to_string_lossy()).map_err(|err| err.into())
    }

    pub fn available(&self) -> u64 {
        self.available.value()
    }

    pub fn logicalused(&self) -> u64 {
        match self.logicalused {
            Some(logicalused) => logicalused.value(),
            None => property::LogicalUsed::default().value(),
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
pub struct VolumeBuilder {
    nvlist: Result<libnvpair::NvList>,
    name: String,
    volblocksize: u64,
}

impl VolumeBuilder {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            nvlist: libnvpair::NvList::new(libnvpair::NvFlag::UniqueName)
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
        if let Ok(nvlist) = &mut self.nvlist {
            if let Err(err) = nvlist.add_uint64("volmode", value.into()) {
                self.nvlist = Err(err.into());
            }
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
        match self.nvlist.as_mut() {
            Ok(nvlist) => {
                nvlist.add_uint64("volsize", size)?;

                // TODO: check if volblocksize is power of 2 and between 512 and 128000
                nvlist.add_uint64("volblocksize", self.volblocksize)?;

                let mut nvl = core::create_volume(self.name, nvlist)?;

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
