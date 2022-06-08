use super::*;

use libzfs::zfs_prop_t::*;

#[derive(Debug)]
pub struct Bookmark {
    dataset: libzfs::ZfsHandle,
}

impl Bookmark {
    pub fn get(name: impl AsRef<str>) -> Result<Self> {
        let name = ffi::CString::new(name.as_ref())?;
        let dataset = libzfs::ZfsHandle::new(name)?;

        Ok(Self { dataset })
    }

    pub fn destroy(self) -> Result<()> {
        lzc::destroy_dataset(self.name())
    }

    pub fn name(&self) -> String {
        self.dataset.name().to_string()
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
}
