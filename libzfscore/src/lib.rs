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
#![allow(clippy::missing_safety_doc)]
#![deny(warnings)]

use std::ptr;

use once_cell::sync::Lazy;

use razor_libnvpair as libnvpair;
use razor_libzfs as libzfs;
use razor_libzfscore_sys as sys;

pub use sys::lzc_dataset_type;
pub use sys::lzc_send_flags;
pub use sys::translate_zfs_error;

pub use error::ZfsError;

mod error;
mod lzc;

pub unsafe fn lzc_create(
    name: *const libc::c_char,
    dataset_type: sys::lzc_dataset_type,
    props: *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    let wkeydata = ptr::null_mut();
    let wkeylen = 0;
    sys::lzc_create(name, dataset_type, props, wkeydata, wkeylen)
}

pub unsafe fn lzc_destroy(name: *const libc::c_char) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_destroy(name)
}

pub unsafe fn lzc_exists(name: *const libc::c_char) -> bool {
    sys::lzc_exists(name).into()
}

pub unsafe fn lzc_snapshot(
    snaps: *mut libnvpair::nvlist_t,
    props: *mut libnvpair::nvlist_t,
    errlist: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_snapshot(snaps, props, errlist)
}

pub unsafe fn lzc_bookmark(
    bookmarks: *mut libnvpair::nvlist_t,
    errlist: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_bookmark(bookmarks, errlist)
}

pub unsafe fn lzc_send(
    snapname: *const libc::c_char,
    from: *const libc::c_char,
    fd: libc::c_int,
    flags: sys::lzc_send_flags,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_send(snapname, from, fd, flags)
}

pub unsafe fn lzc_send_resume(
    snapname: *const libc::c_char,
    from: *const libc::c_char,
    fd: libc::c_int,
    flags: sys::lzc_send_flags,
    resumeobj: u64,
    resumeoff: u64,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_send_resume(snapname, from, fd, flags, resumeobj, resumeoff)
}

pub unsafe fn lzc_receive(
    snapname: *const libc::c_char,
    props: *mut libnvpair::nvlist_t,
    origin: *const libc::c_char,
    force: impl Into<libnvpair::boolean_t>,
    raw: impl Into<libnvpair::boolean_t>,
    fd: libc::c_int,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_receive(snapname, props, origin, force.into(), raw.into(), fd)
}

pub unsafe fn lzc_receive_resumable(
    snapname: *const libc::c_char,
    props: *mut libnvpair::nvlist_t,
    origin: *const libc::c_char,
    force: impl Into<libnvpair::boolean_t>,
    raw: impl Into<libnvpair::boolean_t>,
    fd: libc::c_int,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_receive_resumable(snapname, props, origin, force.into(), raw.into(), fd)
}
