#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

//! High level safe wrappers on top of libzfs_core (lzc)
//!

use std::ffi;
use std::os::unix::io::AsRawFd;
use std::ptr;

use razor_libzfscore as lzc;
use razor_nvpair as nvpair;

pub use error::LzcError;

mod error;

/// Create new ZFS filesystem
///
pub fn create_filesystem(
    name: impl AsRef<str>,
    props: impl nvpair::ToNvList,
) -> Result<(), LzcError> {
    create_dataset(name, lzc::lzc_dataset_type::LZC_DATSET_TYPE_ZFS, props)
}

/// Create new ZFS volume
///
pub fn create_volume(name: impl AsRef<str>, props: impl nvpair::ToNvList) -> Result<(), LzcError> {
    create_dataset(name, lzc::lzc_dataset_type::LZC_DATSET_TYPE_ZVOL, props)
}

fn create_dataset(
    name: impl AsRef<str>,
    dataset_type: lzc::lzc_dataset_type,
    props: impl nvpair::ToNvList,
) -> Result<(), LzcError> {
    let cname = cstring(name)?;
    let code = unsafe { lzc::lzc_create(cname.as_ptr(), dataset_type, props.to_nvlist()) };
    LzcError::err(code)
}

/// Create new ZFS snapshot, with optional properties
///
pub fn create_snapshot(
    snapshot: impl AsRef<str>,
    props: impl Into<Option<nvpair::NvList>>,
) -> Result<(), LzcError> {
    create_snapshots([snapshot], props)
}

/// Create multiple ZFS snapshots, with optional properties
///
pub fn create_snapshots(
    snapshots: impl IntoIterator<Item = impl AsRef<str>>,
    props: impl Into<Option<nvpair::NvList>>,
) -> Result<(), LzcError> {
    let props = props.into();
    let mut snaps = nvpair::NvList::new();
    for snapshot in snapshots {
        snaps.add_boolean(snapshot)?;
    }
    let mut errlist = nvpair::NvList::new();
    let code = unsafe {
        let props = props.as_deref().map_or_else(ptr::null_mut, |p| *p);
        lzc::lzc_snapshot(*snaps, props, &mut *errlist)
    };
    LzcError::err(code)
}

/// Check named dataset for existence
///
pub fn dataset_exists(name: impl AsRef<str>) -> bool {
    if let Ok(name) = cstring(name) {
        unsafe { lzc::lzc_exists(name.as_ptr()) }
    } else {
        false
    }
}

/// Destroy named dataset
///
pub fn destroy_dataset(name: impl AsRef<str>) -> Result<(), LzcError> {
    let name = cstring(name)?;
    let code = unsafe { lzc::lzc_destroy(name.as_ptr()) };
    LzcError::err(code)
}

/// Create new ZFS bookmark from named snapshot
///
pub fn create_bookmark(
    snapshot: impl AsRef<str>,
    bookmark: impl AsRef<str>,
) -> Result<(), LzcError> {
    let mut bookmarks = nvpair::NvList::new();
    bookmarks.add_string(bookmark, snapshot)?;
    let code = unsafe { lzc::lzc_bookmark(*bookmarks, &mut ptr::null_mut()) };
    LzcError::err(code)
}

/// Send
///
pub fn send<S, F, U>(source: S, from: Option<F>, file: U) -> Result<(), LzcError>
where
    S: AsRef<str>,
    F: AsRef<str>,
    U: AsRawFd,
{
    let source = cstring(source)?;
    let from = from.map(cstring).transpose()?;
    let flags = lzc::lzc_send_flags::LZC_SEND_FLAG_EMBED_DATA
        | lzc::lzc_send_flags::LZC_SEND_FLAG_LARGE_BLOCK
        | lzc::lzc_send_flags::LZC_SEND_FLAG_COMPRESS;
    let code = unsafe {
        let source = source.as_ptr();
        let from = from.map_or(ptr::null(), |from| from.as_ptr());
        let fd = file.as_raw_fd();
        lzc::lzc_send(source, from, fd, flags)
    };
    LzcError::err(code)
}

/// Send with resume
///
pub fn send_resume<S, F, U>(
    source: S,
    from: Option<F>,
    file: U,
    resumeobj: u64,
    resumeoff: u64,
) -> Result<(), LzcError>
where
    S: AsRef<str>,
    F: AsRef<str>,
    U: AsRawFd,
{
    let source = cstring(source)?;
    let from = from.map(cstring).transpose()?;
    let fd = file.as_raw_fd();
    let flags = lzc::lzc_send_flags::LZC_SEND_FLAG_EMBED_DATA
        | lzc::lzc_send_flags::LZC_SEND_FLAG_LARGE_BLOCK
        | lzc::lzc_send_flags::LZC_SEND_FLAG_COMPRESS;
    let code = unsafe {
        let source = source.as_ptr();
        let from = from.map_or(ptr::null(), |from| from.as_ptr());
        lzc::lzc_send_resume(source, from, fd, flags, resumeobj, resumeoff)
    };
    LzcError::err(code)
}

/// Receive
///
pub fn receive<S, O, U>(
    snapname: S,
    origin: Option<O>,
    force: bool,
    raw: bool,
    file: U,
) -> Result<(), LzcError>
where
    S: AsRef<str>,
    O: AsRef<str>,
    U: AsRawFd,
{
    let snapname = cstring(snapname)?;
    let origin = origin.map(cstring).transpose()?;
    let props = nvpair::NvList::new();
    let fd = file.as_raw_fd();
    let code = unsafe {
        let snapname = snapname.as_ptr();
        let origin = origin.map_or(ptr::null(), |origin| origin.as_ptr());
        lzc::lzc_receive(snapname, *props, origin, force, raw, fd)
    };
    LzcError::err(code)
}

/// Receive with resume
///
pub fn receive_resumable(
    snapname: impl AsRef<str>,
    origin: impl AsRef<str>,
    force: bool,
    raw: bool,
    file: impl AsRawFd,
) -> Result<(), LzcError> {
    let snapname = cstring(snapname)?;
    let props = nvpair::NvList::new();
    let origin = cstring(origin)?;
    let fd = file.as_raw_fd();
    let code = unsafe {
        let snapname = snapname.as_ptr();
        let origin = origin.as_ptr();
        lzc::lzc_receive_resumable(snapname, *props, origin, force, raw, fd)
    };
    LzcError::err(code)
}

/// Sync named zpool
///
pub fn sync_pool(pool: impl AsRef<str>, force: bool) -> Result<(), LzcError> {
    let pool = cstring(pool)?;
    let mut params = nvpair::NvList::new();
    params += ("force", force);
    let rc = unsafe { lzc::lzc_sync(pool.as_ptr(), *params) };
    LzcError::err(rc)
}

#[inline]
fn cstring(text: impl AsRef<str>) -> Result<ffi::CString, ffi::NulError> {
    ffi::CString::new(text.as_ref())
}
