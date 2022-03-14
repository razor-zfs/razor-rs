use anyhow::Result;

use razor_zfs as zfs;
use tokio::task;
use tracing::debug;

use zfs::{zfs_type_t, Zfs, ZfsDatasetHandle};

use crate::zfsrpc_proto::ZfsType;

use super::error::ZfsError;
use super::*;

pub use recv::recv;
pub use send::SendStream;

mod filesystem;
mod recv;
mod send;
mod snapshot;
mod volume;

const FILESYSTEM: &str = "filesystem";
const SNAPSHOT: &str = "snapshot";
const VOLUME: &str = "volume";
const POOL: &str = "pool";
const BOOKMARK: &str = "bookmark";

#[derive(Debug, Default)]
pub struct ZfsRpcService {}

impl ZfsRpcService {
    pub const DEFAULT_BLOCKSIZE: u64 = 8192;
    pub const DEFAULT_CAPACITY: u64 = 100 * 1024 * 1024 * 1024;
}

pub(crate) fn list() -> Result<proto::Datasets, ZfsError> {
    let datasets = Zfs::list()
        .volumes()
        .filesystems()
        .recursive()
        .get_collection()?
        .into_iter()
        .map(proto::Dataset::from)
        .collect();

    let datasets = proto::Datasets { datasets };
    Ok(datasets)
}

pub(crate) fn destroy(name: String) -> Result<(), ZfsError> {
    Zfs::destroy_dataset(name)?;

    Ok(())
}

impl From<ZfsDatasetHandle> for proto::Dataset {
    fn from(ds: ZfsDatasetHandle) -> Self {
        let name = ds.name().to_string();
        let r#type: ZfsType = ds.r#type().into();
        Self {
            name,
            r#type: r#type as i32,
        }
    }
}

impl From<zfs_type_t> for ZfsType {
    fn from(t: zfs_type_t) -> Self {
        match t {
            zfs_type_t::ZFS_TYPE_FILESYSTEM => Self::Filesystem,
            zfs_type_t::ZFS_TYPE_SNAPSHOT => Self::Snapshot,
            zfs_type_t::ZFS_TYPE_VOLUME => Self::Volume,
            zfs_type_t::ZFS_TYPE_POOL => Self::Pool,
            zfs_type_t::ZFS_TYPE_BOOKMARK => Self::Bookmark,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for ZfsType {
    fn from(s: &str) -> Self {
        match s {
            FILESYSTEM => Self::Filesystem,
            SNAPSHOT => Self::Snapshot,
            VOLUME => Self::Volume,
            POOL => Self::Pool,
            BOOKMARK => Self::Bookmark,
            _ => unreachable!(),
        }
    }
}

fn join_to_status(e: task::JoinError) -> tonic::Status {
    tonic::Status::internal(e.to_string())
}

fn zfs_to_status(e: zfs::DatasetError) -> tonic::Status {
    tonic::Status::internal(e.to_string())
}
