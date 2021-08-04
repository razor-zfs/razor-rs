pub use super::error::DatasetError;
pub use super::Result;
pub use property::InvalidProperty;
use razorzfsnvpair as libnvpair;

pub mod dataset;
mod property;
pub mod zfs_property;
pub mod zpool_property;
