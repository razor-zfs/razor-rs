use crate::zfs::DatasetError;

use super::property;
use super::sys;
use super::Result;
use super::Zfs;
pub use property::InvalidProperty;
pub use property::Property;

use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::ffi::CString;

mod checksum;
mod compression;
mod dataset;
mod onoff;
mod onoffnoauto;
mod timestamp;
mod yesno;

pub use checksum::CheckSum as CheckSumAlgo;
pub use compression::Compression as CompressionAlgo;
pub use dataset::Type as DatasetType;
pub use onoff::OnOff;
pub use onoffnoauto::OnOffNoAuto;
pub use timestamp::TimeStamp;
pub use yesno::YesNo;

// pub type Guid = property::Property<u64>;
// pub type Name = property::Property<String>;
// pub type Available = property::Property<u64>;
// pub type CompressRatio = property::Property<u64>;
// pub type Type = property::Property<DatasetType>;
// pub type Used = property::Property<u64>;
// pub type LogicalUsed = property::Property<u64>;
// pub type Referenced = property::Property<u64>;
// pub type LogicalReferenced = property::Property<u64>;
// pub type CreateTxg = property::Property<u64>;
// //pub type Creation = property::Property<TimeStamp>;
// pub type Creation = property::Property<u64>;
// pub type Volsize = property::Property<u64>;
// pub type VolBlockSize = property::Property<u64>;
// pub type Written = property::Property<u64>;
// pub type ObjSetId = property::Property<u64>;
// pub type Atime = property::Property<OnOff>;
// pub type CanMount = property::Property<OnOffNoAuto>;
// pub type Mounted = property::Property<YesNo>;
// pub type CheckSum = property::Property<CheckSumAlgo>;
// pub type Compression = property::Property<CompressionAlgo>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Guid {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Name {
    value: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Available {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CompressRatio {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Type(DatasetType);
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Used {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LogicalUsed {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Referenced {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LogicalReferenced {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CreateTxg {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Creation {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Volsize {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct VolBlockSize {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Written {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ObjSetId {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Atime {
    value: OnOff,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CanMount {
    value: OnOffNoAuto,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Mounted {
    value: YesNo,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CheckSum {
    value: CheckSumAlgo,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Compression {
    value: CompressionAlgo,
}

impl Atime {
    pub fn new(value: OnOff) -> Self {
        Atime { value }
    }

    // TODO: 1.check mounted
    //       2. implement the same for relative, devices, exec, readonly, setuid, nbmand
    pub fn default(zfs: &Zfs) -> Result<Atime> {
        let x = unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_ATIME) };
        let mut mnttab: sys::mnttab = unsafe { std::mem::zeroed() };
        let mnttab_ptr: *mut sys::mnttab = &mut mnttab;
        let mut mntent: sys::mntent = unsafe { std::mem::zeroed() };
        let mntent_ptr: *mut sys::mntent = &mut mntent;
        dbg!("I GOT A TIME", x);

        match zfs.raw_zfs_handle {
            Some(zfs_handle) => {
                let rc = unsafe {
                    sys::libzfs_mnttab_find(
                        zfs.raw_libzfs_handle,
                        (*zfs_handle).zfs_name.as_ptr(),
                        mnttab_ptr,
                    )
                };

                if rc == 0 {
                    unsafe {
                        (*zfs_handle).zfs_mntopts =
                            sys::zfs_strdup(zfs.raw_libzfs_handle, (*mnttab_ptr).mnt_mntopts)
                    }
                    if unsafe { (*zfs_handle).zfs_mntopts.is_null() } {
                        // TODO: change this from unknown
                        return Err(DatasetError::Unknown);
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
                        return Ok(Atime::new(OnOff::On));
                    } else if unsafe {
                        !sys::hasmntopt(
                            mntent_ptr,
                            CString::from_vec_unchecked(b"noatime".to_vec()).as_ptr(),
                        )
                        .is_null()
                    } && x != 0
                    {
                        return Ok(Atime::new(OnOff::Off));
                    }
                }

                Ok(Atime::new(x.try_into()?))
            }
            None => Err(DatasetError::DatasetGetError),
        }
    }
}

impl Available {
    pub fn new(value: u64) -> Self {
        Available { value }
    }

    pub fn default() -> Available {
        let x = unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_AVAILABLE) };
        dbg!("I GOT available", x);
        Available::new(x)
    }
}

impl LogicalUsed {
    pub fn new(value: u64) -> Self {
        LogicalUsed { value }
    }

    pub fn default() -> LogicalUsed {
        let x = unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_LOGICALUSED) };
        dbg!("I GOT logicalused", x);
        LogicalUsed::new(x)
    }
}

impl CanMount {
    pub fn new(value: OnOffNoAuto) -> Self {
        CanMount { value }
    }

    // TODO: implement the same for volsize, quota, refquota, reservation, refreservation
    //          filesystem_limit, snapshot_limit, filesystem_count, snapshot_count
    pub fn default() -> CanMount {
        let x = unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_CANMOUNT) };
        dbg!("I GOT CanMount", x);
        if x == 1 {
            CanMount::new(OnOffNoAuto::On)
        } else if x == 0 {
            CanMount::new(OnOffNoAuto::Off)
        } else {
            CanMount::new(OnOffNoAuto::NoAuto)
        }
    }
}

impl Mounted {
    pub fn new(value: YesNo) -> Self {
        Mounted { value }
    }

    pub fn default(zfs: &Zfs) -> Result<Mounted> {
        match zfs.raw_zfs_handle {
            Some(zfs_handle) => {
                if unsafe { (*zfs_handle).zfs_mntopts.is_null() } {
                    Ok(Mounted::new(YesNo::No))
                } else {
                    Ok(Mounted::new(YesNo::Yes))
                }
            }
            None => Err(DatasetError::DatasetGetError),
        }
    }
}

impl CheckSum {
    pub fn new(value: CheckSumAlgo) -> Self {
        CheckSum { value }
    }

    // TODO: impl same logic for all indexed properties
    pub fn default() -> Result<CheckSum> {
        let x = unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_CHECKSUM) };
        dbg!("I GOT Checksum", x);
        Ok(CheckSum::new(x.try_into()?))
    }
}

impl Compression {
    pub fn new(value: CompressionAlgo) -> Self {
        Compression { value }
    }

    pub fn default() -> Result<Compression> {
        let x = unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_COMPRESSION) };
        dbg!("I GOT Compression", x);
        Ok(Compression::new(x.try_into()?))
    }
}

impl Guid {
    pub fn new(value: u64) -> Self {
        Guid { value }
    }
}

impl Creation {
    pub fn new(value: u64) -> Self {
        Creation { value }
    }
}

impl CreateTxg {
    pub fn new(value: u64) -> Self {
        CreateTxg { value }
    }
}

impl CompressRatio {
    pub fn new(value: u64) -> Self {
        CompressRatio { value }
    }
}

impl Used {
    pub fn new(value: u64) -> Self {
        Used { value }
    }
}

impl Referenced {
    pub fn new(value: u64) -> Self {
        Referenced { value }
    }
}

impl LogicalReferenced {
    pub fn new(value: u64) -> Self {
        LogicalReferenced { value }
    }
}

impl ObjSetId {
    pub fn new(value: u64) -> Self {
        ObjSetId { value }
    }
}
