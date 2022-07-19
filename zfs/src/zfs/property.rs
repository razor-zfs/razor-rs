use std::borrow::Cow;

use once_cell::sync::Lazy;

use libzfs::zfs_prop_t::*;

use super::*;

pub use error::InvalidProperty;

pub use canmount::CanMount;
pub use checksum::CheckSum;
pub use compression::Compression;
pub use onfoff::OnOff;
pub use volmode::VolMode;
pub use yesno::YesNo;

mod error;

mod canmount;
mod checksum;
mod compression;
mod onfoff;
mod volmode;
mod yesno;

pub static TYPE: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_TYPE));
pub static CREATION: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_CREATION));
pub static USED: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_USED));
pub static AVAILABLE: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_AVAILABLE));
pub static REFERENCED: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_REFERENCED));
pub static COMPRESSRATIO: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_COMPRESSRATIO));
pub static MOUNTED: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_MOUNTED));
pub static ORIGIN: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_ORIGIN));
pub static QUOTA: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_QUOTA));
pub static RESERVATION: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_RESERVATION));
pub static VOLSIZE: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_VOLSIZE));
pub static VOLBLOCKSIZE: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_VOLBLOCKSIZE));
pub static RECORDSIZE: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_RECORDSIZE));
pub static MOUNTPOINT: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_MOUNTPOINT));
pub static SHARENFS: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_SHARENFS));
pub static CHECKSUM: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_CHECKSUM));
pub static COMPRESSION: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_COMPRESSION));
pub static ATIME: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_ATIME));
pub static DEVICES: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_DEVICES));
pub static EXEC: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_EXEC));
pub static SETUID: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_SETUID));
pub static READONLY: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_READONLY));
pub static ZONED: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_ZONED));

pub static CREATETXG: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_CREATETXG));
pub static NAME: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_NAME));
pub static CANMOUNT: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_CANMOUNT));

pub static REFRESERVATION: Lazy<Cow<'static, str>> =
    Lazy::new(|| prop_name(ZFS_PROP_REFRESERVATION));
pub static GUID: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_GUID));

pub static LOGICALUSED: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_LOGICALUSED));
pub static LOGICALREFERENCED: Lazy<Cow<'static, str>> =
    Lazy::new(|| prop_name(ZFS_PROP_LOGICALREFERENCED));
pub static OBJSETID: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_OBJSETID));
pub static VOLMODE: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_VOLMODE));

pub static NBMAND: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_NBMAND));
pub static RELATIME: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_RELATIME));
pub static VSCAN: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_VSCAN));
pub static OVERLAY: Lazy<Cow<'static, str>> = Lazy::new(|| prop_name(ZFS_PROP_OVERLAY));

#[inline]
pub fn prop_name(prop: libzfs::zfs_prop_t) -> Cow<'static, str> {
    libzfs::zfs_prop_to_name(prop)
}

#[derive(Debug)]
pub struct Properties {
    props: nvpair::NvList,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            props: nvpair::NvList::new(),
        }
    }

    pub fn atime(&mut self, atime: impl Into<OnOff>) {
        self.set_numeric(ZFS_PROP_ATIME, atime.into());
    }

    pub fn devices(&mut self, devices: impl Into<OnOff>) {
        self.set_numeric(ZFS_PROP_DEVICES, devices.into());
    }

    pub fn nbmand(&mut self, nbmand: impl Into<OnOff>) {
        self.set_numeric(ZFS_PROP_NBMAND, nbmand.into());
    }

    pub fn overlay(&mut self, overlay: impl Into<OnOff>) {
        self.set_numeric(ZFS_PROP_OVERLAY, overlay.into());
    }

    pub fn readonly(&mut self, readonly: impl Into<OnOff>) {
        self.set_numeric(ZFS_PROP_READONLY, readonly.into());
    }

    pub fn relatime(&mut self, relatime: impl Into<OnOff>) {
        self.set_numeric(ZFS_PROP_RELATIME, relatime.into());
    }

    pub fn setuid(&mut self, setuid: impl Into<OnOff>) {
        self.set_numeric(ZFS_PROP_SETUID, setuid.into());
    }

    pub fn vscan(&mut self, vscan: impl Into<OnOff>) {
        self.set_numeric(ZFS_PROP_VSCAN, vscan.into());
    }

    pub fn zoned(&mut self, zoned: impl Into<OnOff>) {
        self.set_numeric(ZFS_PROP_ZONED, zoned.into());
    }

    pub fn exec(&mut self, exec: impl Into<OnOff>) {
        self.set_numeric(ZFS_PROP_EXEC, exec.into());
    }

    pub fn canmount(&mut self, canmount: impl Into<CanMount>) {
        self.set_numeric(ZFS_PROP_CANMOUNT, canmount.into());
    }

    pub fn checksum(&mut self, checksum: impl Into<CheckSum>) {
        self.set_numeric(ZFS_PROP_CHECKSUM, checksum.into());
    }

    pub fn compression(&mut self, compression: impl Into<Compression>) {
        self.set_numeric(ZFS_PROP_COMPRESSION, compression.into());
    }

    pub fn volblocksize(&mut self, blocksize: u64) {
        self.set_numeric(ZFS_PROP_VOLBLOCKSIZE, blocksize);
    }

    pub fn volmode(&mut self, volmode: impl Into<VolMode>) {
        self.set_numeric(ZFS_PROP_VOLMODE, volmode.into());
    }

    pub fn volsize(&mut self, size: u64) {
        self.set_numeric(ZFS_PROP_VOLSIZE, size);
    }

    pub fn string_property<'a>(
        &mut self,
        property: impl Into<Cow<'a, str>>,
        value: impl AsRef<str>,
    ) -> &mut Self {
        self.props += (property.into().as_ref(), value.as_ref());
        self
    }

    fn set_numeric(
        &mut self,
        property: impl Into<Cow<'static, str>>,
        value: impl Into<u64>,
    ) -> &mut Self {
        self.props += (property.into().as_ref(), value.into());
        self
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Properties> for nvpair::NvList {
    fn from(props: Properties) -> Self {
        props.props
    }
}
