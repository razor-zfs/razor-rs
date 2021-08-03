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
}
