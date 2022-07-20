use std::borrow::Cow;

use once_cell::sync::Lazy;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use super::*;

use libzfs::zfs_prop_t::*;

use crate::error::DatasetError;

static AVAILABLE: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_AVAILABLE));
static LOGICALUSED: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_LOGICALUSED));
static CHECKSUM: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_CHECKSUM));
static COMPRESSION: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_COMPRESSION));
static GUID: Lazy<Cow<'static, str>> = Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_GUID));
static CREATION: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_CREATION));
static CREATETXG: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_CREATETXG));
static COMPRESSRATIO: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_COMPRESSRATIO));
static USED: Lazy<Cow<'static, str>> = Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_USED));
static REFERENCED: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_REFERENCED));
static LOGICALREFERENCED: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_LOGICALREFERENCED));
static OBJSETID: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_OBJSETID));
static ATIME: Lazy<Cow<'static, str>> = Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_ATIME));
static CANMOUNT: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_CANMOUNT));
static MOUNTED: Lazy<Cow<'static, str>> = Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_MOUNTED));
static DEVICES: Lazy<Cow<'static, str>> = Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_DEVICES));
static NBMAND: Lazy<Cow<'static, str>> = Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_NBMAND));
static OVERLAY: Lazy<Cow<'static, str>> = Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_OVERLAY));
static READONLY: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_READONLY));
static RELATIME: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_RELATIME));
static SETUID: Lazy<Cow<'static, str>> = Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_SETUID));
static VSCAN: Lazy<Cow<'static, str>> = Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_VSCAN));
static EXEC: Lazy<Cow<'static, str>> = Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_EXEC));
static ZONED: Lazy<Cow<'static, str>> = Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_ZONED));
static NAME: &str = "name";

#[derive(Debug)]
pub struct Filesystem {
    dataset: libzfs::ZfsHandle,
}

#[derive(Debug)]
pub struct FilesytemPropsSetter<'a> {
    filesystem: &'a mut Filesystem,
    nvl: nvpair::NvList,
    err: Option<DatasetError>,
}

impl<'a> FilesytemPropsSetter<'a> {
    pub fn new(filesystem: &'a mut Filesystem) -> Self {
        Self {
            filesystem,
            nvl: nvpair::NvList::new(),
            err: None,
        }
    }

