use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use guid::Guid;
pub use name::Name;

use crate::sys;

mod guid;
mod name;
pub mod property;

#[derive(Debug, Serialize, Deserialize)]
pub struct Zpool {
    guid: Guid,
    load_guid: Guid,
    name: String,
}

// pub type Dataset = sys::Bunch;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    guid: property::Guid,
    r#type: property::Type,
    available: property::Available,
    compressratio: property::CompressRatio,
}

impl Dataset {
    fn from_bunch(mut bunch: sys::Bunch) -> Result<Self, property::InvalidProperty> {
        let guid = extract_from_bunch(&mut bunch, "guid")?;
        let r#type = extract_from_bunch(&mut bunch, "type")?;
        let available = extract_from_bunch(&mut bunch, "available")?;
        let compressratio = extract_from_bunch(&mut bunch, "compressratio")?;

        let dataset = Self {
            guid,
            r#type,
            available,
            compressratio,
        };
        Ok(dataset)
    }
}

impl TryFrom<sys::Bunch> for Dataset {
    type Error = property::InvalidProperty;

    fn try_from(bunch: sys::Bunch) -> Result<Self, Self::Error> {
        Self::from_bunch(bunch)
    }
}

#[derive(Debug, Error)]
#[error("Invalid input")]
pub struct InvalidInput;

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
