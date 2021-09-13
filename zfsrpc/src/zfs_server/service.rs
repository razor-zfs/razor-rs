use crate::zfsrpc_proto::tonic_zfsrpc::{filesystem_property, volume_property};
use crate::zfsrpc_proto::tonic_zfsrpc::{
    Filesystem as ProtoFilesystem, FilesystemProperty, Volume as ProtoVolume, VolumeProperty,
};
use paste::paste;
use razor_zfs::{
    error::DatasetError, zfs::FileSystemBuilder, zfs::VolumeBuilder, zfs::Zfs, Result,
};

macro_rules! match_dataset_property {

    ($type:ident, $in:ident, $ds:ident => $($prop:ident),+) => {
        match $in {
           $(
                $type::Property::$prop($in) => {
                    paste!{
                        $ds.[<$prop:lower>]($in.values.ok_or_else(|| DatasetError::InvalidArgument)?)
                    }
                }
           )+
        }
    }
}

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
        vol_name: String,
        capacity: u64,
        properties: impl IntoIterator<Item = VolumeProperty>,
    ) -> Result<()> {
        let builder = Zfs::volume();

        let _volume = properties
            .into_iter()
            .filter_map(|property| property.property)
            .try_fold(builder, Self::add_property)?
            .create(vol_name, capacity)?;

        Ok(())
    }

    pub fn get(vol_name: String) -> Result<Self> {
        let volume = Zfs::get_volume(vol_name.to_string())?;

        let inner = ProtoVolume {
            vol_name,
            available: volume.available(),
            volsize: 1,      // Not implemented yet @razor
            volblocksize: 2, // Not implemented yet @razor
            logicalused: volume.logicalused(),
            checksum: Some(volume.checksum().into()),
            compression: Some(volume.compression().into()),
            guid: volume.guid(),
            creation: volume.creation(),
            createtxg: volume.createtxg(),
            compressratio: volume.compressratio(),
            used: volume.used(),
            referenced: volume.referenced(),
            logicalreferenced: volume.logicalreferenced(),
            objsetid: volume.objsetid(),
        };

        Ok(Self { inner })
    }

    fn add_property(
        vol: VolumeBuilder,
        p: volume_property::Property,
    ) -> razor_zfs::Result<VolumeBuilder> {
        let vol = match_dataset_property!(
            volume_property, p, vol =>
            Checksum,
            Compression
        );

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
        fs_name: String,
        properties: impl IntoIterator<Item = FilesystemProperty>,
    ) -> Result<()> {
        let builder = Zfs::filesystem();

        let _fs = properties
            .into_iter()
            .filter_map(|property| property.property)
            .try_fold(builder, Self::add_property)?
            .create(fs_name)?;

        Ok(())
    }

    pub fn get(fs_name: String) -> Result<Self> {
        let fs = Zfs::get_filesystem(fs_name.to_string())?;

        let inner = ProtoFilesystem {
            fs_name,
            available: fs.available(),
            canmount: Some(fs.canmount().into()),
            atime: Some(fs.atime().into()),
            mounted: Some(fs.mounted().into()),
            checksum: Some(fs.checksum().into()),
            compression: Some(fs.compression().into()),
            guid: fs.guid(),
            creation: fs.creation(),
            createtxg: fs.createtxg(),
            compressratio: fs.compressratio(),
            used: fs.used(),
            logicalused: fs.logicalused(),
            referenced: fs.referenced(),
            logicalreferenced: fs.logicalreferenced(),
            objsetid: fs.objsetid(),
        };

        Ok(Self { inner })
    }

    pub fn add_property(
        fs: FileSystemBuilder,
        p: filesystem_property::Property,
    ) -> razor_zfs::Result<FileSystemBuilder> {
        let fs = match_dataset_property!(
            filesystem_property, p, fs =>
            CanMount,
            Devices,
            ATime,
            Checksum,
            Compression,
            Exec,
            Nbmand,
            Overlay,
            Readonly,
            Relatime,
            Setuid,
            Vscan,
            Zoned
        );

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
