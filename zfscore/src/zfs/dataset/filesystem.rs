use super::*;

#[derive(Debug, Deserialize)]
pub struct FilesystemIntermediate {
    available: zfs_property::Available,
    //#[serde(default = "zfs_property::Atime::default")]
    atime: Option<zfs_property::Atime>,
    logicalused: zfs_property::LogicalUsed,
    //#[serde(default = "zfs_property::CanMount::default")]
    canmount: Option<zfs_property::CanMount>,
    //#[serde(default = "zfs_property::Mounted::default")]
    mounted: Option<zfs_property::Mounted>,
    //#[serde(default = "zfs_property::CheckSum::default")]
    checksum: Option<zfs_property::CheckSum>,
    //#[serde(default = "zfs_property::Compression::default")]
    compression: Option<zfs_property::Compression>,
    //common: CommonProperties,
}

// TODO: remove unwrap
impl FilesystemIntermediate {
    pub fn convert_to_valid(self, zfs: &Zfs) -> Result<Filesystem> {
        Ok(Filesystem {
            available: self.available,
            atime: self
                .atime
                .unwrap_or_else(|| zfs_property::Atime::default(zfs).unwrap()),
            //atime: zfs_property::Atime::new(zfs_property::OnOff::On),
            logicalused: self.logicalused,
            canmount: self
                .canmount
                .unwrap_or_else(|| zfs_property::CanMount::default()),
            mounted: self
                .mounted
                .unwrap_or_else(|| zfs_property::Mounted::default(zfs).unwrap()),
            checksum: self
                .checksum
                .unwrap_or_else(|| zfs_property::CheckSum::default().unwrap()),
            compression: self
                .compression
                .unwrap_or_else(|| zfs_property::Compression::default().unwrap()),
        })
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Filesystem {
    pub available: zfs_property::Available,
    pub atime: zfs_property::Atime,
    pub logicalused: zfs_property::LogicalUsed,
    pub canmount: zfs_property::CanMount,
    pub mounted: zfs_property::Mounted,
    pub checksum: zfs_property::CheckSum,
    pub compression: zfs_property::Compression,
    //common: CommonProperties,
}
