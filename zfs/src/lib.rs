#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

use razor_nvpair::NvListError;
use razor_zfscore::core;
use razor_zfscore::zfs_prop_t;
use razor_zfscore::ZfsDatasetHandler;

pub mod error;
pub mod zfs;

pub type Result<T> = std::result::Result<T, error::DatasetError>;
