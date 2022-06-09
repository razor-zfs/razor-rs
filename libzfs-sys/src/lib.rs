#![allow(non_camel_case_types)]
#![allow(deref_nullptr)]

use razor_libnvpair::*;
use razor_libzfscore::*;

include!(concat!(env!("OUT_DIR"), "/zfs.rs"));
