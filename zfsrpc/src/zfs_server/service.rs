use std::convert::TryFrom;
use std::str::FromStr;

use anyhow::Result;
use razor_zfs as zfs;
use tokio::task;
use tracing::debug;

use zfs::{zfs_type_t, Zfs, ZfsDatasetHandle};

use crate::zfsrpc_proto::ZfsType;
use razor_zfs::DatasetError;
use razor_zfs::NvListError;

use super::error::ZfsError;
use super::*;

pub use recv::recv;
pub use send::SendStream;

mod bookmark;
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
        .map(proto::Dataset::try_from)
        .collect::<anyhow::Result<_>>()
        .map_err(|e| DatasetError::NvListError(NvListError::Message(format!("{:?}", e))))
        .map_err(ZfsError::Internal)?;

    let datasets = proto::Datasets { datasets };
    Ok(datasets)
}

pub(crate) fn destroy(name: String) -> Result<(), ZfsError> {
    Zfs::destroy_dataset(name)?;

    Ok(())
}

impl TryFrom<ZfsDatasetHandle> for proto::Dataset {
    type Error = anyhow::Error;
    fn try_from(ds: ZfsDatasetHandle) -> Result<Self, Self::Error> {
        let name = ds.name().to_string();
        let r#type = ZfsType::try_from(ds.r#type())?;
        Ok(Self {
            name,
            r#type: r#type as i32,
        })
    }
}

impl TryFrom<zfs_type_t> for ZfsType {
    type Error = anyhow::Error;
    fn try_from(t: zfs_type_t) -> Result<Self, Self::Error> {
        Ok(match t {
            zfs_type_t::ZFS_TYPE_FILESYSTEM => Self::Filesystem,
            zfs_type_t::ZFS_TYPE_SNAPSHOT => Self::Snapshot,
            zfs_type_t::ZFS_TYPE_VOLUME => Self::Volume,
            zfs_type_t::ZFS_TYPE_POOL => Self::Pool,
            zfs_type_t::ZFS_TYPE_BOOKMARK => Self::Bookmark,
            t => anyhow::bail!("unsupported zfs_type {:?}", t),
        })
    }
}

impl FromStr for ZfsType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            FILESYSTEM => Self::Filesystem,
            SNAPSHOT => Self::Snapshot,
            VOLUME => Self::Volume,
            POOL => Self::Pool,
            BOOKMARK => Self::Bookmark,
            _ => anyhow::bail!(
                "{} is no in {:?}",
                s,
                [FILESYSTEM, SNAPSHOT, VOLUME, POOL, BOOKMARK]
            ),
        })
    }
}

fn join_to_status(e: task::JoinError) -> tonic::Status {
    tonic::Status::internal(e.to_string())
}

fn zfs_to_status(e: zfs::DatasetError) -> tonic::Status {
    tonic::Status::internal(e.to_string())
}
