use std::ffi::CString;

use crate::error::CoreError;

use super::libzfs_handler::LIB_ZFS_HANDLER;
use super::nvpair::NvList;
use super::sys;
use super::Result;

#[derive(Debug)]
pub struct ZfsDatasetHandler {
    raw: *mut sys::zfs_handle_t,
    zfs_props: NvList,
    zfs_user_props: NvList,
    zfs_recvd_props: NvList,
}

impl ZfsDatasetHandler {
    pub fn new(name: CString) -> Result<ZfsDatasetHandler> {
        let zfs_handle =
            unsafe { sys::make_dataset_handle(LIB_ZFS_HANDLER.lock().handler(), name.as_ptr()) };

        if zfs_handle.is_null() {
            return Err(CoreError::DatasetNotExist);
        }

        let zfs_props = unsafe { (*zfs_handle).zfs_props };
        let zfs_user_props = unsafe { (*zfs_handle).zfs_user_props };
        let zfs_recvd_props = unsafe { (*zfs_handle).zfs_recvd_props };

        Ok(ZfsDatasetHandler {
            raw: zfs_handle,
            zfs_props: zfs_props.into(),
            zfs_user_props: zfs_user_props.into(),
            zfs_recvd_props: zfs_recvd_props.into(),
        })
    }
}
