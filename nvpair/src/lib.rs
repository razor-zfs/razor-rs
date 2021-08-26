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

pub use error::NvListError;
pub use nvlist::NvFlag;
pub use nvlist::NvList;
pub use nvlist::NvListIterator;
pub use nvpair::ContextType;
pub use nvpair::CtxIter;
pub use nvpair::NvPair;
pub use nvpair::NvPairType;
pub use nvpair::SafeNvPair;
pub use value::to_value;
pub use value::Value;

use razor_nvpair_sys as sys;

mod error;
mod impls;
mod nvlist;
mod nvpair;
mod value;

pub type Result<T> = std::result::Result<T, NvListError>;
