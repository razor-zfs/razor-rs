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

use razor_libnvpair_sys as sys;

pub use sys::boolean_t;
pub use sys::data_type_t;
pub use sys::nvlist_add_boolean;
pub use sys::nvlist_add_boolean_array;
pub use sys::nvlist_add_boolean_value;
pub use sys::nvlist_add_byte;
pub use sys::nvlist_add_byte_array;
pub use sys::nvlist_add_double;
pub use sys::nvlist_add_int16;
pub use sys::nvlist_add_int16_array;
pub use sys::nvlist_add_int32;
pub use sys::nvlist_add_int32_array;
pub use sys::nvlist_add_int64;
pub use sys::nvlist_add_int64_array;
pub use sys::nvlist_add_int8;
pub use sys::nvlist_add_int8_array;
pub use sys::nvlist_add_nvlist;
pub use sys::nvlist_add_nvlist_array;
pub use sys::nvlist_add_nvpair;
pub use sys::nvlist_add_string;
pub use sys::nvlist_add_string_array;
pub use sys::nvlist_add_uint16;
pub use sys::nvlist_add_uint16_array;
pub use sys::nvlist_add_uint32;
pub use sys::nvlist_add_uint32_array;
pub use sys::nvlist_add_uint64;
pub use sys::nvlist_add_uint64_array;
pub use sys::nvlist_add_uint8;
pub use sys::nvlist_add_uint8_array;
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
