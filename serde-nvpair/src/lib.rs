#![forbid(unsafe_code)]
pub use razor_nvpair as nvpair;

pub use de::from_nvlist;
pub use ser::to_nvlist;

mod de;
mod ser;

pub type Result<T> = std::result::Result<T, nvpair::NvListError>;
