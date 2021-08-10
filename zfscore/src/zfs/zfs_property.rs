use crate::zfs::DatasetError;

use super::property;
use super::sys;
use super::Result;
use super::Zfs;
pub use property::InvalidProperty;
pub use property::Property;

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

pub type Guid = property::Property<u64>;
pub type Name = property::Property<String>;
pub type Available = property::Property<u64>;
pub type CompressRatio = property::Property<u64>;
pub type Type = property::Property<DatasetType>;
pub type Used = property::Property<u64>;
pub type LogicalUsed = property::Property<u64>;
pub type Referenced = property::Property<u64>;
pub type LogicalReferenced = property::Property<u64>;
pub type CreateTxg = property::Property<u64>;
//pub type Creation = property::Property<TimeStamp>;
pub type Creation = property::Property<u64>;
pub type Volsize = property::Property<u64>;
pub type VolBlockSize = property::Property<u64>;
pub type Written = property::Property<u64>;
pub type ObjSetId = property::Property<u64>;
pub type Atime = property::Property<OnOff>;
pub type CanMount = property::Property<OnOffNoAuto>;
pub type Mounted = property::Property<YesNo>;
pub type CheckSum = property::Property<CheckSumAlgo>;
pub type Compression = property::Property<CompressionAlgo>;

// TODO: 1.check mounted
//       2. implement the same for relative, devices, exec, readonly, setuid, nbmand
impl Atime {
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
                            CString::from_vec_unchecked(b"atime".to_vec()).as_ptr() as *mut u8,
                        )
                        .is_null()
                    } && x == 0
                    {
                        return Ok(Atime::new(OnOff::On));
                    } else if unsafe {
                        !sys::hasmntopt(
                            mntent_ptr,
                            CString::from_vec_unchecked(b"noatime".to_vec()).as_ptr() as *mut u8,
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

// TODO: implement the same for volsize, quota, refquota, reservation, refreservation
//          filesystem_limit, snapshot_limit, filesystem_count, snapshot_count
impl CanMount {
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

// TODO: impl same logic for all indexed properties
impl CheckSum {
    pub fn default() -> Result<CheckSum> {
        let x = unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_CHECKSUM) };
        dbg!("I GOT Checksum", x);
        Ok(CheckSum {
            value: x.try_into()?,
        })
    }
}

impl Compression {
    pub fn default() -> Result<Compression> {
        let x = unsafe { sys::zfs_prop_default_numeric(sys::zfs_prop_t::ZFS_PROP_COMPRESSION) };
        dbg!("I GOT Compression", x);
        Ok(Compression {
            value: x.try_into()?,
        })
    }
}
