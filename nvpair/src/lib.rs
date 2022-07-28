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

pub use libnvpair::data_type_t;
pub use libnvpair::NvListError;

pub use nvlist::NvFlag;
pub use nvlist::NvList;
pub use nvlist::NvListIterator;
pub use nvlist::NvListRef;
pub use nvlist::ToNvList;
pub use nvpair::NvPair;
pub use value::to_value;
pub use value::Value;

mod debug;
mod nvlist;
mod nvpair;
mod value;
