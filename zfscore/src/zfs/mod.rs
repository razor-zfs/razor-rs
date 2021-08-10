use std::ffi::CString;

pub use super::error::DatasetError;
use super::libnvpair;
use super::sys;
pub use super::Result;
pub use property::InvalidProperty;

pub mod dataset;
mod property;
pub mod zfs_property;
pub mod zpool_property;

#[derive(Debug)]
pub struct Zfs {
    raw_libzfs_handle: *mut sys::libzfs_handle_t,
    raw_zfs_handle: Option<*mut sys::zfs_handle_t>,
}

impl Zfs {
    pub fn init() -> Self {
        Zfs {
            raw_libzfs_handle: unsafe { sys::libzfs_init() },
            raw_zfs_handle: None,
        }
    }

    pub fn create_dataset_handle<T>(&mut self, name: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        let zfs_handle = unsafe {
            sys::make_dataset_handle(
                self.raw_libzfs_handle,
                CString::new(name.as_ref())?.as_ptr(),
            )
        };

        if zfs_handle.is_null() {
            return Err(DatasetError::DatasetGetError);
        }

        self.raw_zfs_handle = Some(zfs_handle);

        Ok(())
    }

    // TODO: check if dereferenced correctly
    pub fn get_dataset_nvlist(&self) -> Result<libnvpair::NvList> {
        match self.raw_zfs_handle {
            Some(zfs_handle) => Ok(libnvpair::NvList {
                raw: unsafe { (*zfs_handle).zfs_props },
            }),
            None => Err(DatasetError::DatasetGetError),
        }
    }
}

// TODO: check how to free zfs_handle_t
impl Drop for Zfs {
    fn drop(&mut self) {
        unsafe { sys::libzfs_fini(self.raw_libzfs_handle) }
    }
}
