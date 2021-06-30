use serde::{Deserialize, Serialize};

pub use dataset::Dataset;
pub use guid::Guid;
pub use name::Name;

mod dataset;
mod guid;
mod name;
mod property;
pub mod zfs_property;
pub mod zpool_property;

#[derive(Debug, Serialize, Deserialize)]
pub struct Zpool {
    guid: Guid,
    load_guid: Guid,
    name: String,
}
