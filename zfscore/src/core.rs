use std::ffi::CString;

use nvpair::NvListAccess;

use crate::error::CoreError;

use super::error::value_or_err;
use super::libzfs_handler::LibZfsHandler;
use super::nvpair;
use super::sys;
use super::Result;

fn init() {
    LibZfsHandler::handler();
}

pub fn create_filesystem(name: impl AsRef<str>, nvl: &nvpair::NvList) -> Result<()> {
    create_dataset(name, nvl, sys::lzc_dataset_type::LZC_DATSET_TYPE_ZFS)
}

pub fn create_volume(name: impl AsRef<str>, nvl: &nvpair::NvList) -> Result<()> {
    create_dataset(name, nvl, sys::lzc_dataset_type::LZC_DATSET_TYPE_ZVOL)
}

fn create_dataset(
    name: impl AsRef<str>,
    nvl: &nvpair::NvList,
    prop: sys::lzc_dataset_type,
) -> Result<()> {
    init();
    let cname = CString::new(name.as_ref())?;

    let rc = unsafe { sys::lzc_create(cname.as_ptr(), prop, nvl.nvl(), std::ptr::null_mut(), 0) };

    value_or_err((), rc)
}

pub fn destroy_dataset(name: impl AsRef<str>) -> Result<()> {
    init();
    let cname = CString::new(name.as_ref())?;
    let rc = unsafe { sys::lzc_destroy(cname.as_ptr()) };

    value_or_err((), rc)
}
