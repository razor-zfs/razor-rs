use crate::zfsrpc_proto::tonic_zfsrpc::{dataset, filesystem_property, volume_property};
use crate::zfsrpc_proto::tonic_zfsrpc::{
    Dataset as DatasetProto, Datasets as DatasetsProto, Filesystem as ProtoFilesystem,
    FilesystemProperty, Volume as ProtoVolume, VolumeProperty,
};
use crate::zfsrpc_proto::PropErr;
use razor_zfs::{zfs::FileSystemBuilder, zfs::VolumeBuilder, zfs::Zfs, Result};

use tokio::process::Command;

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
    pub const DEFAULT_TIMEOUT: u64 = 1;

    const DEFAULT_ZPOOL: &'static str = "dpool";

    pub async fn init() -> Self {
        enum HostType {
            Relay,
            Edge,
        }

        enum Vendor {
            Azure,
            Aws(HostType),
        }

        let zspan = tracing::debug_span!("zpool");
        let _entered = zspan.entered();

        let output = Command::new("zpool")
            .arg("list")
            .output()
            .await
            .expect("failed to get zpool list");

        let out = std::str::from_utf8(output.stdout.as_slice()).expect("failed to get output");
        if out.contains(Self::DEFAULT_ZPOOL) {
            debug!("{} already exists", Self::DEFAULT_ZPOOL);
        } else {
            let output = Command::new("zpool")
                .arg("import")
                .arg(Self::DEFAULT_ZPOOL)
                .output()
                .await
                .expect("failed to exec zpool import command");

            if output.status.success() {
                debug!("{} was imported", Self::DEFAULT_ZPOOL);
            } else {
                debug!("Creating zpool {}", Self::DEFAULT_ZPOOL);
                let vendor = if Command::new("ls")
                    .arg("/replixio/dev/disk/azure")
                    .output()
                    .await
                    .unwrap()
                    .status
                    .success()
                {
                    Vendor::Azure
                } else if Command::new("ls")
                    .arg("/replixio/dev/disk/nvme1n1")
                    .output()
                    .await
                    .unwrap()
                    .status
                    .success()
                {
                    Vendor::Aws(HostType::Relay)
                } else {
                    Vendor::Aws(HostType::Edge)
                };

                let disks = match vendor {
                    Vendor::Azure => &[
                        "/replixio/dev/disk/azure/scsi1/lun2",
                        "/replixio/dev/disk/azure/scsi1/lun3",
                        "/replixio/dev/disk/azure/scsi1/lun4",
                        "/replixio/dev/disk/azure/scsi1/lun5",
                        "/replixio/dev/disk/azure/scsi1/lun6",
                    ],
                    Vendor::Aws(HostType::Relay) => &[
                        "/replix/dev/disk/nvme2n1",
                        "/replix/dev/disk/nvme3n1",
                        "/replix/dev/disk/nvme4n1",
                        "/replix/dev/disk/nvme5n1",
                        "/replix/dev/disk/nvme6n1",
                    ],
                    Vendor::Aws(HostType::Edge) => &[
                        "/replix/dev/disk/nvme1n1",
                        "/replix/dev/disk/nvme2n1",
                        "/replix/dev/disk/nvme3n1",
                        "/replix/dev/disk/nvme4n1",
                        "/replix/dev/disk/nvme5n1",
                    ],
                };

                let output = Command::new("zpool")
                    .arg("create")
                    .arg(Self::DEFAULT_ZPOOL)
                    .args(&["-o", "ashift=12"])
                    .args(&["-O", "mountpoint=none"])
                    .arg("raidz")
                    .args(disks)
                    .output()
                    .await
                    .unwrap_or_else(|_| panic!("zpool create {} failed", Self::DEFAULT_ZPOOL));

                if !output.status.success() {
                    error!("{}", std::str::from_utf8(output.stderr.as_slice()).unwrap());
                    panic!("failed to create zpool {} ", Self::DEFAULT_ZPOOL);
                }

                debug!("zpool {} was created", Self::DEFAULT_ZPOOL);
            }
        }

        Self::default()
    }
}

pub fn list() -> Result<DatasetsProto> {
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

pub trait Dataset {
    type U;
    type I;

    fn create(
        name: String,
        arg: Self::U,
        properties: impl IntoIterator<Item = Self::I>,
    ) -> std::result::Result<(), PropErr> {

    fn get(name: String) -> Result<Self>
    where
        Self: Sized;

    fn destroy(name: String) -> Result<()> {
        Zfs::destroy_dataset(name)?;

        Ok(())
    }
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
    ) -> std::result::Result<VolumeBuilder, PropErr> {
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
            volume_property::Property::BlockSize(property) => vol.blocksize(
                property
                    .check()
                    .map_err(|_| PropErr::InvalidArgument)?
                    .value,
            ),
        };

        Ok(vol)
    }
}

impl Dataset for ProtoVolume {
    type U = u64;
    type I = VolumeProperty;

    fn get(name: String) -> Result<Self> {
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

    fn create(
        name: String,
        capacity: u64,
        properties: impl IntoIterator<Item = VolumeProperty>,
    ) -> std::result::Result<(), PropErr> {
        let builder = Zfs::volume();

        let _volume = properties
            .into_iter()
            .filter_map(|property| property.property)
            .try_fold(builder, Self::add_property)?
            .create(name, capacity)?;

        Ok(())
    }
}

impl ProtoFilesystem {
    pub fn add_property(
        fs: FileSystemBuilder,
        property: filesystem_property::Property,
    ) -> std::result::Result<FileSystemBuilder, PropErr> {
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

impl Dataset for ProtoFilesystem {
    type U = u64;
    type I = FilesystemProperty;

    fn create(
        name: String,
        _unused: u64,
        properties: impl IntoIterator<Item = FilesystemProperty>,
    ) -> Result<()> {
        let builder = Zfs::filesystem();

        let _fs = properties
            .into_iter()
            .filter_map(|property| property.property)
            .try_fold(builder, Self::add_property)?
            .create(name)?;

        Ok(())
    }

    fn get(name: String) -> Result<Self> {
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
