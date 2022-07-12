use std::borrow::Cow;
use std::ffi::CString;

use once_cell::sync::Lazy;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use super::*;

use crate::error::DatasetError;

use libzfs::zfs_prop_t::*;

static AVAILABLE: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_AVAILABLE));
static VOLSIZE: Lazy<Cow<'static, str>> = Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_VOLSIZE));
static VOLBLOCKSIZE: Lazy<Cow<'static, str>> =
    Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_VOLBLOCKSIZE));
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
static VOLMODE: Lazy<Cow<'static, str>> = Lazy::new(|| libzfs::zfs_prop_to_name(ZFS_PROP_VOLMODE));
static NAME: &str = "name";

#[derive(Debug)]
pub struct Volume {
    dataset: libzfs::ZfsHandle,
}

#[derive(Debug)]
pub struct VolumeSetter<'a> {
    volume: &'a Volume,
    nvl: nvpair::NvList,
    err: Option<DatasetError>,
}

impl<'a> VolumeSetter<'a> {
    pub fn new(volume: &'a Volume) -> Self {
        Self {
            volume,
            nvl: nvpair::NvList::new(),
            err: None,
        }
    }

    #[must_use]
    pub fn checksum(mut self, v: impl Into<property::CheckSum>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvl.add_string(CHECKSUM.as_ref(), value.as_str()) {
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
    pub fn blocksize(mut self, v: u64) -> Self {
        let value = v;

        if let Err(err) = self.nvl.add_uint64(VOLBLOCKSIZE.as_ref(), value) {
            self.err = Some(err.into());
        }

        self
    }

    #[must_use]
    pub fn volmode(mut self, v: impl Into<property::VolMode>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvl.add_uint64(VOLMODE.as_ref(), value.into()) {
            self.err = Some(err.into());
        }

        self
    }

    pub fn add(self) -> Result<()> {
        self.volume.dataset.set_properties(self.nvl)?;
        Ok(())
    }
}

impl Volume {
    pub fn set(&self) -> VolumeSetter<'_> {
        VolumeSetter::new(self)
    }

    pub fn destroy(self) -> Result<()> {
        lzc::destroy_dataset(self.name())
    }

    pub fn snapshot(&self, name: impl AsRef<str>) -> Result<()> {
        lzc::snapshot(format!("{}@{}", self.name(), name.as_ref()))?;
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
}

impl Serialize for Volume {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Volume", 16)?;
        state.serialize_field(NAME.as_ref(), &self.name())?;
        state.serialize_field(AVAILABLE.as_ref(), &self.available())?;
        state.serialize_field(VOLSIZE.as_ref(), &self.volsize())?;
        state.serialize_field(VOLBLOCKSIZE.as_ref(), &self.volblocksize())?;
        state.serialize_field(VOLMODE.as_ref(), &self.volmode())?;
        state.serialize_field(LOGICALUSED.as_ref(), &self.logicalused())?;
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

        state.end()
    }
}

#[derive(Debug)]
pub struct VolumeBuilder {
    nvlist: nvpair::NvList,
    volblocksize: u64,
    err: Option<DatasetError>,
}

impl VolumeBuilder {
    pub fn new() -> Self {
        let nvlist = nvpair::NvList::new();
        let volblocksize = Self::calculate_default_volblocksize();

        Self {
            nvlist,
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

        let cname = CString::new(name.as_ref())?;

        if let Some(err) = self.err {
            return Err(err);
        }

        self.nvlist.add_uint64(VOLSIZE.as_ref(), size)?;
        // TODO: check if volblocksize is power of 2 and between 512 and 128000
        self.nvlist
            .add_uint64(VOLBLOCKSIZE.as_ref(), self.volblocksize)?;

        lzc::create_volume(name.as_ref(), self.nvlist)?;

        let dataset = libzfs::ZfsHandle::new(cname)?;
        let volume = Volume { dataset };

        Ok(volume)
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
    pub fn blocksize(mut self, v: u64) -> Self {
        self.volblocksize = v;
        self
    }

    // TODO: implement calculation algorithm
    fn calculate_default_volblocksize() -> u64 {
        8192
    }

    #[must_use]
    pub fn volmode(mut self, v: impl Into<property::VolMode>) -> Self {
        let value = v.into();

        if let Err(err) = self.nvlist.add_uint64(VOLMODE.as_ref(), value.into()) {
            self.err = Some(err.into());
        }

        self
    }
}

impl Default for VolumeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
