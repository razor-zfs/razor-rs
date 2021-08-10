use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use crate::sys;

use super::zfs_property;

#[derive(Debug, Serialize, Deserialize)]
pub enum Dataset {
    Filesystem(Filesystem),
    Volume(Volume),
    Snapshot(Snapshot),
    Bookmark(Bookmark),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filesystem {
    available: zfs_property::Available,
    atime: zfs_property::Atime,
    logicalused: zfs_property::LogicalUsed,
    canmount: zfs_property::CanMount,
    mounted: zfs_property::Mounted,
    checksum: zfs_property::CheckSum,
    compression: zfs_property::Compression,
    #[serde(flatten)]
    common: CommonProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    available: zfs_property::Available,
    volsize: zfs_property::Volsize,
    volblocksize: zfs_property::VolBlockSize,
    logicalused: zfs_property::LogicalUsed,
    checksum: zfs_property::CheckSum,
    compression: zfs_property::Compression,
    #[serde(flatten)]
    common: CommonProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(flatten)]
    common: CommonProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    #[serde(flatten)]
    common: CommonProperties,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommonProperties {
    guid: zfs_property::Guid,
    creation: zfs_property::Creation,
    createtxg: zfs_property::CreateTxg,
    compressratio: zfs_property::CompressRatio,
    used: zfs_property::Used,
    referenced: zfs_property::Referenced,
    logicalreferenced: zfs_property::LogicalReferenced,
    objsetid: zfs_property::ObjSetId,
    unprocessed: sys::Bunch,
}

impl Dataset {
    fn from_bunch(mut bunch: sys::Bunch) -> Result<Self, zfs_property::InvalidProperty> {
        let r#type: zfs_property::Type = zfs_property::extract_from_bunch(&mut bunch, "type")?;
        match r#type.value() {
            zfs_property::DatasetType::Filesystem => {
                let filesystem = Self::filesystem(bunch)?;
                Ok(Self::Filesystem(filesystem))
            }
            zfs_property::DatasetType::Volume => {
                let volume = Self::volume(bunch)?;
                Ok(Self::Volume(volume))
            }
            zfs_property::DatasetType::Snapshot => {
                let snapshot = Self::snapshot(bunch)?;
                Ok(Self::Snapshot(snapshot))
            }
            zfs_property::DatasetType::Bookmark => {
                let bookmark = Self::bookmark(bunch)?;
                Ok(Self::Bookmark(bookmark))
            }
        }
    }

    fn filesystem(mut bunch: sys::Bunch) -> Result<Filesystem, zfs_property::InvalidProperty> {
        let available = zfs_property::extract_from_bunch(&mut bunch, "available")?;
        let atime = zfs_property::extract_from_bunch(&mut bunch, "atime")?;
        let logicalused = zfs_property::extract_from_bunch(&mut bunch, "logicalused")?;
        let canmount = zfs_property::extract_from_bunch(&mut bunch, "canmount")?;
        let mounted = zfs_property::extract_from_bunch(&mut bunch, "mounted")?;
        let checksum = zfs_property::extract_from_bunch(&mut bunch, "checksum")?;
        let compression = zfs_property::extract_from_bunch(&mut bunch, "compression")?;
        let common = CommonProperties::try_from(bunch)?;
        let filesystem = Filesystem {
            available,
            atime,
            logicalused,
            canmount,
            mounted,
            checksum,
            compression,
            common,
        };
        Ok(filesystem)
    }

    fn volume(mut bunch: sys::Bunch) -> Result<Volume, zfs_property::InvalidProperty> {
        let available = zfs_property::extract_from_bunch(&mut bunch, "available")?;
        let volsize = zfs_property::extract_from_bunch(&mut bunch, "volsize")?;
        let volblocksize = zfs_property::extract_from_bunch(&mut bunch, "volblocksize")?;
        let logicalused = zfs_property::extract_from_bunch(&mut bunch, "logicalused")?;
        let checksum = zfs_property::extract_from_bunch(&mut bunch, "checksum")?;
        let compression = zfs_property::extract_from_bunch(&mut bunch, "compression")?;
        let common = CommonProperties::try_from(bunch)?;
        let volume = Volume {
            available,
            volsize,
            volblocksize,
            logicalused,
            checksum,
            compression,
            common,
        };
        Ok(volume)
    }

    fn snapshot(bunch: sys::Bunch) -> Result<Snapshot, zfs_property::InvalidProperty> {
        let common = CommonProperties::try_from(bunch)?;
        let snapshot = Snapshot { common };
        Ok(snapshot)
    }

    fn bookmark(bunch: sys::Bunch) -> Result<Bookmark, zfs_property::InvalidProperty> {
        let common = CommonProperties::try_from(bunch)?;
        let bookmark = Bookmark { common };
        Ok(bookmark)
    }
}

impl TryFrom<sys::Bunch> for Dataset {
    type Error = zfs_property::InvalidProperty;

    fn try_from(bunch: sys::Bunch) -> Result<Self, Self::Error> {
        Self::from_bunch(bunch)
    }
}

impl TryFrom<sys::Bunch> for CommonProperties {
    type Error = zfs_property::InvalidProperty;

    fn try_from(mut bunch: sys::Bunch) -> Result<Self, Self::Error> {
        let guid = zfs_property::extract_from_bunch(&mut bunch, "guid")?;
        let creation = zfs_property::extract_from_bunch(&mut bunch, "creation")?;
        let createtxg = zfs_property::extract_from_bunch(&mut bunch, "createtxg")?;
        let compressratio = zfs_property::extract_from_bunch(&mut bunch, "compressratio")?;
        let used = zfs_property::extract_from_bunch(&mut bunch, "used")?;
        let referenced = zfs_property::extract_from_bunch(&mut bunch, "referenced")?;
        let logicalreferenced = zfs_property::extract_from_bunch(&mut bunch, "logicalreferenced")?;
        let objsetid = zfs_property::extract_from_bunch(&mut bunch, "objsetid")?;

        let properties = Self {
            guid,
            creation,
            createtxg,
            compressratio,
            used,
            referenced,
            logicalreferenced,
            objsetid,
            unprocessed: bunch,
        };
        Ok(properties)
    }
}
