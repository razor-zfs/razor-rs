use super::*;

mod impls;

pub struct NvList {
    raw: *mut sys::nvlist_t,
}
