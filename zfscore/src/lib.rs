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

use razor_nvpair as nvpair;

use nvpair::NvListError;

pub use crate::lzc::zfs_prop_t;
pub use zfs_handler::ZfsDatasetHandler;

pub mod error;
pub mod lzc;

mod libzfs;
mod mnttab;
mod zfs_handler;

pub type Result<T> = std::result::Result<T, error::CoreError>;
