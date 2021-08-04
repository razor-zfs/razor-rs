use std::sync::Once;

pub use super::error::DatasetError;
pub use super::Result;
pub use property::InvalidProperty;
pub(crate) use razorzfscore_sys as sys;
use razorzfsnvpair as libnvpair;

pub mod dataset;
mod property;
pub mod zfs_property;
pub mod zpool_property;

static START: Once = Once::new();

fn init_zfs() {
    START.call_once(|| {
        unsafe { sys::libzfs_core_init() };
    });
}
