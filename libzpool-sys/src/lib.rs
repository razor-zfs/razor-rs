#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(clippy::missing_safety_doc)]

use razor_libnvpair::*;
// use razor_libzfscore::*;

include!(concat!(env!("OUT_DIR"), "/zpool.rs"));
