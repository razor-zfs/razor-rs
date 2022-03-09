use anyhow::Result;

use razor_zfs as zfs;
use zfs::{zfs_type_t, FileSystemBuilder, Filesystem, VolumeBuilder, Zfs, ZfsDatasetHandle};

use crate::zfsrpc_proto::tonic_zfsrpc::Dataset as DatasetProto;
use crate::zfsrpc_proto::tonic_zfsrpc::Datasets as DatasetsProto;
use crate::zfsrpc_proto::tonic_zfsrpc::Filesystem as ProtoFilesystem;
use crate::zfsrpc_proto::tonic_zfsrpc::{filesystem_property, volume_property};
use crate::zfsrpc_proto::tonic_zfsrpc::{FilesystemProperty, VolumeProperty};

use crate::zfsrpc_proto::Snapshot;
use crate::zfsrpc_proto::Volume;
use crate::zfsrpc_proto::ZfsType;

use tracing::debug;

use super::error::ZfsError;

use super::*;

pub use send::SendStream;

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

pub(crate) fn list() -> Result<DatasetsProto, ZfsError> {
    let datasets = Zfs::list()
        .volumes()
        .filesystems()
        .recursive()
        .get_collection()?
        .into_iter()
        .map(DatasetProto::from)
        .collect();

    let datasets = DatasetsProto { datasets };
    Ok(datasets)
}

pub(crate) fn destroy(name: String) -> Result<(), ZfsError> {
    Zfs::destroy_dataset(name)?;

    Ok(())
}

impl From<ZfsDatasetHandle> for DatasetProto {
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

impl ProtoFilesystem {
    pub(crate) fn add_property(
        fs: FileSystemBuilder,
        property: filesystem_property::Property,
    ) -> Result<FileSystemBuilder, zfs::DatasetError> {
        let fs = match property {
            filesystem_property::Property::ATime(atime) => {
                fs.atime(atime.value.ok_or_else(zfs::DatasetError::missing_value)?)
            }
            filesystem_property::Property::CanMount(canmount) => fs.canmount(
                canmount
                    .value
                    .ok_or_else(zfs::DatasetError::missing_value)?,
            ),
            filesystem_property::Property::Checksum(checksum) => fs.checksum(
                checksum
                    .value
                    .ok_or_else(zfs::DatasetError::missing_value)?,
            ),
            filesystem_property::Property::Compression(compression) => fs.compression(
                compression
                    .value
                    .ok_or_else(zfs::DatasetError::missing_value)?,
            ),
            filesystem_property::Property::Devices(devices) => {
                fs.devices(devices.value.ok_or_else(zfs::DatasetError::missing_value)?)
            }
            filesystem_property::Property::Exec(exec) => {
                fs.exec(exec.value.ok_or_else(zfs::DatasetError::missing_value)?)
            }
            filesystem_property::Property::Nbmand(nbmand) => {
                fs.nbmand(nbmand.value.ok_or_else(zfs::DatasetError::missing_value)?)
            }
            filesystem_property::Property::Overlay(overlay) => {
                fs.overlay(overlay.value.ok_or_else(zfs::DatasetError::missing_value)?)
            }
            filesystem_property::Property::Readonly(readonly) => fs.readonly(
                readonly
                    .value
                    .ok_or_else(zfs::DatasetError::missing_value)?,
            ),
            filesystem_property::Property::Relatime(relatime) => fs.relatime(
                relatime
                    .value
                    .ok_or_else(zfs::DatasetError::missing_value)?,
            ),
            filesystem_property::Property::Setuid(setuid) => {
                fs.setuid(setuid.value.ok_or_else(zfs::DatasetError::missing_value)?)
            }
            filesystem_property::Property::Vscan(vscan) => {
                fs.vscan(vscan.value.ok_or_else(zfs::DatasetError::missing_value)?)
            }
            filesystem_property::Property::Zoned(zoned) => {
                fs.zoned(zoned.value.ok_or_else(zfs::DatasetError::missing_value)?)
            }
            // Dummy for now
            filesystem_property::Property::OnOff(_) => fs,
            filesystem_property::Property::OnOffNoAuto(_) => fs,
        };

        Ok(fs)
    }

    pub(crate) fn create(
        name: String,
        properties: impl IntoIterator<Item = FilesystemProperty>,
    ) -> Result<Self, ZfsError> {
        let builder = Zfs::filesystem();

        let fs = properties
            .into_iter()
            .filter_map(|property| property.property)
            .try_fold(builder, Self::add_property)?
            .create(name)?;

        Ok(fs.into())
    }

    pub(crate) fn get(name: String) -> Result<Self, ZfsError> {
        let fs = Zfs::get_filesystem(&name)?;

        Ok(fs.into())
    }

    pub(crate) async fn mount(name: String, mountpoint: String) -> Result<(), ZfsError> {
        use tokio::process::Command;

        if let Err(out) = Command::new("zfs")
            .arg("set")
            .arg(&format!("mountpoint={}", mountpoint))
            .arg(name.clone())
            .status()
            .await
        {
            return Err(ZfsError::MountFs(out));
        };

        if let Err(out) = Command::new("zfs")
            .arg("mount")
            .arg(name.clone())
            .status()
            .await
        {
            return Err(ZfsError::MountFs(out));
        };

        debug!("Filesystem {} was mounted", name);

        Ok(())
    }

    pub(crate) async fn unmount(name: String) -> Result<(), ZfsError> {
        use tokio::process::Command;

        if let Err(out) = Command::new("zfs")
            .arg("unmount")
            .arg(name.clone())
            .status()
            .await
        {
            return Err(ZfsError::MountFs(out));
        };

        debug!("Filesystem {} was unmounted", name);

        Ok(())
    }
}

impl From<Filesystem> for ProtoFilesystem {
    fn from(fs: Filesystem) -> Self {
        Self {
            name: Some(fs.name().into()),
            available: Some(fs.available().into()),
            atime: Some(fs.atime().into()),
            devices: Some(fs.devices().into()),
            nbmand: Some(fs.nbmand().into()),
            overlay: Some(fs.overlay().into()),
            readonly: Some(fs.readonly().into()),
            relatime: Some(fs.relatime().into()),
            setuid: Some(fs.setuid().into()),
            vscan: Some(fs.vscan().into()),
            zoned: Some(fs.zoned().into()),
            exec: Some(fs.exec().into()),
            canmount: Some(fs.canmount().into()),
            mounted: Some(fs.mounted().into()),
            checksum: Some(fs.checksum().into()),
            compression: Some(fs.compression().into()),
            guid: Some(fs.guid().into()),
            creation: Some(fs.creation().into()),
            createtxg: Some(fs.createtxg().into()),
            compressratio: Some(fs.compressratio().into()),
            used: Some(fs.used().into()),
            logicalused: Some(fs.logicalused().into()),
            referenced: Some(fs.referenced().into()),
            logicalreferenced: Some(fs.logicalreferenced().into()),
            objsetid: Some(fs.objsetid().into()),
        }
    }
}
