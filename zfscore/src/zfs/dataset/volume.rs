use super::*;

#[derive(Debug, Deserialize)]
pub struct VolumeIntermediate {
    available: Option<zfs_property::Available>,
    volsize: zfs_property::Volsize,
    volblocksize: zfs_property::VolBlockSize,
    logicalused: Option<zfs_property::LogicalUsed>,
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
impl VolumeIntermediate {
    pub fn convert_to_valid<T>(self, dataset_name: T) -> Result<Volume>
    where
        T: AsRef<str>,
    {
        Ok(Volume {
            name: dataset_name.as_ref().to_string(),
            available: self
                .available
                .unwrap_or_else(zfs_property::Available::default),
            volsize: self.volsize,
            volblocksize: self.volblocksize,
            logicalused: self
                .logicalused
                .unwrap_or_else(zfs_property::LogicalUsed::default),
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

#[derive(Debug, Deserialize)]
pub struct Volume {
    name: String,
    available: zfs_property::Available,
    volsize: zfs_property::Volsize,
    volblocksize: zfs_property::VolBlockSize,
    logicalused: zfs_property::LogicalUsed,
    checksum: zfs_property::CheckSum,
    compression: zfs_property::Compression,
    guid: zfs_property::Guid,
    creation: zfs_property::Creation,
    createtxg: zfs_property::CreateTxg,
    compressratio: zfs_property::CompressRatio,
    used: zfs_property::Used,
    referenced: zfs_property::Referenced,
    logicalreferenced: zfs_property::LogicalReferenced,
    objsetid: zfs_property::ObjSetId,
}
