use std::ffi::CString;

use super::error::CoreError;
use super::libzfs_handler::LIB_ZFS_HANDLER;
use super::nvpair::NvList;
use super::sys;
use super::Result;

#[derive(Debug, Clone, PartialEq)]
struct MntOpts {
    raw: *mut u8,
}

impl Drop for MntOpts {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut libc::c_void) };
    }
}

impl MntOpts {
    fn as_ptr(&self) -> *mut u8 {
        self.raw
    }

    fn exist(&self) -> bool {
        !self.raw.is_null()
    }
}

impl From<*mut u8> for MntOpts {
    fn from(mnt_opts: *mut u8) -> Self {
        Self { raw: mnt_opts }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MntFsType {
    raw: *mut u8,
}

impl Drop for MntFsType {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut libc::c_void) };
    }
}

impl MntFsType {
    fn _as_ptr(&self) -> *mut u8 {
        self.raw
    }
}

impl From<*mut u8> for MntFsType {
    fn from(mnt_fstype: *mut u8) -> Self {
        Self { raw: mnt_fstype }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MntMountp {
    raw: *mut u8,
}

impl Drop for MntMountp {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut libc::c_void) };
    }
}

impl MntMountp {
    fn _as_ptr(&self) -> *mut u8 {
        self.raw
    }
}

impl From<*mut u8> for MntMountp {
    fn from(mnt_mountp: *mut u8) -> Self {
        Self { raw: mnt_mountp }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MntSpecial {
    raw: *mut u8,
}

impl Drop for MntSpecial {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut libc::c_void) };
    }
}

impl MntSpecial {
    fn _as_ptr(&self) -> *mut u8 {
        self.raw
    }
}

impl From<*mut u8> for MntSpecial {
    fn from(mnt_special: *mut u8) -> Self {
        Self { raw: mnt_special }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MntData {
    raw: *mut sys::mnttab,
    mnt_opts: MntOpts,
    mnt_fstype: MntFsType,
    mnt_mountp: MntMountp,
    mnt_special: MntSpecial,
}

impl Drop for MntData {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut libc::c_void) };
    }
}

impl MntData {
    fn new(mnt_data: *mut sys::mnttab) -> MntData {
        MntData {
            raw: mnt_data,
            mnt_opts: unsafe { (*mnt_data).mnt_mntopts.into() },
            mnt_fstype: unsafe { (*mnt_data).mnt_fstype.into() },
            mnt_mountp: unsafe { (*mnt_data).mnt_mountp.into() },
            mnt_special: unsafe { (*mnt_data).mnt_special.into() },
        }
    }

    fn mntopts(&self) -> &MntOpts {
        &self.mnt_opts
    }

    fn _fstype(&self) -> &MntFsType {
        &self.mnt_fstype
    }

    fn _mntmountp(&self) -> &MntMountp {
        &self.mnt_mountp
    }

    fn _mntspecial(&self) -> &MntSpecial {
        &self.mnt_special
    }

    fn has_mnt_option(&self, option: &[u8]) -> bool {
        let mut mntent: sys::mntent = unsafe { std::mem::zeroed() };
        mntent.mnt_opts = self.mnt_opts.as_ptr();

        unsafe {
            !sys::hasmntopt(
                &mut mntent,
                CString::from_vec_unchecked(option.to_vec()).as_ptr(),
            )
            .is_null()
        }
    }

    fn has_mnt_atime(&self) -> bool {
        self.has_mnt_option(b"atime")
    }

    fn has_mnt_noatime(&self) -> bool {
        self.has_mnt_option(b"noatime")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ZfsDatasetHandler {
    raw: *mut sys::zfs_handle_t,
    mntdata: Option<MntData>,
}

impl ZfsDatasetHandler {
    pub fn new(name: CString) -> Result<ZfsDatasetHandler> {
        let mut mnt = unsafe { std::mem::zeroed() };

        let zfs_handle =
            unsafe { sys::make_dataset_handle(LIB_ZFS_HANDLER.lock().handler(), name.as_ptr()) };

        if zfs_handle.is_null() {
            return Err(CoreError::DatasetNotExist);
        }

        let rc = unsafe {
            sys::libzfs_mnttab_find(
                LIB_ZFS_HANDLER.lock().handler(),
                (*zfs_handle).zfs_name.as_ptr(),
                &mut mnt,
            )
        };

        if rc == 0 {
            Ok(ZfsDatasetHandler {
                raw: zfs_handle,
                mntdata: Some(MntData::new(&mut mnt)),
            })
        } else {
            Ok(ZfsDatasetHandler {
                raw: zfs_handle,
                mntdata: None,
            })
        }
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
