#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(clippy::return_self_not_must_use)]

pub use razor_libnvpair::*;

include!(concat!(env!("OUT_DIR"), "/zfs_core.rs"));
