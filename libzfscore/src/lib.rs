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

use razor_libnvpair as libnvpair;
use razor_libzfs as libzfs;
use razor_libzfscore_sys as sys;
use razor_nvpair as nvpair;

use nvpair::NvList;
use nvpair::NvListError;

pub use sys::zfs_type_t;

pub use crate::dataset::DatasetCollectorBuilder;
pub use crate::dataset::ZfsDatasetHandle;
pub use crate::lzc::zfs_prop_t;

pub mod error;
pub mod lzc;

mod dataset;

pub type Result<T, E = error::CoreError> = ::std::result::Result<T, E>;
