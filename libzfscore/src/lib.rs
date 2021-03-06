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
#![allow(clippy::too_many_arguments)]
#![deny(warnings)]

use std::ptr;

use once_cell::sync::Lazy;

use razor_libnvpair as libnvpair;
use razor_libzfscore_sys as sys;

pub use sys::dmu_replay_record;
pub use sys::lzc_dataset_type;
pub use sys::lzc_send_flags;
pub use sys::pool_initialize_func_t;
pub use sys::pool_trim_func_t;

#[cfg(feature = "wait")]
pub use sys::zfs_wait_activity_t;

mod lzc;

pub unsafe fn lzc_snapshot(
    snaps: *mut libnvpair::nvlist_t,
    props: *mut libnvpair::nvlist_t,
    errlist: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_snapshot(snaps, props, errlist)
}

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

pub unsafe fn lzc_clone(
    fsname: *const libc::c_char,
    origin: *const libc::c_char,
    props: *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_clone(fsname, origin, props)
}

pub unsafe fn lzc_promote(
    fsname: *const libc::c_char,
    snapnamebuf: *mut libc::c_char,
    snapnamelen: libc::c_int,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_promote(fsname, snapnamebuf, snapnamelen)
}

pub unsafe fn lzc_destroy_snaps(
    snaps: *mut libnvpair::nvlist_t,
    defer: impl Into<libnvpair::boolean_t>,
    errlist: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    let defer = defer.into();
    sys::lzc_destroy_snaps(snaps, defer, errlist)
}

pub unsafe fn lzc_bookmark(
    bookmarks: *mut libnvpair::nvlist_t,
    errlist: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_bookmark(bookmarks, errlist)
}

pub unsafe fn lzc_get_bookmarks(
    fsname: *const libc::c_char,
    props: *mut libnvpair::nvlist_t,
    bookmarks: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_get_bookmarks(fsname, props, bookmarks)
}

pub unsafe fn lzc_get_bookmark_props(
    bookmark: *const libc::c_char,
    props: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_get_bookmark_props(bookmark, props)
}

pub unsafe fn lzc_destroy_bookmarks(
    bookmarks: *mut libnvpair::nvlist_t,
    errlist: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_destroy_bookmarks(bookmarks, errlist)
}

pub unsafe fn lzc_load_key(
    fsname: *const libc::c_char,
    noop: impl Into<libnvpair::boolean_t>,
    wkeydata: *mut u8,
    wkeylen: libc::c_uint,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    let noop = noop.into();
    sys::lzc_load_key(fsname, noop, wkeydata, wkeylen)
}

pub unsafe fn lzc_unload_key(fsname: *const libc::c_char) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_unload_key(fsname)
}

pub unsafe fn lzc_change_key(
    fsname: *const libc::c_char,
    cryptcmd: u64,
    props: *mut libnvpair::nvlist_t,
    wkeydata: *mut u8,
    wkeylen: libc::c_uint,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_change_key(fsname, cryptcmd, props, wkeydata, wkeylen)
}

pub unsafe fn lzc_initialize(
    poolname: *const libc::c_char,
    cmd_type: pool_initialize_func_t,
    vdevs: *mut libnvpair::nvlist_t,
    errlist: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_initialize(poolname, cmd_type, vdevs, errlist)
}

pub unsafe fn lzc_trim(
    poolname: *const libc::c_char,
    cmd_type: pool_trim_func_t,
    rate: u64,
    secure: impl Into<libnvpair::boolean_t>,
    vdevs: *mut libnvpair::nvlist_t,
    errlist: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    let secure = secure.into();
    sys::lzc_trim(poolname, cmd_type, rate, secure, vdevs, errlist)
}

pub unsafe fn lzc_redact(
    snapshot: *const libc::c_char,
    bookname: *const libc::c_char,
    snapnv: *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_redact(snapshot, bookname, snapnv)
}

pub unsafe fn lzc_snaprange_space(
    firstsnap: *const libc::c_char,
    lastsnap: *const libc::c_char,
    usedp: *mut u64,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_snaprange_space(firstsnap, lastsnap, usedp)
}

pub unsafe fn lzc_hold(
    holds: *mut libnvpair::nvlist_t,
    cleanup_fd: libc::c_int,
    errlist: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_hold(holds, cleanup_fd, errlist)
}

pub unsafe fn lzc_release(
    holds: *mut libnvpair::nvlist_t,
    errlist: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_release(holds, errlist)
}

pub unsafe fn lzc_get_holds(
    snapname: *const libc::c_char,
    holdsp: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_get_holds(snapname, holdsp)
}

pub unsafe fn lzc_destroy(name: *const libc::c_char) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_destroy(name)
}

pub unsafe fn lzc_exists(name: *const libc::c_char) -> bool {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_exists(name).into()
}

pub unsafe fn lzc_send(
    snapname: *const libc::c_char,
    from: *const libc::c_char,
    fd: libc::c_int,
    flags: lzc_send_flags,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_send(snapname, from, fd, flags)
}

pub unsafe fn lzc_send_resume(
    snapname: *const libc::c_char,
    from: *const libc::c_char,
    fd: libc::c_int,
    flags: lzc_send_flags,
    resumeobj: u64,
    resumeoff: u64,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_send_resume(snapname, from, fd, flags, resumeobj, resumeoff)
}

pub unsafe fn lzc_send_space(
    snapname: *const libc::c_char,
    from: *const libc::c_char,
    flags: lzc_send_flags,
    spacep: *mut u64,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_send_space(snapname, from, flags, spacep)
}

