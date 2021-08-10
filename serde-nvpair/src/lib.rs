#![forbid(unsafe_code)]

pub use de::from_nvlist;
pub use razor_nvpair as libnvpair;
pub use ser::to_nvlist;

mod de;
mod ser;

pub type Result<T> = std::result::Result<T, libnvpair::NvListError>;
