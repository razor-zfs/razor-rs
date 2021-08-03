use super::*;

#[derive(Debug, Deserialize)]
pub struct Filesystem {
    available: zfs_property::Available,
    atime: zfs_property::Atime,
    logicalused: zfs_property::LogicalUsed,
    canmount: zfs_property::CanMount,
    mounted: zfs_property::Mounted,
    checksum: zfs_property::CheckSum,
    compression: zfs_property::Compression,
    common: CommonProperties,
}
