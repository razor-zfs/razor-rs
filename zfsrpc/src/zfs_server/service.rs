use crate::zfsrpc_proto::tonic_zfsrpc::{filesystem_property, volume_property};
use crate::zfsrpc_proto::tonic_zfsrpc::{
    Filesystem as ProtoFilesystem, FilesystemProperty, Volume as ProtoVolume, VolumeProperty,
};
use razor_zfs::{
    error::DatasetError, zfs::FileSystemBuilder, zfs::VolumeBuilder, zfs::Zfs, Result,
};

#[allow(unused)]
use tracing::{debug, error, info, trace, warn};

#[derive(Debug, Default)]
pub struct ZfsRpcService {}

impl ZfsRpcService {
    pub const DEFAULT_BLOCKSIZE: u64 = 8192;
    pub const DEFAULT_CAPACITY: u64 = 100 * 1024 * 1024 * 1024;
    pub const DEFAULT_TIMEOUT: u64 = 1;
}

#[derive(Debug)]
pub struct Volume {
    inner: ProtoVolume,
}

impl Volume {
    pub fn create(
        name: String,
        capacity: u64,
        properties: impl IntoIterator<Item = VolumeProperty>,
    ) -> Result<()> {
        let builder = Zfs::volume();

        let _volume = properties
            .into_iter()
            .filter_map(|property| property.property)
            .try_fold(builder, Self::add_property)?
            .create(name, capacity)?;

        Ok(())
    }

    pub fn get(name: String) -> Result<Self> {
        let volume = Zfs::get_volume(&name)?;

        let inner = ProtoVolume {
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

        Ok(Self { inner })
    }

    fn add_property(
        vol: VolumeBuilder,
        property: volume_property::Property,
    ) -> razor_zfs::Result<VolumeBuilder> {
        let vol = match property {
            volume_property::Property::Checksum(property) => {
                vol.checksum(property.value.ok_or(DatasetError::InvalidArgument)?)
            }
            volume_property::Property::Compression(property) => {
                vol.compression(property.value.ok_or(DatasetError::InvalidArgument)?)
            }
            volume_property::Property::VolMode(property) => {
                vol.volmode(property.value.ok_or(DatasetError::InvalidArgument)?)
            }
            volume_property::Property::BlockSize(property) => vol.blocksize(
                property
                    .check()
                    .map_err(|_| DatasetError::InvalidArgument)?
                    .value,
            ),
        };

        Ok(vol)
    }

    pub fn destroy(name: String) -> Result<()> {
        Zfs::destroy_dataset(name)?;

        Ok(())
    }

    pub fn into_inner(self) -> ProtoVolume {
        self.inner
    }
}

impl From<Volume> for ProtoVolume {
    fn from(vol: Volume) -> Self {
        vol.into_inner()
    }
}

#[derive(Debug)]
pub struct Filesystem {
    inner: ProtoFilesystem,
}

impl Filesystem {
    pub fn create(
        name: String,
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

    pub fn get(name: String) -> Result<Self> {
        let fs = Zfs::get_filesystem(&name)?;

        let inner = ProtoFilesystem {
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

        Ok(Self { inner })
    }

    pub fn add_property(
        fs: FileSystemBuilder,
        property: filesystem_property::Property,
    ) -> razor_zfs::Result<FileSystemBuilder> {
        let fs = match property {
            filesystem_property::Property::ATime(atime) => {
                fs.atime(atime.value.ok_or(DatasetError::InvalidArgument)?)
            }
            filesystem_property::Property::CanMount(canmount) => {
                fs.canmount(canmount.value.ok_or(DatasetError::InvalidArgument)?)
            }
            filesystem_property::Property::Checksum(checksum) => {
                fs.checksum(checksum.value.ok_or(DatasetError::InvalidArgument)?)
            }
            filesystem_property::Property::Compression(compression) => {
                fs.compression(compression.value.ok_or(DatasetError::InvalidArgument)?)
            }
            filesystem_property::Property::Devices(devices) => {
                fs.devices(devices.value.ok_or(DatasetError::InvalidArgument)?)
            }
            filesystem_property::Property::Exec(exec) => {
                fs.exec(exec.value.ok_or(DatasetError::InvalidArgument)?)
            }
            filesystem_property::Property::Nbmand(nbmand) => {
                fs.nbmand(nbmand.value.ok_or(DatasetError::InvalidArgument)?)
            }
            filesystem_property::Property::Overlay(overlay) => {
                fs.overlay(overlay.value.ok_or(DatasetError::InvalidArgument)?)
            }
            filesystem_property::Property::Readonly(readonly) => {
                fs.readonly(readonly.value.ok_or(DatasetError::InvalidArgument)?)
            }
            filesystem_property::Property::Relatime(relatime) => {
                fs.relatime(relatime.value.ok_or(DatasetError::InvalidArgument)?)
            }
            filesystem_property::Property::Setuid(setuid) => {
                fs.setuid(setuid.value.ok_or(DatasetError::InvalidArgument)?)
            }
            filesystem_property::Property::Vscan(vscan) => {
                fs.vscan(vscan.value.ok_or(DatasetError::InvalidArgument)?)
            }
            filesystem_property::Property::Zoned(zoned) => {
                fs.zoned(zoned.value.ok_or(DatasetError::InvalidArgument)?)
            }
        };

        Ok(fs)
    }

    pub fn destroy(name: String) -> Result<()> {
        Zfs::destroy_dataset(name)?;

        Ok(())
    }

    pub fn into_inner(self) -> ProtoFilesystem {
        self.inner
    }
}

impl From<Filesystem> for ProtoFilesystem {
    fn from(fs: Filesystem) -> Self {
        fs.into_inner()
    }
}
