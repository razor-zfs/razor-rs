#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
// #![warn(clippy::use_self)]
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

pub use error::InvalidProperty;
pub use property::CheckSum;
pub use property::Compression;
pub use property::OnOff;
pub use property::OnOffNoAuto;
pub use property::TimeStamp;
pub use property::Type;
pub use property::VolMode;
pub use property::YesNo;

mod error;
mod property;
