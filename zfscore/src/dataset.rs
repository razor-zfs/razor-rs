use std::ffi::CString;

use razor_nvpair as nvpair;

use nvpair::NvListAccess;

use crate::libzfs;
use crate::lzc;

use super::error::CoreError;
use super::mnttab::Mnttab;
use super::Result;

#[derive(Debug)]
pub struct ZfsDatasetHandle {
    name: CString,
    handle: *mut lzc::zfs_handle_t,
    // zfs_props: razor_nvpair::NvList,
    mntdata: Option<Mnttab>,
}

impl ZfsDatasetHandle {
    pub fn new(name: CString) -> Result<Self> {
        let handle = unsafe { libzfs::zfs_open(name.as_ptr()) };

        if handle.is_null() {
            return Err(CoreError::DatasetNotExist);
        }

        // let zfs_props = razor_nvpair::NvList::from(unsafe { (*zfs_handle).zfs_props });

        let mntdata = Mnttab::find(&name);

        Ok(Self {
            name,
            handle,
            mntdata,
        })
    }

    pub fn get_name(&self) -> String {
        self.name.to_string_lossy().into_owned()
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

    pub fn numeric_property(&self, name: &str, property: lzc::zfs_prop_t) -> u64 {
        let nvl = unsafe { libzfs::zfs_get_all_props(self.handle) };
        let nvl = nvpair::NvListRef::from_raw(nvl, self);

        if let Ok(nvp) = nvl.lookup_nvpair(name) {
            nvp.uint64()
        } else {
            lzc::zfs_prop_default_numeric(property)
        }
    }

    pub fn numeric_property_new(&self, property: lzc::zfs_prop_t) -> u64 {
        unsafe { libzfs::zfs_prop_get_numeric(self.handle, property) }
    }

    pub fn string_property(&self, name: &str, property: lzc::zfs_prop_t) -> String {
        let nvl = unsafe { libzfs::zfs_get_all_props(self.handle) };
        let nvl = nvpair::NvListRef::from_raw(nvl, self);

        if let Ok(nvp) = nvl.lookup_nvpair(name) {
            nvp.string().to_string()
        } else {
            lzc::zfs_prop_default_string(property).to_string()
        }
    }
}

impl Drop for ZfsDatasetHandle {
    fn drop(&mut self) {
        unsafe { libzfs::zfs_close(self.handle) };
    }
}