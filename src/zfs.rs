use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use serde_json as json;

pub use guid::Guid;
pub use name::Name;

use crate::sys;

mod guid;
mod name;
pub mod property;

// const ZFS_GET_DELIMITER: char = '\t';

#[derive(Debug, Serialize, Deserialize)]
pub struct Zpool {
    guid: Guid,
    load_guid: Guid,
    name: String,
}

// pub type Dataset = sys::Bunch;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    guid: Guid,
    name: String,
    available: property::Available,
    compressratio: property::CompressRatio,
}

impl TryFrom<sys::Bunch> for Dataset {
    type Error = json::Error;

    fn try_from(bunch: sys::Bunch) -> Result<Self, Self::Error> {
        let value = json::to_value(bunch)?;
        let dataset = json::from_value(value)?;
        Ok(dataset)
    }
}
