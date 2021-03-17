use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::sys;

use super::property;

#[derive(Debug, Serialize, Deserialize)]
pub enum Dataset {
    Filesystem(Filesystem),
    Volume(Volume),
    Snapshot(Snapshot),
    Bookmark(Bookmark),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filesystem {
    available: property::Available,
    #[serde(flatten)]
    common: CommonProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
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
    guid: property::Guid,
    compressratio: property::CompressRatio,
    used: property::Used,
    referenced: property::Referenced,
    unprocessed: sys::Bunch,
}

impl Dataset {
    fn from_bunch(mut bunch: sys::Bunch) -> Result<Self, property::InvalidProperty> {
        let r#type: property::Type = extract_from_bunch(&mut bunch, "type")?;
        match r#type.value() {
            property::DatasetType::Filesystem => {
                let filesystem = Self::filesystem(bunch)?;
                Ok(Self::Filesystem(filesystem))
            }
            property::DatasetType::Volume => {
                let volume = Self::volume(bunch)?;
                Ok(Self::Volume(volume))
            }
            property::DatasetType::Snapshot => {
                let snapshot = Self::snapshot(bunch)?;
                Ok(Self::Snapshot(snapshot))
            }
            property::DatasetType::Bookmark => {
                let bookmark = Self::bookmark(bunch)?;
                Ok(Self::Bookmark(bookmark))
            }
        }
    }

    fn filesystem(mut bunch: sys::Bunch) -> Result<Filesystem, property::InvalidProperty> {
        let available = extract_from_bunch(&mut bunch, "available")?;
        let common = CommonProperties::try_from(bunch)?;
        let filesystem = Filesystem { available, common };
        Ok(filesystem)
    }

    fn volume(bunch: sys::Bunch) -> Result<Volume, property::InvalidProperty> {
        let common = CommonProperties::try_from(bunch)?;
        let volume = Volume { common };
        Ok(volume)
    }

    fn snapshot(bunch: sys::Bunch) -> Result<Snapshot, property::InvalidProperty> {
        let common = CommonProperties::try_from(bunch)?;
        let snapshot = Snapshot { common };
        Ok(snapshot)
    }

    fn bookmark(bunch: sys::Bunch) -> Result<Bookmark, property::InvalidProperty> {
        let common = CommonProperties::try_from(bunch)?;
        let bookmark = Bookmark { common };
        Ok(bookmark)
    }
}

impl TryFrom<sys::Bunch> for Dataset {
    type Error = property::InvalidProperty;

    fn try_from(bunch: sys::Bunch) -> Result<Self, Self::Error> {
        Self::from_bunch(bunch)
    }
}

impl TryFrom<sys::Bunch> for CommonProperties {
    type Error = property::InvalidProperty;

    fn try_from(mut bunch: sys::Bunch) -> Result<Self, Self::Error> {
        let guid = extract_from_bunch(&mut bunch, "guid")?;
        let compressratio = extract_from_bunch(&mut bunch, "compressratio")?;
        let used = extract_from_bunch(&mut bunch, "used")?;
        let referenced = extract_from_bunch(&mut bunch, "referenced")?;

        let properties = Self {
            guid,
            compressratio,
            used,
            referenced,
            unprocessed: bunch,
        };
        Ok(properties)
    }
}

fn extract_from_bunch<T>(
    bunch: &mut sys::Bunch,
    key: &str,
) -> Result<property::Property<T>, property::InvalidProperty>
where
    T: FromStr,
{
    let prop = bunch
        .remove(key)
        .ok_or_else(|| property::InvalidProperty::no_such_property(key))?
        .try_into()?;
    Ok(prop)
}
