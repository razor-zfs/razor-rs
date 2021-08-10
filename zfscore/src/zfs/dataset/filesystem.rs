use super::sys;
use super::*;

#[derive(Debug, Deserialize)]
pub struct FilesystemIntermediate {
    available: zfs_property::Available,
    atime: Option<zfs_property::Atime>,
    logicalused: zfs_property::LogicalUsed,
    canmount: Option<zfs_property::CanMount>,
    mounted: Option<zfs_property::Mounted>,
    checksum: Option<zfs_property::CheckSum>,
    compression: Option<zfs_property::Compression>,
    guid: zfs_property::Guid,
    creation: zfs_property::Creation,
    createtxg: zfs_property::CreateTxg,
    compressratio: zfs_property::CompressRatio,
    used: zfs_property::Used,
    referenced: zfs_property::Referenced,
    logicalreferenced: zfs_property::LogicalReferenced,
    objsetid: zfs_property::ObjSetId,
}

// TODO: remove unwrap
impl FilesystemIntermediate {
    pub fn convert_to_valid<T>(self, zfs: &Zfs, dataset_name: T) -> Result<Filesystem>
    where
        T: AsRef<str>,
    {
        Ok(Filesystem {
            name: dataset_name.as_ref().to_string(),
            available: self.available,
            atime: self
                .atime
                .unwrap_or_else(|| zfs_property::Atime::default(zfs).unwrap()),
            //atime: zfs_property::Atime::new(zfs_property::OnOff::On),
            logicalused: self.logicalused,
            canmount: self
                .canmount
                .unwrap_or_else(zfs_property::CanMount::default),
            mounted: self
                .mounted
                .unwrap_or_else(|| zfs_property::Mounted::default(zfs).unwrap()),
            checksum: self
                .checksum
                .unwrap_or_else(|| zfs_property::CheckSum::default().unwrap()),
            compression: self
                .compression
                .unwrap_or_else(|| zfs_property::Compression::default().unwrap()),
            guid: self.guid,
            creation: self.creation,
            createtxg: self.createtxg,
            compressratio: self.compressratio,
            used: self.used,
            referenced: self.referenced,
            logicalreferenced: self.logicalreferenced,
            objsetid: self.objsetid,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Filesystem {
    pub name: String,
    pub available: zfs_property::Available,
    pub atime: zfs_property::Atime,
    pub logicalused: zfs_property::LogicalUsed,
    pub canmount: zfs_property::CanMount,
    pub mounted: zfs_property::Mounted,
    pub checksum: zfs_property::CheckSum,
    pub compression: zfs_property::Compression,
    pub guid: zfs_property::Guid,
    pub creation: zfs_property::Creation,
    pub createtxg: zfs_property::CreateTxg,
    pub compressratio: zfs_property::CompressRatio,
    pub used: zfs_property::Used,
    pub referenced: zfs_property::Referenced,
    pub logicalreferenced: zfs_property::LogicalReferenced,
    pub objsetid: zfs_property::ObjSetId,
}
impl Filesystem {
    pub fn destroy(self) -> Result<()> {
        unsafe { sys::libzfs_core_init() };
        if unsafe { sys::lzc_destroy(CString::new(self.name)?.as_ptr()) } != 0 {
            return Err(DatasetError::DatasetDeleteError);
        }
        unsafe { sys::libzfs_core_fini() };

        Ok(())
    }
}
