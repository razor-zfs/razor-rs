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

use razor_nvpair_sys as sys;

pub use error::NvListError;
pub use library::ContextType;
pub use library::CtxIter;
pub use library::NvFlag;
pub use library::NvList;
pub use library::NvListIterator;
pub use library::NvPair;
pub use library::NvPairType;
pub use library::SafeNvPair;

mod error;
mod library;

pub type Result<T> = std::result::Result<T, NvListError>;
