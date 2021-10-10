use anyhow::Result;

use razor_zfs::{
    zfs::FileSystemBuilder, zfs::VolumeBuilder, zfs::Zfs, zfs_type_t, ZfsDatasetHandle,
};

use crate::zfsrpc_proto::tonic_zfsrpc::{dataset, filesystem_property, volume_property};
use crate::zfsrpc_proto::tonic_zfsrpc::{
    Dataset as DatasetProto, Datasets as DatasetsProto, Filesystem as ProtoFilesystem,
    FilesystemProperty, Volume as ProtoVolume, VolumeProperty,
};
use crate::zfsrpc_proto::PropErr;

#[allow(unused)]
use tracing::{debug, error, info, trace, warn};

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

pub(crate) fn list() -> Result<DatasetsProto> {
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

pub(crate) fn destroy(name: String) -> Result<()> {
    Zfs::destroy_dataset(name)?;

    Ok(())
}

impl From<ZfsDatasetHandle> for DatasetProto {
    fn from(ds: ZfsDatasetHandle) -> Self {
        let name = ds.name().to_string();
        let r#type: dataset::Type = ds.r#type().into();
        Self {
            name,
            r#type: r#type as i32,
        }
    }
}

impl From<zfs_type_t> for dataset::Type {
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

impl From<&str> for dataset::Type {
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

impl ProtoVolume {
    fn add_property(
        vol: VolumeBuilder,
        property: volume_property::Property,
    ) -> Result<VolumeBuilder, PropErr> {
        let vol = match property {
            volume_property::Property::Checksum(property) => {
                vol.checksum(property.value.ok_or(PropErr::InvalidArgument)?)
            }
            volume_property::Property::Compression(property) => {
                vol.compression(property.value.ok_or(PropErr::InvalidArgument)?)
            }
            volume_property::Property::VolMode(property) => {
                vol.volmode(property.value.ok_or(PropErr::InvalidArgument)?)
            }
        };

        Ok(vol)
    }
}

impl ProtoVolume {
    pub(crate) fn get(name: String) -> Result<Self> {
        let volume = Zfs::get_volume(&name)?;

        let vol = Self {
            name: Some(volume.name().into()),
            available: Some(volume.available().into()),
            volsize: Some(volume.volsize().into()),
            blocksize: Some(volume.volblocksize().into()),
            logicalused: Some(volume.logicalused().into()),
            checksum: Some(volume.checksum().into()),
            compression: Some(volume.compression().into()),
            guid: Some(volume.guid().into()),
            creation: Some(volume.creation().into()),
            createtxg: Some(volume.createtxg().into()),
            compressratio: Some(volume.compressratio().into()),
            used: Some(volume.used().into()),
            referenced: Some(volume.referenced().into()),
            logicalreferenced: Some(volume.logicalreferenced().into()),
            objsetid: Some(volume.objsetid().into()),
            volmode: Some(volume.volmode().into()),
        };

        Ok(vol)
    }

    pub(crate) fn create(
        name: String,
        capacity: u64,
        blocksize: u64,
        properties: impl IntoIterator<Item = VolumeProperty>,
    ) -> Result<(), PropErr> {
        let builder = Zfs::volume();

        let _volume = properties
            .into_iter()
            .filter_map(|property| property.property)
            .try_fold(builder, Self::add_property)?
            .blocksize(blocksize)
            .create(name, capacity)?;

        Ok(())
    }
}

impl ProtoFilesystem {
    pub(crate) fn add_property(
        fs: FileSystemBuilder,
        property: filesystem_property::Property,
    ) -> Result<FileSystemBuilder, PropErr> {
        let fs = match property {
            filesystem_property::Property::ATime(atime) => {
                fs.atime(atime.value.ok_or(PropErr::InvalidArgument)?)
            }
            filesystem_property::Property::CanMount(canmount) => {
                fs.canmount(canmount.value.ok_or(PropErr::InvalidArgument)?)
            }
            filesystem_property::Property::Checksum(checksum) => {
                fs.checksum(checksum.value.ok_or(PropErr::InvalidArgument)?)
            }
            filesystem_property::Property::Compression(compression) => {
                fs.compression(compression.value.ok_or(PropErr::InvalidArgument)?)
            }
            filesystem_property::Property::Devices(devices) => {
                fs.devices(devices.value.ok_or(PropErr::InvalidArgument)?)
            }
            filesystem_property::Property::Exec(exec) => {
                fs.exec(exec.value.ok_or(PropErr::InvalidArgument)?)
            }
            filesystem_property::Property::Nbmand(nbmand) => {
                fs.nbmand(nbmand.value.ok_or(PropErr::InvalidArgument)?)
            }
            filesystem_property::Property::Overlay(overlay) => {
                fs.overlay(overlay.value.ok_or(PropErr::InvalidArgument)?)
            }
            filesystem_property::Property::Readonly(readonly) => {
                fs.readonly(readonly.value.ok_or(PropErr::InvalidArgument)?)
            }
            filesystem_property::Property::Relatime(relatime) => {
                fs.relatime(relatime.value.ok_or(PropErr::InvalidArgument)?)
            }
            filesystem_property::Property::Setuid(setuid) => {
                fs.setuid(setuid.value.ok_or(PropErr::InvalidArgument)?)
            }
            filesystem_property::Property::Vscan(vscan) => {
                fs.vscan(vscan.value.ok_or(PropErr::InvalidArgument)?)
            }
            filesystem_property::Property::Zoned(zoned) => {
                fs.zoned(zoned.value.ok_or(PropErr::InvalidArgument)?)
            }
        };

        Ok(fs)
    }
}

impl ProtoFilesystem {
    pub(crate) fn create(
        name: String,
        _unused: u64,
        properties: impl IntoIterator<Item = FilesystemProperty>,
    ) -> Result<(), PropErr> {
        let builder = Zfs::filesystem();

        let _fs = properties
            .into_iter()
            .filter_map(|property| property.property)
            .try_fold(builder, Self::add_property)?
            .create(name)?;

        Ok(())
    }

    pub(crate) fn get(name: String) -> Result<Self> {
        let fs = Zfs::get_filesystem(&name)?;

        let fs = Self {
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
        };

        Ok(fs)
    }
}
