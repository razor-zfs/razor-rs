use std::ffi::CStr;
use std::ffi::CString;

use super::error::CoreError;
use super::libzfs_handler::LibZfsHandler;
use super::mnttab::Mnttab;
use super::sys;
use super::Result;

#[derive(Debug)]
pub struct ZfsDatasetHandler {
    raw: *mut sys::zfs_handle_t,
    mntdata: Option<Mnttab>,
}

impl ZfsDatasetHandler {
    pub fn new(name: CString) -> Result<ZfsDatasetHandler> {
        let zfs_handle =
            unsafe { sys::make_dataset_handle(LibZfsHandler::handler(), name.as_ptr()) };

        if zfs_handle.is_null() {
            return Err(CoreError::DatasetNotExist);
        }

        let mntdata = Mnttab::find(name);

        Ok(Self {
            raw: zfs_handle,
            mntdata,
        })
    }

    fn _name_ptr(&self) -> String {
        unsafe {
            CStr::from_bytes_with_nul_unchecked(&(*self.raw).zfs_name)
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn get_name(&self) -> String {
        unsafe { String::from_utf8_lossy(&(*self.raw).zfs_name).into_owned() }
    }

    pub fn get_prop_default_numeric(&self, prop: sys::zfs_prop_t) -> u64 {
        unsafe { sys::zfs_prop_default_numeric(prop) }
    }

    pub fn get_prop_default_string(&self, prop: sys::zfs_prop_t) -> String {
        unsafe {
            CStr::from_ptr(sys::zfs_prop_default_string(prop))
                .to_string_lossy()
                .into_owned()
        }
    }
}

// TODO: check how to free zfs_handle_t
impl Drop for ZfsDatasetHandler {
    fn drop(&mut self) {
        unsafe { libc::free((*self.raw).zfs_mntopts as *mut libc::c_void) };
        unsafe { libc::free(self.raw as *mut libc::c_void) };
    }
}