pub unsafe fn lzc_send_redacted(
    snapname: *const libc::c_char,
    from: *const libc::c_char,
    fd: libc::c_int,
    flags: lzc_send_flags,
    redactbook: *const libc::c_char,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_send_redacted(snapname, from, fd, flags, redactbook)
}

pub unsafe fn lzc_send_resume_redacted(
    snapname: *const libc::c_char,
    from: *const libc::c_char,
    fd: libc::c_int,
    flags: lzc_send_flags,
    resumeobj: u64,
    resumeoff: u64,
    redactbook: *const libc::c_char,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_send_resume_redacted(snapname, from, fd, flags, resumeobj, resumeoff, redactbook)
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

pub unsafe fn lzc_receive_with_header(
    snapname: *const libc::c_char,
    props: *mut libnvpair::nvlist_t,
    origin: *const libc::c_char,
    force: impl Into<libnvpair::boolean_t>,
    resumable: impl Into<libnvpair::boolean_t>,
    raw: impl Into<libnvpair::boolean_t>,
    fd: libc::c_int,
    begin_record: *const dmu_replay_record,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    let force = force.into();
    let resumable = resumable.into();
    let raw = raw.into();
    sys::lzc_receive_with_header(
        snapname,
        props,
        origin,
        force,
        resumable,
        raw,
        fd,
        begin_record,
    )
}

pub unsafe fn lzc_receive_one(
    snapname: *const libc::c_char,
    props: *mut libnvpair::nvlist_t,
    origin: *const libc::c_char,
    force: impl Into<libnvpair::boolean_t>,
    resumable: impl Into<libnvpair::boolean_t>,
    raw: impl Into<libnvpair::boolean_t>,
    input_fd: libc::c_int,
    begin_record: *const dmu_replay_record,
    cleanup_fd: libc::c_int,
    read_bytes: *mut u64,
    errflags: *mut u64,
    action_handle: *mut u64,
    errors: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    let force = force.into();
    let resumable = resumable.into();
    let raw = raw.into();
    sys::lzc_receive_one(
        snapname,
        props,
        origin,
        force,
        resumable,
        raw,
        input_fd,
        begin_record,
        cleanup_fd,
        read_bytes,
        errflags,
        action_handle,
        errors,
    )
}

pub unsafe fn lzc_receive_with_cmdprops(
    snapname: *const libc::c_char,
    props: *mut libnvpair::nvlist_t,
    cmdprops: *mut libnvpair::nvlist_t,
    wkeydata: *mut u8,
    wkeylen: libc::c_uint,
    origin: *const libc::c_char,
    force: impl Into<libnvpair::boolean_t>,
    resumable: impl Into<libnvpair::boolean_t>,
    raw: impl Into<libnvpair::boolean_t>,
    input_fd: libc::c_int,
    begin_record: *const dmu_replay_record,
    cleanup_fd: libc::c_int,
    read_bytes: *mut u64,
    errflags: *mut u64,
    action_handle: *mut u64,
    errors: *mut *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    let force = force.into();
    let resumable = resumable.into();
    let raw = raw.into();
    sys::lzc_receive_with_cmdprops(
        snapname,
        props,
        cmdprops,
        wkeydata,
        wkeylen,
        origin,
        force,
        resumable,
        raw,
        input_fd,
        begin_record,
        cleanup_fd,
        read_bytes,
        errflags,
        action_handle,
        errors,
    )
}

pub unsafe fn lzc_send_space_resume_redacted(
    snapname: *const libc::c_char,
    from: *const libc::c_char,
    flags: lzc_send_flags,
    resumeobj: u64,
    resumeoff: u64,
    resume_bytes: u64,
    redactbook: *const libc::c_char,
    fd: libc::c_int,
    spacep: *mut u64,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_send_space_resume_redacted(
        snapname,
        from,
        flags,
        resumeobj,
        resumeoff,
        resume_bytes,
        redactbook,
        fd,
        spacep,
    )
}

pub unsafe fn lzc_rollback(
    fsname: *const libc::c_char,
    snapnamebuf: *mut libc::c_char,
    snapnamelen: libc::c_int,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_rollback(fsname, snapnamebuf, snapnamelen)
}

pub unsafe fn lzc_rollback_to(
    fsname: *const libc::c_char,
    snapname: *const libc::c_char,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_rollback_to(fsname, snapname)
}

pub unsafe fn lzc_rename(source: *const libc::c_char, target: *const libc::c_char) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_rename(source, target)
}

pub unsafe fn lzc_sync(
    pool_name: *const libc::c_char,
    params: *mut libnvpair::nvlist_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_sync(pool_name, params, ptr::null_mut())
}

pub unsafe fn lzc_reopen(
    pool_name: *const libc::c_char,
    scrub_restart: impl Into<libnvpair::boolean_t>,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    let scrub_restart = scrub_restart.into();
    sys::lzc_reopen(pool_name, scrub_restart)
}

pub unsafe fn lzc_pool_checkpoint(pool: *const libc::c_char) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_pool_checkpoint(pool)
}

pub unsafe fn lzc_pool_checkpoint_discard(pool: *const libc::c_char) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_pool_checkpoint_discard(pool)
}

#[cfg(feature = "wait")]
pub unsafe fn lzc_wait_fs(
    name: *const libc::c_char,
    activity: zfs_wait_activity_t,
    waited: *mut libnvpair::boolean_t,
) -> libc::c_int {
    Lazy::force(&lzc::LIBZFS_CORE);
    sys::lzc_wait_fs(name, activity, waited)
}
