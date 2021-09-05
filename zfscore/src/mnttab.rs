use std::ffi;
use std::mem;

use razor_zfscore_sys as sys;

use super::libzfs_handler::LibZfsHandler;

#[derive(Clone, Debug)]
pub(crate) struct Mnttab {
    fstype: String,
    mntopts: String,
    mountp: String,
    special: String,
}

impl Mnttab {
    pub(crate) fn find(name: impl AsRef<ffi::CStr>) -> Option<Self> {
        let name = name.as_ref();
        let handler = LibZfsHandler::handler();

        unsafe {
            let mut entry = mem::MaybeUninit::uninit();
            if sys::libzfs_mnttab_find(handler, name.as_ptr(), entry.as_mut_ptr()) == 0 {
                Some(Self::from_entry(entry.assume_init()))
            } else {
                None
            }
        }
    }

    pub(crate) fn _fstype(&self) -> &str {
        &self.fstype
    }

    pub(crate) fn mntopts(&self) -> &str {
        &self.mntopts
    }

    pub(crate) fn _mountp(&self) -> &str {
        &self.mountp
    }

    pub(crate) fn _special(&self) -> &str {
        &self.special
    }

    pub(crate) fn hasmntopt(&self, opt: impl AsRef<str>) -> bool {
        let opt = opt.as_ref();
        self.mntopts.split(',').any(|o| o == opt)
    }

    unsafe fn from_entry(entry: sys::mnttab) -> Self {
        let fstype = ffi::CStr::from_ptr(entry.mnt_fstype)
            .to_string_lossy()
            .into_owned();
        let mntopts = ffi::CStr::from_ptr(entry.mnt_mntopts)
            .to_string_lossy()
            .into_owned();
        let mountp = ffi::CStr::from_ptr(entry.mnt_mountp)
            .to_string_lossy()
            .into_owned();
        let special = ffi::CStr::from_ptr(entry.mnt_special)
            .to_string_lossy()
            .into_owned();

        Self {
            fstype,
            mntopts,
            mountp,
            special,
        }
    }
}
