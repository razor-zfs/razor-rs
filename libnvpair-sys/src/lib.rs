#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

//!
//! This module provides immediate bindings to libnvpair library.
//! It is not intended for direct use, but rather serves as an FFI layer.
//!
include!(concat!(env!("OUT_DIR"), "/nvpair.rs"));
