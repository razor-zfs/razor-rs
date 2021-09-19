use chrono::prelude::*;

use crate::libzfs;
use crate::ZfsDatasetHandle;
use razor_zfscore_sys as sys;

#[derive(Debug)]
pub struct DatasetCollectorBuilder {
    datasets: Vec<ZfsDatasetHandle>,
    volumes: Option<bool>,
    filesystems: Option<bool>,
    bookmarks: Option<bool>,
    snapshots: Option<bool>,
    recursive: Option<bool>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    depth: u8,
    current_depth: u8,
}

impl DatasetCollectorBuilder {
    pub(crate) fn new() -> Self {
        Self {
            datasets: Vec::new(),
            volumes: None,
            filesystems: None,
            bookmarks: None,
            snapshots: None,
            start_date: None,
            end_date: None,
            recursive: None,
            depth: 0,
            current_depth: 0,
        }
    }

    pub(crate) fn _filesystems(mut self) -> Self {
        self.filesystems = Some(true);

        self
    }

    pub(crate) fn _volumes(mut self) -> Self {
        self.volumes = Some(true);

        self
    }

    pub(crate) fn _snapshots(mut self) -> Self {
        self.snapshots = Some(true);

        self
    }

    pub(crate) fn _bookmarks(mut self) -> Self {
        self.bookmarks = Some(true);

        self
    }

    pub(crate) fn _recursive(mut self) -> Self {
        self.recursive = Some(true);

        self
    }

    pub(crate) fn _depth(mut self, depth: u8) -> Self {
        self.depth = depth;

        self
    }

    pub(crate) fn _start_time(self) -> Self {
        unimplemented!()
    }

    pub(crate) fn _end_time(self) -> Self {
        unimplemented!()
    }

    fn add_dataset(&mut self, handle: *mut sys::zfs_handle_t) {
        self.datasets.push(ZfsDatasetHandle::from(handle));

        unsafe {
            if sys::zfs_get_type(handle) == sys::zfs_type_t::ZFS_TYPE_FILESYSTEM {
                sys::zfs_iter_filesystems(
                    handle,
                    Some(zfs_list_cb),
                    &mut self.datasets as *mut _ as *mut libc::c_void,
                );
            }
        }
    }

    pub fn get_collection(mut self) -> DatasetCollector {
        unsafe {
            libzfs::zfs_iter_root(
                zfs_list_cb,
                &mut self.datasets as *mut _ as *mut libc::c_void,
            )
        };

        DatasetCollector {
            datasets: self.datasets,
        }
    }
}

#[derive(Debug)]
pub struct DatasetCollector {
    datasets: Vec<ZfsDatasetHandle>,
}

impl DatasetCollector {
    pub(crate) fn new() -> Self {
        Self {
            datasets: Vec::new(),
        }
    }

    pub(crate) fn get_all(mut self) -> Vec<ZfsDatasetHandle> {
        unsafe {
            libzfs::zfs_iter_root(
                zfs_list_cb,
                &mut self.datasets as *mut _ as *mut libc::c_void,
            )
        };

        self.datasets
    }

    pub(crate) fn get_volumes(self) -> Vec<ZfsDatasetHandle> {
        self.get_all()
            .into_iter()
            .filter(|dataset| dataset.is_volume())
            .collect()
    }

    pub(crate) fn get_filesystems(self) -> Vec<ZfsDatasetHandle> {
        self.get_all()
            .into_iter()
            .filter(|dataset| dataset.is_filesystem())
            .collect()
    }

    // fn add_dataset(&mut self, handle: *mut sys::zfs_handle_t) {
    //     self.datasets.push(ZfsDatasetHandle::from(handle));

    //     unsafe {
    //         if sys::zfs_get_type(handle) == sys::zfs_type_t::ZFS_TYPE_FILESYSTEM {
    //             sys::zfs_iter_filesystems(
    //                 handle,
    //                 Some(zfs_list_cb),
    //                 &mut self.datasets as *mut _ as *mut libc::c_void,
    //             );
    //         }
    //     }
    // }
}

#[no_mangle]
extern "C" fn zfs_list_cb(handle: *mut sys::zfs_handle_t, ptr: *mut libc::c_void) -> libc::c_int {
    let data: &mut DatasetCollectorBuilder = unsafe { &mut *(ptr as *mut DatasetCollectorBuilder) };
    data.add_dataset(handle);

    0
}
