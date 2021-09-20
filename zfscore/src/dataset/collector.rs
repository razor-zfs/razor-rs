//use chrono::prelude::*;

use std::ffi::CString;

use super::Result;
use crate::libzfs;
use crate::ZfsDatasetHandle;
use razor_zfscore_sys as sys;

#[derive(Debug)]
pub struct DatasetCollectorBuilder {
    from_dataset: Option<String>,
    datasets: Vec<ZfsDatasetHandle>,
    r#type: libzfs::zfs_type_t,
    recursive: bool,
}

impl DatasetCollectorBuilder {
    pub(crate) fn new() -> Self {
        Self {
            from_dataset: None,
            datasets: Vec::new(),
            r#type: libzfs::zfs_type_t(0),
            recursive: false,
        }
    }

    pub(crate) fn from(dataset: impl AsRef<str>) -> Self {
        Self {
            from_dataset: Some(dataset.as_ref().to_owned()),
            datasets: Vec::new(),
            r#type: libzfs::zfs_type_t(0),
            recursive: false,
        }
    }

    pub fn filesystems(mut self) -> Self {
        self.r#type |= libzfs::zfs_type_t::ZFS_TYPE_FILESYSTEM;

        self
    }

    pub fn volumes(mut self) -> Self {
        self.r#type |= libzfs::zfs_type_t::ZFS_TYPE_VOLUME;

        self
    }

    pub fn _snapshots(mut self) -> Self {
        self.r#type |= libzfs::zfs_type_t::ZFS_TYPE_SNAPSHOT;

        self
    }

    pub fn _bookmarks(mut self) -> Self {
        self.r#type |= libzfs::zfs_type_t::ZFS_TYPE_BOOKMARK;

        self
    }

    pub fn recursive(mut self) -> Self {
        self.recursive = true;

        self
    }

    fn recursive_children(&mut self, handle: Option<&ZfsDatasetHandle>) {
        let childrens = Self::get_children(handle);

        for child in childrens {
            self.recursive_children(Some(&child));
            if child.r#type() & self.r#type != libzfs::zfs_type_t(0) {
                self.datasets.push(child);
            }
        }
    }

    pub fn get_collection(mut self) -> Result<DatasetCollector> {
        let zfs_handle;

        let handle = if let Some(ref name) = self.from_dataset {
            let cname = CString::new(name.as_bytes())?;
            zfs_handle = super::ZfsDatasetHandle::new(cname)?;
            Some(&zfs_handle)
        } else {
            None
        };

        let children = Self::get_children(handle);

        for child in children {
            if self.recursive {
                self.recursive_children(Some(&child))
            }

            if child.r#type() & self.r#type != libzfs::zfs_type_t(0) {
                self.datasets.push(child);
            }
        }

        let collector = DatasetCollector::new(self.datasets);
        Ok(collector)
    }

    pub fn get_children(
        parent: Option<&ZfsDatasetHandle>,
    ) -> impl Iterator<Item = ZfsDatasetHandle> {
        let mut children = Vec::<*mut sys::zfs_handle_t>::new();
        if let Some(parent) = parent {
            unsafe {
                libzfs::zfs_iter_filesystems(
                    parent.handle,
                    zfs_list_cb,
                    &mut children as *mut _ as *mut libc::c_void,
                );
            }
        } else {
            unsafe {
                libzfs::zfs_iter_root(zfs_list_cb, &mut children as *mut _ as *mut libc::c_void)
            };
        }
        children.into_iter().map(ZfsDatasetHandle::from)
    }
}

#[derive(Debug)]
pub struct DatasetCollector {
    datasets: Vec<ZfsDatasetHandle>,
}

impl DatasetCollector {
    pub(crate) fn new(datasets: Vec<ZfsDatasetHandle>) -> Self {
        Self { datasets }
    }
}

impl IntoIterator for DatasetCollector {
    type Item = ZfsDatasetHandle;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.datasets.into_iter()
    }
}

#[no_mangle]
extern "C" fn zfs_list_cb(handle: *mut sys::zfs_handle_t, ptr: *mut libc::c_void) -> libc::c_int {
    let children = unsafe { &mut *(ptr as *mut Vec<*mut sys::zfs_handle_t>) };
    children.push(handle);

    0
}
