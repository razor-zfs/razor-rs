use crate::libzfs;
use crate::ZfsDatasetHandle;
use razor_zfscore_sys as sys;

pub(crate) struct DatasetCollector {
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
}

#[no_mangle]
extern "C" fn zfs_list_cb(handle: *mut sys::zfs_handle_t, ptr: *mut libc::c_void) -> libc::c_int {
    let data: &mut DatasetCollector = unsafe { &mut *(ptr as *mut DatasetCollector) };
    data.add_dataset(handle);

    0
}
