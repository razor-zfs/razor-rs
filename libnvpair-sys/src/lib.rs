#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

//!
//! This module provides immediate bindings to libnvpair library.
//! It is not intended for direct use, but rather serves as an FFI layer.
//!
include!(concat!(env!("OUT_DIR"), "/nvpair.rs"));

impl From<boolean_t> for bool {
    fn from(value: boolean_t) -> Self {
        match value {
            boolean_t::B_FALSE => false,
            boolean_t::B_TRUE => true,
        }
    }
}

impl From<&boolean_t> for bool {
    fn from(value: &boolean_t) -> Self {
        match value {
            boolean_t::B_FALSE => false,
            boolean_t::B_TRUE => true,
        }
    }
}

impl From<bool> for boolean_t {
    fn from(value: bool) -> Self {
        match value {
            false => boolean_t::B_FALSE,
            true => boolean_t::B_TRUE,
        }
    }
}

impl From<&bool> for boolean_t {
    fn from(value: &bool) -> Self {
        match value {
            false => boolean_t::B_FALSE,
            true => boolean_t::B_TRUE,
        }
    }
}
