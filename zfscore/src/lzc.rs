use std::borrow::Cow;
use std::ffi;
use std::ptr;

use once_cell::sync::Lazy;
use razor_libnvpair as libnvpair;
use razor_nvpair as nvpair;
use razor_zfscore_sys as sys;

use nvpair::NvListAccess;

pub use sys::zfs_handle_t;
pub use sys::zfs_prop_t;

use super::error::value_or_err;
use super::Result;

static LIBZFS_CORE: Lazy<Lzc> = Lazy::new(Lzc::init);

struct Lzc;

impl Lzc {
    fn init() -> Self {
        let _rc = unsafe { sys::libzfs_core_init() };
        Self
    }

    unsafe fn lzc_create(
        &self,
        name: *const libc::c_char,
        dataset_type: sys::lzc_dataset_type,
        props: *mut libnvpair::nvlist_t,
    ) -> libc::c_int {
        let wkeydata = ptr::null_mut();
        let wkeylen = 0;
        sys::lzc_create(name, dataset_type, props, wkeydata, wkeylen)
    }

    unsafe fn lzc_destroy(&self, name: *const libc::c_char) -> libc::c_int {
        sys::lzc_destroy(name)
    }

    unsafe fn zfs_prop_default_string(&self, property: zfs_prop_t) -> *const libc::c_char {
        sys::zfs_prop_default_string(property)
    }

    unsafe fn zfs_prop_default_numeric(&self, property: zfs_prop_t) -> u64 {
        sys::zfs_prop_default_numeric(property)
    }
}

pub fn create_filesystem(name: impl AsRef<str>, nvl: &nvpair::NvList) -> Result<()> {
    create_dataset(name, sys::lzc_dataset_type::LZC_DATSET_TYPE_ZFS, nvl)
}

pub fn create_volume(name: impl AsRef<str>, nvl: &nvpair::NvList) -> Result<()> {
    create_dataset(name, sys::lzc_dataset_type::LZC_DATSET_TYPE_ZVOL, nvl)
}

fn create_dataset(
    name: impl AsRef<str>,
    dataset_type: sys::lzc_dataset_type,
    nvl: &nvpair::NvList,
) -> Result<()> {
    let cname = ffi::CString::new(name.as_ref())?;
    let name = cname.as_ptr();
    let nvl = nvl.nvl();

    let rc = unsafe { LIBZFS_CORE.lzc_create(name, dataset_type, nvl) };

    value_or_err((), rc)
}

pub fn destroy_dataset(name: impl AsRef<str>) -> Result<()> {
    let cname = ffi::CString::new(name.as_ref())?;
    let name = cname.as_ptr();
    let rc = unsafe { LIBZFS_CORE.lzc_destroy(name) };

    value_or_err((), rc)
}

pub fn zfs_prop_default_string(property: zfs_prop_t) -> Cow<'static, str> {
    unsafe {
        let cstr = LIBZFS_CORE.zfs_prop_default_string(property);
        ffi::CStr::from_ptr(cstr).to_string_lossy()
    }
}

pub fn zfs_prop_default_numeric(property: zfs_prop_t) -> u64 {
    unsafe { LIBZFS_CORE.zfs_prop_default_numeric(property) }
}
