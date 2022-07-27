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

type PropName = Lazy<Cow<'static, str>>;

pub static TYPE: PropName = Lazy::new(|| prop_name(ZFS_PROP_TYPE));
pub static CREATION: PropName = Lazy::new(|| prop_name(ZFS_PROP_CREATION));
pub static USED: PropName = Lazy::new(|| prop_name(ZFS_PROP_USED));
pub static AVAILABLE: PropName = Lazy::new(|| prop_name(ZFS_PROP_AVAILABLE));
pub static REFERENCED: PropName = Lazy::new(|| prop_name(ZFS_PROP_REFERENCED));
pub static COMPRESSRATIO: PropName = Lazy::new(|| prop_name(ZFS_PROP_COMPRESSRATIO));
pub static MOUNTED: PropName = Lazy::new(|| prop_name(ZFS_PROP_MOUNTED));
pub static ORIGIN: PropName = Lazy::new(|| prop_name(ZFS_PROP_ORIGIN));
pub static QUOTA: PropName = Lazy::new(|| prop_name(ZFS_PROP_QUOTA));
pub static RESERVATION: PropName = Lazy::new(|| prop_name(ZFS_PROP_RESERVATION));
pub static VOLSIZE: PropName = Lazy::new(|| prop_name(ZFS_PROP_VOLSIZE));
pub static VOLBLOCKSIZE: PropName = Lazy::new(|| prop_name(ZFS_PROP_VOLBLOCKSIZE));
pub static RECORDSIZE: PropName = Lazy::new(|| prop_name(ZFS_PROP_RECORDSIZE));
pub static MOUNTPOINT: PropName = Lazy::new(|| prop_name(ZFS_PROP_MOUNTPOINT));
pub static SHARENFS: PropName = Lazy::new(|| prop_name(ZFS_PROP_SHARENFS));
pub static CHECKSUM: PropName = Lazy::new(|| prop_name(ZFS_PROP_CHECKSUM));
pub static COMPRESSION: PropName = Lazy::new(|| prop_name(ZFS_PROP_COMPRESSION));
pub static ATIME: PropName = Lazy::new(|| prop_name(ZFS_PROP_ATIME));
pub static DEVICES: PropName = Lazy::new(|| prop_name(ZFS_PROP_DEVICES));
pub static EXEC: PropName = Lazy::new(|| prop_name(ZFS_PROP_EXEC));
pub static SETUID: PropName = Lazy::new(|| prop_name(ZFS_PROP_SETUID));
pub static READONLY: PropName = Lazy::new(|| prop_name(ZFS_PROP_READONLY));
pub static ZONED: PropName = Lazy::new(|| prop_name(ZFS_PROP_ZONED));

pub static CREATETXG: PropName = Lazy::new(|| prop_name(ZFS_PROP_CREATETXG));
pub static NAME: PropName = Lazy::new(|| prop_name(ZFS_PROP_NAME));
pub static CANMOUNT: PropName = Lazy::new(|| prop_name(ZFS_PROP_CANMOUNT));

pub static REFRESERVATION: PropName = Lazy::new(|| prop_name(ZFS_PROP_REFRESERVATION));
pub static GUID: PropName = Lazy::new(|| prop_name(ZFS_PROP_GUID));

pub static LOGICALUSED: PropName = Lazy::new(|| prop_name(ZFS_PROP_LOGICALUSED));
pub static LOGICALREFERENCED: PropName = Lazy::new(|| prop_name(ZFS_PROP_LOGICALREFERENCED));
pub static OBJSETID: PropName = Lazy::new(|| prop_name(ZFS_PROP_OBJSETID));
pub static VOLMODE: PropName = Lazy::new(|| prop_name(ZFS_PROP_VOLMODE));

pub static NBMAND: PropName = Lazy::new(|| prop_name(ZFS_PROP_NBMAND));
pub static RELATIME: PropName = Lazy::new(|| prop_name(ZFS_PROP_RELATIME));
pub static VSCAN: PropName = Lazy::new(|| prop_name(ZFS_PROP_VSCAN));
pub static OVERLAY: PropName = Lazy::new(|| prop_name(ZFS_PROP_OVERLAY));

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
        self.set_string(ZFS_PROP_COMPRESSION, compression.into());
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
        property: impl Property<'a>,
        value: impl AsRef<str>,
    ) -> &mut Self {
        self.set_string(property, value)
    }

    fn set_string<'a>(&mut self, property: impl Property<'a>, value: impl AsRef<str>) -> &mut Self {
        self.props += (property.name().as_ref(), value.as_ref());
        self
    }

    fn set_numeric<'a>(&mut self, property: impl Property<'a>, value: impl Into<u64>) -> &mut Self {
        self.props += (property.name().as_ref(), value.into());
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

pub trait Property<'a> {
    fn name(self) -> Cow<'a, str>;
}

impl Property<'static> for libzfs::zfs_prop_t {
    fn name(self) -> Cow<'static, str> {
        libzfs::zfs_prop_to_name(self)
    }
}

impl<'a> Property<'a> for String {
    fn name(self) -> Cow<'a, str> {
        self.into()
    }
}

impl<'a> Property<'a> for &'a str {
    fn name(self) -> Cow<'a, str> {
        self.into()
    }
}
