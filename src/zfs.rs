use serde::{Deserialize, Serialize};

pub use guid::Guid;
pub use name::Name;

mod guid;
mod name;
pub mod property;

const ZFS_GET_DELIMITER: char = '\t';

#[derive(Debug, Serialize, Deserialize)]
pub struct ZPool {
    guid: Guid,
    load_guid: Guid,
    name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    guid: Guid,
    name: String,
    available: property::Available,
    compressratio: property::CompressRatio,
}

impl From<property::Bunch> for Dataset {
    fn from(_bunch: property::Bunch) -> Self {
        todo!()
    }
}
