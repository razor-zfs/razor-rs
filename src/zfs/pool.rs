use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use crate::sys;

use super::zpool_property;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pool {
    guid: zpool_property::Guid,
    loadguid: zpool_property::Loadguid,
    name: zpool_property::Name,
    size: zpool_property::Size,
    health: zpool_property::Health,
    free: zpool_property::Free,
    #[serde(flatten)]
    low_level: LowLevel,
    #[serde(flatten)]
    feature: Feature,
}

#[derive(Debug, Serialize, Deserialize)]
struct Feature {}

#[derive(Debug, Serialize, Deserialize)]
struct LowLevel {
    allocated: zpool_property::Allocated,
    altroot: zpool_property::Altroot,
    ashift: zpool_property::Ashift,
    autoexpand: zpool_property::Autoexpand,
    autoreplace: zpool_property::Autoreplace,
    autotrim: zpool_property::Autotrim,
    bootfs: zpool_property::Bootfs,
    cachefile: zpool_property::Cachefile,
    capacity: zpool_property::Capacity,
    comment: zpool_property::Comment,
    delegation: zpool_property::Delegation,
    dedupditto: zpool_property::Dedupditto,
    expandsize: zpool_property::Expandsize,
    failmode: zpool_property::Failmode,
    fragmentation: zpool_property::Fragmentation,
    freeing: zpool_property::Freeing,
    listsnapshots: zpool_property::Listsnapshots,
    multihost: zpool_property::Readonly,
    version: zpool_property::Version,
}

impl Pool {
    fn from_bunch(mut bunch: sys::Bunch) -> Result<Self, zpool_property::InvalidProperty> {
        let guid = zpool_property::extract_from_bunch(&mut bunch, "guid")?;
        let loadguid = zpool_property::extract_from_bunch(&mut bunch, "load_guid")?;
        let size = zpool_property::extract_from_bunch(&mut bunch, "size")?;
        let health = zpool_property::extract_from_bunch(&mut bunch, "health")?;
        let free = zpool_property::extract_from_bunch(&mut bunch, "free")?;
        let name = zpool_property::extract_from_bunch(&mut bunch, "name")?;
        let low_level = LowLevel::try_from(&mut bunch)?;
        let feature = Feature::try_from(&mut bunch)?;
        let pool = Self {
            guid,
            loadguid,
            name,
            size,
            health,
            free,
            low_level,
            feature,
        };

        Ok(pool)
    }
}

impl TryFrom<sys::Bunch> for Pool {
    type Error = zpool_property::InvalidProperty;

    fn try_from(bunch: sys::Bunch) -> Result<Self, Self::Error> {
        Self::from_bunch(bunch)
    }
}

impl TryFrom<&mut sys::Bunch> for Feature {
    type Error = zpool_property::InvalidProperty;

    fn try_from(mut _bunch: &mut sys::Bunch) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&mut sys::Bunch> for LowLevel {
    type Error = zpool_property::InvalidProperty;

    fn try_from(bunch: &mut sys::Bunch) -> Result<Self, Self::Error> {
        let allocated = zpool_property::extract_from_bunch(bunch, "allocated")?;
        let altroot = zpool_property::extract_from_bunch(bunch, "altroot")?;
        let ashift = zpool_property::extract_from_bunch(bunch, "ashift")?;
        let autoexpand = zpool_property::extract_from_bunch(bunch, "autoexpand")?;
        let autoreplace = zpool_property::extract_from_bunch(bunch, "autoreplace")?;
        let autotrim = zpool_property::extract_from_bunch(bunch, "autotrim")?;
        let bootfs = zpool_property::extract_from_bunch(bunch, "bootfs")?;
        let cachefile = zpool_property::extract_from_bunch(bunch, "cachefile")?;
        let capacity = zpool_property::extract_from_bunch(bunch, "capacity")?;
        let comment = zpool_property::extract_from_bunch(bunch, "comment")?;
        let delegation = zpool_property::extract_from_bunch(bunch, "delegation")?;
        let dedupditto = zpool_property::extract_from_bunch(bunch, "dedupditto")?;
        let expandsize = zpool_property::extract_from_bunch(bunch, "expandsize")?;
        let failmode = zpool_property::extract_from_bunch(bunch, "failmode")?;
        let fragmentation = zpool_property::extract_from_bunch(bunch, "fragmentation")?;
        let freeing = zpool_property::extract_from_bunch(bunch, "freeing")?;
        let listsnapshots = zpool_property::extract_from_bunch(bunch, "listsnapshots")?;
        let multihost = zpool_property::extract_from_bunch(bunch, "multihost")?;
        let version = zpool_property::extract_from_bunch(bunch, "version")?;

        let properties = Self {
            allocated,
            altroot,
            ashift,
            autoexpand,
            autoreplace,
            autotrim,
            bootfs,
            cachefile,
            capacity,
            comment,
            delegation,
            dedupditto,
            expandsize,
            failmode,
            fragmentation,
            freeing,
            listsnapshots,
            multihost,
            version,
        };

        Ok(properties)
    }
}
