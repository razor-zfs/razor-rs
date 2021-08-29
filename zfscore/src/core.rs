use std::ffi::CString;

use crate::error::CoreError;

use super::error::value_or_err;
use super::nvpair;
use super::sys;
use super::zfs_handler::ZFS_HANDLER;
use super::Result;

fn init() {
    let guard = ZFS_HANDLER.lock();
    drop(guard);
}

pub fn create_dataset(name: impl AsRef<str>, nvl: &nvpair::NvList) -> Result<nvpair::NvList> {
    init();
    let cname = CString::new(name.as_ref())?;
    let rc = unsafe {
        sys::lzc_create(
            cname.as_ptr(),
            sys::lzc_dataset_type::LZC_DATSET_TYPE_ZFS,
            nvl.raw,
            std::ptr::null_mut(),
            0,
        )
    };

    let zfs_handle =
        unsafe { sys::make_dataset_handle(ZFS_HANDLER.lock().handler(), cname.as_ptr()) };

    let nvl = unsafe {
        nvpair::NvList {
            raw: (*zfs_handle).zfs_props,
        }
    };

    value_or_err(nvl, rc)
}

pub fn get_dataset_nvlist(name: impl AsRef<str>) -> Result<nvpair::NvList> {
    init();
    let cname = CString::new(name.as_ref())?;
    let zfs_handle =
        unsafe { sys::make_dataset_handle(ZFS_HANDLER.lock().handler(), cname.as_ptr()) };

    if zfs_handle.is_null() {
        return Err(CoreError::DatasetGetError);
    }

    let nvl = unsafe {
        nvpair::NvList {
            raw: (*zfs_handle).zfs_props,
        }
    };

    Ok(nvl)
}

pub fn destroy_dataset(name: CString) -> Result<()> {
    init();
    let rc = unsafe { sys::lzc_destroy(name.as_ptr()) };

    value_or_err((), rc)
}
