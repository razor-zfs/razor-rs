use std::ffi;
use std::os::unix::io::AsRawFd;
use std::ptr;

use razor_libzfscore as lzc;

use super::*;

/// Create new ZFS filesystem
///
pub fn create_filesystem(name: impl AsRef<str>, props: nvpair::NvList) -> Result<()> {
    create_dataset(name, lzc::lzc_dataset_type::LZC_DATSET_TYPE_ZFS, props)
}

/// Create new ZFS volume
///
pub fn create_volume(name: impl AsRef<str>, props: nvpair::NvList) -> Result<()> {
    create_dataset(name, lzc::lzc_dataset_type::LZC_DATSET_TYPE_ZVOL, props)
}

fn create_dataset(
    name: impl AsRef<str>,
    dataset_type: lzc::lzc_dataset_type,
    props: nvpair::NvList,
) -> Result<()> {
    let cname = cstring(name)?;
    let rc = unsafe { lzc::lzc_create(cname.as_ptr(), dataset_type, *props) };
    value_or_err((), rc)
}

pub fn snapshot(snapshot: impl AsRef<str>) -> Result<()> {
    snapshots_impl([snapshot])
}

pub fn snapshots(
    dataset: impl AsRef<str>,
    snapshot: impl AsRef<str>,
    recursive: bool,
) -> Result<()> {
    let snapshot = snapshot.as_ref();
    let snapshots = zfs_list_from(dataset)
        .filesystems()
        .volumes()
        .recursive(recursive)
        .get_collection()?
        .into_iter()
        .map(|dataset| format!("{}@{}", dataset.name(), snapshot));

    snapshots_impl(snapshots)
}

// TODO Pass props nvlist
fn snapshots_impl(snapshots: impl IntoIterator<Item = impl AsRef<str>>) -> Result<()> {
    let mut snaps = nvpair::NvList::new();
    for snapshot in snapshots {
        snaps.add_boolean(snapshot)?;
    }
    let props = ptr::null_mut();
    let mut errlist = nvpair::NvList::new();
    let rc = unsafe { lzc::lzc_snapshot(*snaps, props, &mut *errlist) };
    value_or_err((), rc)
}

pub fn dataset_exists(name: impl AsRef<str>) -> bool {
    if let Ok(name) = cstring(name) {
        unsafe { lzc::lzc_exists(name.as_ptr()) }
    } else {
        false
    }
}

pub fn destroy_dataset(name: impl AsRef<str>) -> Result<()> {
    let name = cstring(name)?;
    let rc = unsafe { lzc::lzc_destroy(name.as_ptr()) };

    value_or_err((), rc)
}

pub fn bookmark(snapshot: impl AsRef<str>, bookmark: impl AsRef<str>) -> Result<()> {
    let mut bookmarks = nvpair::NvList::new();
    bookmarks.add_string(bookmark, snapshot)?;
    let rc = unsafe { lzc::lzc_bookmark(*bookmarks, &mut ptr::null_mut()) };
    value_or_err((), rc)
}

pub fn send<S, F, U>(source: S, from: Option<F>, file: U) -> Result<()>
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
    let rc = unsafe {
        let source = source.as_ptr();
        let from = from.map_or(ptr::null(), |from| from.as_ptr());
        let fd = file.as_raw_fd();
        lzc::lzc_send(source, from, fd, flags)
    };
    value_or_err((), rc)
}

pub fn send_resume<S, F, U>(
    source: S,
    from: Option<F>,
    file: U,
    resumeobj: u64,
    resumeoff: u64,
) -> Result<()>
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
    let rc = unsafe {
        let source = source.as_ptr();
        let from = from.map_or(ptr::null(), |from| from.as_ptr());
        lzc::lzc_send_resume(source, from, fd, flags, resumeobj, resumeoff)
    };
    value_or_err((), rc)
}

pub fn receive<S, O, U>(
    snapname: S,
    origin: Option<O>,
    force: bool,
    raw: bool,
    file: U,
) -> Result<()>
where
    S: AsRef<str>,
    O: AsRef<str>,
    U: AsRawFd,
{
    let snapname = cstring(snapname)?;
    let origin = origin.map(cstring).transpose()?;
    let props = nvpair::NvList::new();
    let fd = file.as_raw_fd();
    let rc = unsafe {
        let snapname = snapname.as_ptr();
        let origin = origin.map_or(ptr::null(), |origin| origin.as_ptr());
        lzc::lzc_receive(snapname, *props, origin, force, raw, fd)
    };
    value_or_err((), rc)
}

pub fn receive_resumable(
    snapname: impl AsRef<str>,
    origin: impl AsRef<str>,
    force: bool,
    raw: bool,
    file: impl AsRawFd,
) -> Result<()> {
    let snapname = cstring(snapname)?;
    let props = nvpair::NvList::new();
    let origin = cstring(origin)?;
    let fd = file.as_raw_fd();
    let rc = unsafe {
        let snapname = snapname.as_ptr();
        let origin = origin.as_ptr();
        lzc::lzc_receive_resumable(snapname, *props, origin, force, raw, fd)
    };
    value_or_err((), rc)
}

pub fn zfs_list() -> libzfs::DatasetCollectorBuilder {
    libzfs::DatasetCollectorBuilder::new()
}

pub fn zfs_list_from(name: impl AsRef<str>) -> libzfs::DatasetCollectorBuilder {
    libzfs::DatasetCollectorBuilder::from(name)
}

#[inline]
fn cstring(text: impl AsRef<str>) -> Result<ffi::CString, ffi::NulError> {
    ffi::CString::new(text.as_ref())
}
