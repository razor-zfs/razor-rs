#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/nvpair.rs"));

pub fn init() -> data_type_t {
    let x = data_type_t::DATA_TYPE_UINT16;
    x
}
