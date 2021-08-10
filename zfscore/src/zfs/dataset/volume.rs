use super::*;

#[derive(Debug, Deserialize)]
pub struct Volume {
    available: zfs_property::Available,
    volsize: zfs_property::Volsize,
    volblocksize: zfs_property::VolBlockSize,
    logicalused: zfs_property::LogicalUsed,
    checksum: zfs_property::CheckSum,
    compression: zfs_property::Compression,
    common: CommonProperties,
    guid: zfs_property::Guid,
    creation: zfs_property::Creation,
    createtxg: zfs_property::CreateTxg,
    compressratio: zfs_property::CompressRatio,
    used: zfs_property::Used,
    referenced: zfs_property::Referenced,
    logicalreferenced: zfs_property::LogicalReferenced,
    objsetid: zfs_property::ObjSetId,
}
