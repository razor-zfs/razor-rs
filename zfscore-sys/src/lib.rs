#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

pub use razor_libnvpair::*;

include!(concat!(env!("OUT_DIR"), "/zfs_core.rs"));
