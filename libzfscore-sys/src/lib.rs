#![allow(non_camel_case_types)]
#![allow(deref_nullptr)]

use razor_libnvpair::*;

include!(concat!(env!("OUT_DIR"), "/zfs_core.rs"));

pub fn translate_zfs_error(code: i32) -> zfs_error_t {
    let code = code as u32;
    if code == zfs_error::EZFS_SUCCESS {
        code
    } else if code < zfs_error::EZFS_NOMEM {
        zfs_error::EZFS_UNKNOWN
    } else if code < zfs_error::EZFS_UNKNOWN {
        code
    } else {
        zfs_error::EZFS_UNKNOWN
    }
}
