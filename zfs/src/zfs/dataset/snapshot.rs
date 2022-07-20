use super::*;

use libzfs::zfs_prop_t::*;

#[derive(Debug)]
pub struct Snapshot {
    dataset: libzfs::ZfsHandle,
}

impl Snapshot {
    pub fn get(name: impl AsRef<str>) -> Result<Self> {
        let cname = ffi::CString::new(name.as_ref())?;
        let dataset = libzfs::ZfsHandle::new(cname)?;

        Ok(Self { dataset })
    }

    pub fn destroy(self) -> Result<()> {
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

#[derive(Debug)]
pub struct SnapshotBuilder {
    props: Result<nvpair::NvList>,
    recursive: bool,
}

impl SnapshotBuilder {
    pub fn new() -> Self {
        let props = Ok(nvpair::NvList::new());
        let recursive = false;
        Self { props, recursive }
    }

    pub fn create(self, name: impl AsRef<str>) -> Result<Snapshot> {
        let _props = self.props?;
        let name = name.as_ref();
        if let Some((dataset, snapshot)) = name.split_once('@') {
            snapshots(dataset, snapshot, self.recursive)?;
        } else {
            Err(DatasetError::invalid_snapshot_name(name))?;
        }
        Snapshot::get(name)
    }

    pub fn recursive(self) -> Self {
        Self {
            recursive: true,
            ..self
        }
    }
}

impl Default for SnapshotBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn snapshots(
    dataset: impl AsRef<str>,
    snapshot: impl AsRef<str>,
    recursive: bool,
) -> Result<()> {
    let snapshot = snapshot.as_ref();
    let snapshots = libzfs::zfs_list_from(dataset)
        .filesystems()
        .volumes()
        .recursive(recursive)
        .get_collection()?
        .into_iter()
        .map(|dataset| format!("{}@{}", dataset.name(), snapshot));

    lzc::create_snapshots(snapshots, None)?;
    Ok(())
}
