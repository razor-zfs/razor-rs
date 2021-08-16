use std::ffi::CString;

use crate::error::DatasetError;
use crate::zfs::zfs_handler::ZFS_HANDLER;
use serde_nvpair::from_nvlist;

use super::libnvpair;
use super::sys;
use super::Dataset;
use super::FilesystemIntermediate;
use super::Result;

pub struct FileSystemBuilder {
    nvlist: Option<libnvpair::NvList>,
    name: String,
    err: Option<DatasetError>,
}

impl FileSystemBuilder {
    pub fn new(name: impl AsRef<str>) -> Self {
        match libnvpair::NvList::nvlist_alloc(libnvpair::NvFlag::UniqueName) {
            Ok(nvlist) => FileSystemBuilder {
                nvlist: Some(nvlist),
                name: name.as_ref().to_string(),
                err: None,
            },
            Err(error) => FileSystemBuilder {
                nvlist: None,
                name: name.as_ref().to_string(),
                err: Some(error.into()),
            },
        }
    }

    pub fn create(self) -> Result<Dataset> {
        match self.err {
            Some(err) => Err(err),
            None => {
                if let Some(nvlist) = self.nvlist {
                    let ret = unsafe {
                        sys::lzc_create(
                            CString::new(self.name.clone())?.as_ptr(),
                            sys::lzc_dataset_type::LZC_DATSET_TYPE_ZFS,
                            nvlist.raw,
                            std::ptr::null_mut(),
                            0,
                        )
                    };
                    dbg!(ret);

                    if ret != 0 {
                        return Err(DatasetError::DatasetCreationFailure);
                    }

                    let zfs_handle = unsafe {
                        sys::make_dataset_handle(
                            ZFS_HANDLER.lock().unwrap().raw_libzfs_handle,
                            CString::new(self.name.as_ref())?.as_ptr(),
                        )
                    };

                    nvlist = unsafe {
                        libnvpair::NvList {
                            raw: (*zfs_handle).zfs_props,
                        }
                    };

                    let interfs: FilesystemIntermediate = from_nvlist(&mut nvlist)?;

                    filesystem = interfs.convert_to_valid(&self.zfs_handle, &self.name);

                    Dataset {
                        name: self.name,
                        dataset: DatasetType::Filesystem(filesystem),
                    }
                }
            }
        }
    }
}
