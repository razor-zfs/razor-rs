use std::ffi::CString;

use serde::ser::{Serialize, SerializeStruct, Serializer};

use super::*;

use crate::error::DatasetError;

use libzfs::zfs_prop_t::*;
use property::*;

#[derive(Debug)]
pub struct Volume {
    dataset: libzfs::ZfsHandle,
}

impl Volume {
    pub fn set(&mut self) -> VolumePropSetter<'_> {
        VolumePropSetter::new(self)
    }

    pub fn destroy(self) -> Result<()> {
        lzc::destroy_dataset(self.name())
    }

    pub fn snapshot(&self, name: impl AsRef<str>) -> Result<()> {
        let snapshot = format!("{}@{}", self.name(), name.as_ref());
        lzc::snapshot(snapshot, None)?;
        Ok(())
    }

    pub fn name(&self) -> String {
        self.dataset.name().to_string()
    }

    pub fn get(name: impl AsRef<str>) -> Result<Self> {
        let cname = CString::new(name.as_ref())?;
        let dataset = libzfs::ZfsHandle::new(cname)?;

        Ok(Self { dataset })
    }

    #[inline]
    pub fn available(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_AVAILABLE)
    }

    #[inline]
    pub fn volsize(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_VOLSIZE)
    }

    #[inline]
    pub fn volblocksize(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_VOLBLOCKSIZE)
    }

    #[inline]
    pub fn logicalused(&self) -> u64 {
        self.dataset.numeric_property(ZFS_PROP_LOGICALUSED)
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
    pub fn volmode(&self) -> property::VolMode {
        self.dataset.numeric_property(ZFS_PROP_VOLMODE).into()
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
}

impl Serialize for Volume {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Volume", 16)?;
        state.serialize_field(&property::NAME, &self.name())?;
        state.serialize_field(&property::AVAILABLE, &self.available())?;
        state.serialize_field(&property::VOLSIZE, &self.volsize())?;
        state.serialize_field(&property::VOLBLOCKSIZE, &self.volblocksize())?;
        state.serialize_field(&property::VOLMODE, &self.volmode())?;
        state.serialize_field(&property::LOGICALUSED, &self.logicalused())?;
        state.serialize_field(&property::CHECKSUM, &self.checksum())?;
        state.serialize_field(&property::COMPRESSION, &self.compression())?;
        state.serialize_field(&property::GUID, &self.guid())?;
        state.serialize_field(&property::CREATION, &self.creation())?;
        state.serialize_field(&property::CREATETXG, &self.createtxg())?;
        state.serialize_field(&property::COMPRESSRATIO, &self.compressratio())?;
        state.serialize_field(&property::USED, &self.used())?;
        state.serialize_field(&property::REFERENCED, &self.referenced())?;
        state.serialize_field(&property::LOGICALREFERENCED, &self.logicalreferenced())?;
        state.serialize_field(&property::OBJSETID, &self.objsetid())?;

        state.end()
    }
}

#[derive(Debug)]
pub struct VolumeBuilder {
    props: Properties,
    volblocksize: u64,
    err: Option<DatasetError>,
}

impl VolumeBuilder {
    pub fn new() -> Self {
        let props = Properties::new();
        let volblocksize = Self::calculate_default_volblocksize();

        Self {
            props,
            volblocksize,
            err: None,
        }
    }

    // TODO: 1. default block size should be calculated
    //       2. volsize should be multiple of volblocksize and rounded to nearest 128k bytes
    //       3. add noreserve functionality
    //       4. add parents creation if needed
    //       5. add zfs_mount_and_share functionality
    pub fn create(mut self, name: impl AsRef<str>, size: u64) -> Result<Volume> {
        #[inline]
        fn _is_power_of_two(num: u64) -> bool {
            (num != 0) && ((num & (num - 1)) == 0)
        }

        if let Some(err) = self.err {
            return Err(err);
        }

        let name = name.as_ref();
        let cname = CString::new(name)?;

        self.props.volsize(size);
        self.props.volblocksize(self.volblocksize);

        lzc::create_volume(name, self.props.into())?;

        let dataset = libzfs::ZfsHandle::new(cname)?;
        let volume = Volume { dataset };

        Ok(volume)
    }

    pub fn checksum(mut self, value: impl Into<property::CheckSum>) -> Self {
        self.props.checksum(value);
        self
    }

    pub fn compression(mut self, value: impl Into<property::Compression>) -> Self {
        self.props.compression(value);
        self
    }

    pub fn blocksize(mut self, v: u64) -> Self {
        self.volblocksize = v;
        self
    }

    // TODO: implement calculation algorithm
    fn calculate_default_volblocksize() -> u64 {
        8192
    }

    pub fn volmode(mut self, value: impl Into<property::VolMode>) -> Self {
        self.props.volmode(value);
        self
    }

    pub fn property(mut self, property: &str, value: &str) -> Self {
        self.props.string_property(property, value);
        self
    }
}

impl Default for VolumeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct VolumePropSetter<'a> {
    volume: &'a mut Volume,
    props: Properties,
}

impl<'a> VolumePropSetter<'a> {
    pub fn new(volume: &'a mut Volume) -> Self {
        Self {
            volume,
            props: Properties::new(),
        }
    }

    pub fn checksum(mut self, value: impl Into<property::CheckSum>) -> Self {
        self.props.checksum(value);
        self
    }

    pub fn compression(mut self, value: impl Into<property::Compression>) -> Self {
        self.props.compression(value);
        self
    }

    pub fn blocksize(mut self, value: u64) -> Self {
        self.props.volblocksize(value);
        self
    }

    pub fn volmode(mut self, value: impl Into<property::VolMode>) -> Self {
        self.props.volmode(value);
        self
    }

    pub fn commit(self) -> Result<()> {
        self.volume.dataset.set_properties(self.props)
    }
}