    #[must_use]
    pub fn atime(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvl.add_string(ATIME.as_ref(), value) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn canmount(mut self, v: impl Into<property::OnOffNoAuto>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvl.add_string(CANMOUNT.as_ref(), value) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn checksum(mut self, v: impl Into<property::CheckSum>) -> Self {
        let value = v.into();
        if let Err(err) = self.nvl.add_string(CHECKSUM.as_ref(), value) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn devices(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvl.add_string(DEVICES.as_ref(), value) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn nbmand(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();
        if let Err(err) = self.nvl.add_string(NBMAND.as_ref(), value) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn overlay(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvl.add_string(OVERLAY.as_ref(), value) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn readonly(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvl.add_string(READONLY.as_ref(), value) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn relatime(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvl.add_string(RELATIME.as_ref(), value) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn setuid(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvl.add_string(SETUID.as_ref(), value) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn vscan(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvl.add_string(VSCAN.as_ref(), value) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn zoned(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvl.add_string(ZONED.as_ref(), value) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn compression(mut self, v: impl Into<property::Compression>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvl.add_string(COMPRESSION.as_ref(), value.as_str()) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn exec(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvl.add_string(EXEC.as_ref(), value) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn commit(self) -> Result<()> {
        self.filesystem.dataset.set_properties(self.nvl)?;
        Ok(())
    }
}

impl Filesystem {
    pub fn set(&mut self) -> FilesytemPropsSetter<'_> {
        FilesytemPropsSetter::new(self)
    }

    pub fn destroy(&self) -> Result<()> {
        lzc::destroy_dataset(self.name())
    }

    pub fn snapshot(&self, name: impl AsRef<str>) -> Result<()> {
        let snapshot = format!("{}@{}", self.name(), name.as_ref());
        lzc::snapshot(snapshot, None)
    }

    pub fn destroy_recursive(&self) -> Result<()> {
        let ns_datasets = lzc::zfs_list_from(self.name())
            .filesystems()
            .volumes()
            .snapshots()
            .recursive(true)
            .get_collection()?;

        for dataset in ns_datasets.into_iter() {
            lzc::destroy_dataset(dataset.name())?;
        }

        lzc::destroy_dataset(self.name())
    }

    pub fn name(&self) -> String {
        self.dataset.name().to_string()
    }

    #[inline]
    pub fn available(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_AVAILABLE)
    }

    #[inline]
    pub fn atime(&self) -> property::OnOff {
        self.dataset.numeric_property(ZFS_PROP_ATIME).into()
    }

    #[inline]
    pub fn devices(&self) -> property::OnOff {
        self.dataset.numeric_property(ZFS_PROP_DEVICES).into()
    }

    #[inline]
    pub fn nbmand(&self) -> property::OnOff {
        self.dataset.numeric_property(ZFS_PROP_NBMAND).into()
    }

    #[inline]
    pub fn overlay(&self) -> property::OnOff {
        self.dataset.numeric_property(ZFS_PROP_OVERLAY).into()
    }

    #[inline]
    pub fn readonly(&self) -> property::OnOff {
        self.dataset.numeric_property(ZFS_PROP_READONLY).into()
    }

    #[inline]
    pub fn relatime(&self) -> property::OnOff {
        self.dataset.numeric_property(ZFS_PROP_RELATIME).into()
    }

    #[inline]
    pub fn setuid(&self) -> property::OnOff {
        self.dataset.numeric_property(ZFS_PROP_SETUID).into()
    }

    #[inline]
    pub fn vscan(&self) -> property::OnOff {
        self.dataset.numeric_property(ZFS_PROP_VSCAN).into()
    }

    #[inline]
    pub fn zoned(&self) -> property::OnOff {
        self.dataset.numeric_property(ZFS_PROP_ZONED).into()
    }

    #[inline]
    pub fn exec(&self) -> property::OnOff {
        self.dataset.numeric_property(ZFS_PROP_EXEC).into()
    }

    #[inline]
    pub fn logicalused(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_LOGICALUSED)
    }

    #[inline]
    pub fn canmount(&self) -> property::OnOffNoAuto {
        self.dataset.numeric_property(ZFS_PROP_CANMOUNT).into()
    }

    #[inline]
    pub fn mounted(&self) -> property::YesNo {
        self.dataset.numeric_property(ZFS_PROP_MOUNTED).into()
    }

    #[inline]
    pub fn checksum(&self) -> property::CheckSum {
        self.dataset.numeric_property(ZFS_PROP_CHECKSUM).into()
    }

    #[inline]
    pub fn compression(&self) -> property::Compression {
        self.dataset.numeric_property(ZFS_PROP_COMPRESSION).into()
    }

    #[inline]
    pub fn guid(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_GUID)
    }

    #[inline]
    pub fn creation(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_CREATION)
    }

    #[inline]
    pub fn createtxg(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_CREATETXG)
    }

    #[inline]
    pub fn compressratio(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_COMPRESSRATIO)
    }

    #[inline]
    pub fn used(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_USED)
    }

    #[inline]
    pub fn referenced(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_REFERENCED)
    }

    #[inline]
    pub fn logicalreferenced(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_LOGICALREFERENCED)
    }

    #[inline]
    pub fn objsetid(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_OBJSETID)
    }

    pub fn get(name: impl AsRef<str>) -> Result<Self> {
        let cname = ffi::CString::new(name.as_ref())?;
        let dataset = libzfs::ZfsHandle::new(cname)?;

        Ok(Self { dataset })
    }
}

impl Serialize for Filesystem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        dbg!("serializing filesystem");
        let mut state = serializer.serialize_struct("Filesystem", 25)?;
        state.serialize_field(NAME.as_ref(), &self.name())?;
        state.serialize_field(AVAILABLE.as_ref(), &self.available())?;
        state.serialize_field(ATIME.as_ref(), &self.atime())?;
        state.serialize_field(LOGICALUSED.as_ref(), &self.logicalused())?;
        state.serialize_field(CANMOUNT.as_ref(), &self.canmount())?;
        state.serialize_field(MOUNTED.as_ref(), &self.mounted())?;
        state.serialize_field(DEVICES.as_ref(), &self.devices())?;
        state.serialize_field(OVERLAY.as_ref(), &self.overlay())?;
        state.serialize_field(READONLY.as_ref(), &self.readonly())?;
        state.serialize_field(RELATIME.as_ref(), &self.relatime())?;
        state.serialize_field(SETUID.as_ref(), &self.setuid())?;
        state.serialize_field(VSCAN.as_ref(), &self.vscan())?;
        state.serialize_field(ZONED.as_ref(), &self.zoned())?;
        state.serialize_field(EXEC.as_ref(), &self.exec())?;
        state.serialize_field(NBMAND.as_ref(), &self.nbmand())?;
        state.serialize_field(CHECKSUM.as_ref(), &self.checksum())?;
        state.serialize_field(COMPRESSION.as_ref(), &self.compression())?;
        state.serialize_field(GUID.as_ref(), &self.guid())?;
        state.serialize_field(CREATION.as_ref(), &self.creation())?;
        state.serialize_field(CREATETXG.as_ref(), &self.createtxg())?;
        state.serialize_field(COMPRESSRATIO.as_ref(), &self.compressratio())?;
        state.serialize_field(USED.as_ref(), &self.used())?;
        state.serialize_field(REFERENCED.as_ref(), &self.referenced())?;
        state.serialize_field(LOGICALREFERENCED.as_ref(), &self.logicalreferenced())?;
        state.serialize_field(OBJSETID.as_ref(), &self.objsetid())?;
        dbg!("serializing finished");

        state.end()
    }
}

