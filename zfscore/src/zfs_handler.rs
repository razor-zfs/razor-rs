use std::ffi::CStr;
use std::ffi::CString;

use razor_nvpair::NvListAccess;
use razor_nvpair::Value;

use super::error::CoreError;
use super::libzfs_handler::LibZfsHandler;
use super::mnttab::Mnttab;
use super::sys;
use super::Result;

#[derive(Debug)]
pub struct ZfsDatasetHandler {
    name: CString,
    zfs_props: razor_nvpair::NvList,
    mntdata: Option<Mnttab>,
}

impl ZfsDatasetHandler {
    pub fn new(name: CString) -> Result<Self> {
        let zfs_handle =
            unsafe { sys::make_dataset_handle(LibZfsHandler::handler(), name.as_ptr()) };

        if zfs_handle.is_null() {
            return Err(CoreError::DatasetNotExist);
        }

        let zfs_props = razor_nvpair::NvList::from(unsafe { (*zfs_handle).zfs_props });

        let mntdata = Mnttab::find(&name);

        Ok(Self {
            name,
            zfs_props,
            mntdata,
        })
    }

    pub fn get_name(&self) -> String {
        self.name.to_string_lossy().into_owned()
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

    pub fn check_mnt_option(&self, opt: impl AsRef<str>) -> bool {
        if let Some(mnt) = &self.mntdata {
            mnt.hasmntopt(opt)
        } else {
            false
        }
    }

    pub fn is_mounted(&self) -> bool {
        if let Some(mnt) = &self.mntdata {
            !mnt.mntopts().is_empty()
        } else {
            false
        }
    }

    pub fn search_property(&self, name: impl AsRef<str>) -> Result<Value> {
        let nvp = self.zfs_props.lookup_nvpair(name)?;
        Ok(nvp.value())
    }
}

// TODO: check how to free zfs_handle_t
// impl Drop for ZfsDatasetHandler {
//     fn drop(&mut self) {
//         unsafe { sys::zfs_close((*self.raw).zfs_mntopts as *mut libc::c_void) };
//         unsafe { libc::free(self.raw as *mut libc::c_void) };
//     }
// }
