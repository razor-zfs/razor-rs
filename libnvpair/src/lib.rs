#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
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
#![allow(clippy::missing_safety_doc)]
#![deny(warnings)]

//! This module mostly re-exports things from its -sys counterpart and adds
//! very simple ergonomic helpers
use razor_libnvpair_sys as sys;

pub use sys::boolean_t;
pub use sys::data_type_t;

pub use sys::fnvlist_add_boolean;
pub use sys::fnvlist_add_boolean_array;
pub use sys::fnvlist_add_boolean_value;
pub use sys::fnvlist_add_byte;
pub use sys::fnvlist_add_byte_array;
pub use sys::fnvlist_add_int16;
pub use sys::fnvlist_add_int16_array;
pub use sys::fnvlist_add_int32;
pub use sys::fnvlist_add_int32_array;
pub use sys::fnvlist_add_int64;
pub use sys::fnvlist_add_int64_array;
pub use sys::fnvlist_add_int8;
pub use sys::fnvlist_add_int8_array;
pub use sys::fnvlist_add_nvlist;
pub use sys::fnvlist_add_nvlist_array;
pub use sys::fnvlist_add_nvpair;
pub use sys::fnvlist_add_string;
pub use sys::fnvlist_add_string_array;
pub use sys::fnvlist_add_uint16;
pub use sys::fnvlist_add_uint16_array;
pub use sys::fnvlist_add_uint32;
pub use sys::fnvlist_add_uint32_array;
pub use sys::fnvlist_add_uint64;
pub use sys::fnvlist_add_uint64_array;
pub use sys::fnvlist_add_uint8;
pub use sys::fnvlist_add_uint8_array;
pub use sys::fnvlist_alloc;
pub use sys::fnvlist_dup;
pub use sys::fnvlist_free;

pub use sys::fnvlist_lookup_boolean;
pub use sys::fnvlist_lookup_boolean_array;
pub use sys::fnvlist_lookup_boolean_value;
pub use sys::fnvlist_lookup_byte;
pub use sys::fnvlist_lookup_byte_array;
pub use sys::fnvlist_lookup_int16;
pub use sys::fnvlist_lookup_int16_array;
pub use sys::fnvlist_lookup_int32;
pub use sys::fnvlist_lookup_int32_array;
pub use sys::fnvlist_lookup_int64;
pub use sys::fnvlist_lookup_int64_array;
pub use sys::fnvlist_lookup_int8;
pub use sys::fnvlist_lookup_int8_array;
pub use sys::fnvlist_lookup_nvlist;
pub use sys::fnvlist_lookup_nvpair;
pub use sys::fnvlist_lookup_string;
pub use sys::fnvlist_lookup_uint16;
pub use sys::fnvlist_lookup_uint16_array;
pub use sys::fnvlist_lookup_uint32;
pub use sys::fnvlist_lookup_uint32_array;
pub use sys::fnvlist_lookup_uint64;
pub use sys::fnvlist_lookup_uint64_array;
pub use sys::fnvlist_lookup_uint8;
pub use sys::fnvlist_lookup_uint8_array;

pub use sys::fnvpair_value_boolean_value;
pub use sys::fnvpair_value_byte;
pub use sys::fnvpair_value_int16;
pub use sys::fnvpair_value_int32;
pub use sys::fnvpair_value_int64;
pub use sys::fnvpair_value_int8;
pub use sys::fnvpair_value_nvlist;
pub use sys::fnvpair_value_string;
pub use sys::fnvpair_value_uint16;
pub use sys::fnvpair_value_uint32;
pub use sys::fnvpair_value_uint64;
pub use sys::fnvpair_value_uint8;

pub use sys::nvlist_empty;
pub use sys::nvlist_exists;
pub use sys::nvlist_free;
pub use sys::nvlist_next_nvpair;
pub use sys::nvlist_t;
pub use sys::nvpair_name;
pub use sys::nvpair_t;
pub use sys::nvpair_type;
pub use sys::NV_UNIQUE_NAME;
pub use sys::NV_UNIQUE_NAME_TYPE;

pub use nvlist::*;
pub use nvpair::*;

mod nvlist;
mod nvpair;

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("nvpair type mismatch")]
pub struct NvPairTypeMismatch;

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum NvListError {
    #[error("Invalid Argument (nvlist?)")]
    InvalidArgument,
    #[error("Insufficient memory")]
    OutOfMemory,
    #[error("No matching name-value pair is found")]
    NotFound,
}
