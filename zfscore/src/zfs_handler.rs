use std::ffi::CString;

use super::error::CoreError;
use super::libzfs_handler::LibZfsHandler;
use super::mnttab::Mnttab;
use super::nvpair::NvList;
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

    fn _name_ptr(&self) -> *const u8 {
        unsafe { (*self.raw).zfs_name.as_ptr() }
    }

    fn _zfs_properties(&self) -> NvList {
        let props = unsafe { (*self.raw).zfs_props };

        props.into()
    }

    fn _zfs_user_properties(&self) -> NvList {
        let props = unsafe { (*self.raw).zfs_user_props };

        props.into()
    }

    fn _zfs_recvd_properties(&self) -> NvList {
        let zfs_recvd_props = unsafe { (*self.raw).zfs_recvd_props };

        zfs_recvd_props.into()
    }

    fn mntdata(&self) -> Option<&MntData> {
        self.mntdata.as_ref()
    }

    // TODO: 1.check mounted
    //       2. implement the same for relative, devices, exec, readonly, setuid, nbmand
    pub fn default_atime(&self) -> u64 {
        let res = unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_ATIME) };

        match self.mntdata() {
            Some(mntdata) => {
                if mntdata.has_mnt_atime() && res == 0 {
                    return 1;
                } else if mntdata.has_mnt_noatime() && res != 0 {
                    return 0;
                } else {
                    res
                }
            }
            None => res,
        }
    }

    pub fn default_mounted(&self) -> bool {
        match self.mntdata() {
            Some(mntdata) => mntdata.mntopts().exist(),
            None => false,
        }
    }
    pub fn default_type(&self) -> u64 {
        unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_TYPE) }
    }

    pub fn default_available(&self) -> u64 {
        unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_AVAILABLE) }
    }

    pub fn default_logicalused(&self) -> u64 {
        unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_LOGICALUSED) }
    }

    pub fn default_canmount(&self) -> u64 {
        unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_CANMOUNT) }
    }

    pub fn default_checksum(&self) -> u64 {
        unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_CHECKSUM) }
    }

    pub fn default_volmode(&self) -> u64 {
        unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_VOLMODE) }
    }

    pub fn default_compression(&self) -> u64 {
        unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_COMPRESSION) }
    }
}

// TODO: check how to free zfs_handle_t
impl Drop for ZfsDatasetHandler {
    fn drop(&mut self) {
        unsafe { libc::free((*self.raw).zfs_mntopts as *mut libc::c_void) };
        unsafe { libc::free(self.raw as *mut libc::c_void) };
    }
}
