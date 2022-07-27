use serde::ser::{Serialize, SerializeStruct, Serializer};

use super::*;

use libzfs::zfs_prop_t::*;
use property::*;

#[derive(Debug)]
pub struct Filesystem {
    dataset: libzfs::ZfsHandle,
}

impl Filesystem {
    pub fn set(&mut self) -> FilesytemPropSetter<'_> {
        FilesytemPropSetter::new(self)
    }

    pub fn destroy(self) -> Result<()> {
        lzc::destroy_dataset(self.name())?;
        Ok(())
    }

    pub fn snapshot(&self, name: impl AsRef<str>) -> Result<()> {
        let snapshot = format!("{}@{}", self.name(), name.as_ref());
        lzc::create_snapshot(snapshot, None)?;
        Ok(())
    }

    pub fn destroy_recursive(&self) -> Result<()> {
        let ns_datasets = libzfs::zfs_list_from(self.name())
            .filesystems()
            .volumes()
            .snapshots()
            .recursive(true)
            .get_collection();

        for dataset in ns_datasets.into_iter() {
            lzc::destroy_dataset(dataset.name())?;
        }

        lzc::destroy_dataset(self.name())?;
        Ok(())
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
    pub fn canmount(&self) -> property::CanMount {
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
    props: Properties,
}

impl FilesystemBuilder {
    pub fn new() -> Self {
        let props = Properties::new();
        Self { props }
    }

    // TODO: should check mount options and mount the FS if needed
    pub fn create(self, name: impl AsRef<str>) -> Result<Filesystem> {
        let cname = ffi::CString::new(name.as_ref())?;
        libzfs::create_filesystem(name, self.props)?;
        let dataset = libzfs::ZfsHandle::new(cname)?;
        let filesystem = Filesystem { dataset };

        Ok(filesystem)
    }

    pub fn atime(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.atime(value);
        self
    }

    pub fn canmount(mut self, value: impl Into<property::CanMount>) -> Self {
        self.props.canmount(value);
        self
    }

    pub fn checksum(mut self, value: impl Into<property::CheckSum>) -> Self {
        self.props.checksum(value);
        self
    }

    pub fn devices(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.devices(value);
        self
    }

    pub fn nbmand(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.nbmand(value);
        self
    }

    pub fn overlay(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.overlay(value);
        self
    }

    pub fn readonly(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.readonly(value);
        self
    }

    pub fn relatime(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.relatime(value);
        self
    }

    pub fn setuid(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.setuid(value);
        self
    }

    pub fn vscan(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.vscan(value);
        self
    }

    pub fn zoned(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.zoned(value);
        self
    }

    pub fn compression(mut self, value: impl Into<property::Compression>) -> Self {
        self.props.compression(value);
        self
    }

    pub fn exec(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.exec(value);
        self
    }

    pub fn mountpoint(mut self, value: impl AsRef<str>) -> Self {
        self.props.mountpoint(value);
        self
    }

    pub fn property(mut self, property: &str, value: &str) -> Self {
        self.props.string_property(property, value);
        self
    }
}

impl Default for FilesystemBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct FilesytemPropSetter<'a> {
    filesystem: &'a mut Filesystem,
    props: Properties,
}

impl<'a> FilesytemPropSetter<'a> {
    pub fn new(filesystem: &'a mut Filesystem) -> Self {
        Self {
            filesystem,
            props: Properties::new(),
        }
    }

    pub fn atime(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.atime(value);
        self
    }

    pub fn canmount(mut self, value: impl Into<property::CanMount>) -> Self {
        self.props.canmount(value);
        self
    }

    pub fn checksum(mut self, value: impl Into<property::CheckSum>) -> Self {
        self.props.checksum(value);
        self
    }

    pub fn devices(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.devices(value);
        self
    }

    pub fn nbmand(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.nbmand(value);
        self
    }

    pub fn overlay(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.overlay(value);
        self
    }

    pub fn readonly(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.readonly(value);
        self
    }

    pub fn relatime(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.relatime(value);
        self
    }

    pub fn setuid(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.setuid(value);
        self
    }

    pub fn vscan(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.vscan(value);
        self
    }

    pub fn zoned(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.zoned(value);
        self
    }

    pub fn compression(mut self, value: impl Into<property::Compression>) -> Self {
        self.props.compression(value);
        self
    }

    pub fn exec(mut self, value: impl Into<property::OnOff>) -> Self {
        self.props.exec(value);
        self
    }

    pub fn commit(self) -> Result<()> {
        self.filesystem.dataset.set_properties(self.props)?;
        Ok(())
    }
}
