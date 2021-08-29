use std::ffi::CString;

use serde::{Deserialize, Serialize};

use super::core;
use super::sys;
use crate::error::{DatasetError, InvalidProperty};

pub use checksum::CheckSum as CheckSumAlgo;
pub use compression::Compression as CompressionAlgo;
pub use dataset::Type as DsType;
pub use onoff::OnOff;
pub use onoffnoauto::OnOffNoAuto;
pub use timestamp::TimeStamp;
pub use yesno::YesNo;

mod checksum;
mod compression;
mod dataset;
mod onoff;
mod onoffnoauto;
mod timestamp;
mod yesno;

use crate::zfs::zfs_handler::ZFS_HANDLER;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Guid {
    value: u64,
}
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct Name {
    value: CString,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Available {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct CompressRatio {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Type {
    value: DsType,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Used {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct LogicalUsed {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Referenced {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct LogicalReferenced {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct CreateTxg {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Creation {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Volsize {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct VolBlockSize {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Written {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct ObjSetId {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Atime {
    value: OnOff,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct CanMount {
    value: OnOffNoAuto,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Mounted {
    value: YesNo,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct CheckSum {
    value: CheckSumAlgo,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Compression {
    value: CompressionAlgo,
}

impl Atime {
    pub fn new(value: OnOff) -> Self {
        Self { value }
    }

    // TODO: 1.check mounted
    //       2. implement the same for relative, devices, exec, readonly, setuid, nbmand
    pub fn default(dataset: CString) -> Self {
        let x = core::get_default_numeric_property(sys::zfs_prop_t::ZFS_PROP_ATIME); //unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_ATIME) };
        let mut mnttab: sys::mnttab = unsafe { std::mem::zeroed() };
        let mnttab_ptr: *mut sys::mnttab = &mut mnttab;
        let mut mntent: sys::mntent = unsafe { std::mem::zeroed() };
        let mntent_ptr: *mut sys::mntent = &mut mntent;
        dbg!("I GOT A TIME", x);

        let zfs_handle = unsafe {
            sys::make_dataset_handle(ZFS_HANDLER.lock().unwrap().handler(), dataset.as_ptr())
        };

        let rc = unsafe {
            sys::libzfs_mnttab_find(
                ZFS_HANDLER.lock().unwrap().handler(),
                (*zfs_handle).zfs_name.as_ptr(),
                mnttab_ptr,
            )
        };

        if rc == 0 {
            unsafe {
                (*zfs_handle).zfs_mntopts = sys::zfs_strdup(
                    ZFS_HANDLER.lock().unwrap().handler(),
                    (*mnttab_ptr).mnt_mntopts,
                )
            }

            // TODO: boolean_t already exist in libnvpair
            unsafe { (*zfs_handle).zfs_mntcheck = sys::boolean_t::B_TRUE }
        }

        if unsafe { (*zfs_handle).zfs_mntopts.is_null() } {
            dbg!("zfs mntops is null");
            unsafe { (*mntent_ptr).mnt_opts = std::ptr::null_mut() };
        } else {
            dbg!("zfs mntops is not null");
            unsafe { (*mntent_ptr).mnt_opts = (*zfs_handle).zfs_mntopts }
        }

        if unsafe { !(*mntent_ptr).mnt_opts.is_null() } {
            if unsafe {
                !sys::hasmntopt(
                    mntent_ptr,
                    CString::from_vec_unchecked(b"atime".to_vec()).as_ptr(),
                )
                .is_null()
            } && x == 0
            {
                return Self::new(OnOff::On);
            } else if unsafe {
                !sys::hasmntopt(
                    mntent_ptr,
                    CString::from_vec_unchecked(b"noatime".to_vec()).as_ptr(),
                )
                .is_null()
            } && x != 0
            {
                return Self::new(OnOff::Off);
            }
        }

        Self::new(OnOff::from(x))
    }

    pub fn value(&self) -> OnOff {
        self.value
    }
}

impl Type {
    pub fn new(value: DsType) -> Self {
        Self { value }
    }

    pub fn default() -> Self {
        let x = core::get_default_numeric_property(sys::zfs_prop_t::ZFS_PROP_TYPE);
        dbg!("I GOT available", x);
        Self::new(x.into())
    }

    pub fn value(&self) -> DsType {
        self.value
    }
}

impl Available {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn default() -> Self {
        let x = core::get_default_numeric_property(sys::zfs_prop_t::ZFS_PROP_AVAILABLE);
        dbg!("I GOT available", x);
        Self::new(x)
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl LogicalUsed {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn default() -> Self {
        let x = core::get_default_numeric_property(sys::zfs_prop_t::ZFS_PROP_LOGICALUSED);
        dbg!("I GOT logicalused", x);
        Self::new(x)
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl CanMount {
    pub fn new(value: OnOffNoAuto) -> Self {
        Self { value }
    }

    // TODO: implement the same for volsize, quota, refquota, reservation, refreservation
    //          filesystem_limit, snapshot_limit, filesystem_count, snapshot_count
    pub fn default() -> Self {
        let x = core::get_default_numeric_property(sys::zfs_prop_t::ZFS_PROP_CANMOUNT);
        dbg!("I GOT CanMount", x);
        if x == 1 {
            Self::new(OnOffNoAuto::On)
        } else if x == 0 {
            Self::new(OnOffNoAuto::Off)
        } else {
            Self::new(OnOffNoAuto::NoAuto)
        }
    }

    pub fn value(&self) -> OnOffNoAuto {
        self.value
    }
}

impl Mounted {
    pub fn new(value: YesNo) -> Self {
        Self { value }
    }

    pub(super) fn default(dataset: CString) -> Self {
        let zfs_handle = unsafe {
            sys::make_dataset_handle(ZFS_HANDLER.lock().unwrap().handler(), dataset.as_ptr())
        };

        if unsafe { (*zfs_handle).zfs_mntopts.is_null() } {
            Self::new(YesNo::No)
        } else {
            Self::new(YesNo::Yes)
        }
    }

    pub fn value(&self) -> YesNo {
        self.value
    }
}

impl CheckSum {
    pub fn new(value: CheckSumAlgo) -> Self {
        Self { value }
    }

    // TODO: impl same logic for all indexed properties
    pub fn default() -> Self {
        let x = core::get_default_numeric_property(sys::zfs_prop_t::ZFS_PROP_CHECKSUM);
        dbg!("I GOT Checksum", x);
        Self::new(CheckSumAlgo::from(x))
    }

    pub fn value(&self) -> CheckSumAlgo {
        self.value
    }
}

impl Compression {
    pub fn new(value: CompressionAlgo) -> Self {
        Self { value }
    }

    pub fn default() -> Self {
        let x = core::get_default_numeric_property(sys::zfs_prop_t::ZFS_PROP_COMPRESSION);
        dbg!("I GOT Compression", x);
        Self::new(CompressionAlgo::from(x))
    }

    pub fn value(&self) -> CompressionAlgo {
        self.value
    }
}

impl Guid {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Name {
    pub fn new(value: CString) -> Self {
        Self { value }
    }

    // TODO: remove clone
    pub fn value(&self) -> CString {
        self.value.clone()
    }
}

impl Creation {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl CreateTxg {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl CompressRatio {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Used {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Referenced {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl LogicalReferenced {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl ObjSetId {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}