#[derive(Debug)]
pub struct FilesystemBuilder {
    nvlist: nvpair::NvList,
    err: Option<DatasetError>,
}

impl FilesystemBuilder {
    pub fn new() -> Self {
        let nvlist = nvpair::NvList::new();
        Self { nvlist, err: None }
    }

    // TODO: should check mount options and mount the FS if needed
    pub fn create(self, name: impl AsRef<str>) -> Result<Filesystem> {
        let cname = ffi::CString::new(name.as_ref())?;
        if let Some(err) = self.err {
            return Err(err);
        }

        lzc::create_filesystem(name.as_ref(), self.nvlist)?;

        let dataset = libzfs::ZfsHandle::new(cname)?;
        let filesystem = Filesystem { dataset };

        Ok(filesystem)
    }

    #[must_use]
    pub fn atime(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_uint64(ATIME.as_ref(), u64::from(value)) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn canmount(mut self, v: impl Into<property::OnOffNoAuto>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_uint64(CANMOUNT.as_ref(), u64::from(value)) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn checksum(mut self, v: impl Into<property::CheckSum>) -> Self {
        let value = v.into();
        if let Err(err) = self.nvlist.add_uint64(CHECKSUM.as_ref(), u64::from(value)) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn devices(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_uint64(DEVICES.as_ref(), u64::from(value)) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn nbmand(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();
        if let Err(err) = self.nvlist.add_uint64(NBMAND.as_ref(), u64::from(value)) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn overlay(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_uint64(OVERLAY.as_ref(), u64::from(value)) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn readonly(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_uint64(READONLY.as_ref(), u64::from(value)) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn relatime(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_uint64(RELATIME.as_ref(), u64::from(value)) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn setuid(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_uint64(SETUID.as_ref(), u64::from(value)) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn vscan(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_uint64(VSCAN.as_ref(), u64::from(value)) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn zoned(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_uint64(ZONED.as_ref(), u64::from(value)) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn compression(mut self, v: impl Into<property::Compression>) -> Self {
        let value = v.into();

        if let Err(err) = self
            .nvlist
            .add_uint64(COMPRESSION.as_ref(), u64::from(value))
        {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn exec(mut self, v: impl Into<property::OnOff>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_uint64(EXEC.as_ref(), u64::from(value)) {
            self.err = Some(err.into());
        }

        self
    }
}

impl Default for FilesystemBuilder {
    fn default() -> Self {
        Self::new()
    }
}
