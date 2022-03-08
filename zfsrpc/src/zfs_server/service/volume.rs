use super::*;

impl Volume {
    fn add_property(
        vol: VolumeBuilder,
        property: volume_property::Property,
    ) -> Result<VolumeBuilder, DatasetError> {
        let vol = match property {
            volume_property::Property::Checksum(property) => {
                vol.checksum(property.value.ok_or_else(DatasetError::missing_value)?)
            }
            volume_property::Property::Compression(property) => {
                vol.compression(property.value.ok_or_else(DatasetError::missing_value)?)
            }
            volume_property::Property::VolMode(property) => {
                vol.volmode(property.value.ok_or_else(DatasetError::missing_value)?)
            }
        };

        Ok(vol)
    }

    pub(crate) fn get(name: String) -> Result<Self, ZfsError> {
        let volume = Zfs::get_volume(&name)?;

        Ok(volume.into())
    }

    pub(crate) fn create(
        name: String,
        capacity: u64,
        blocksize: u64,
        properties: impl IntoIterator<Item = VolumeProperty>,
    ) -> Result<Self, ZfsError> {
        let builder = Zfs::volume();

        let volume = properties
            .into_iter()
            .filter_map(|property| property.property)
            .try_fold(builder, Self::add_property)?
            .blocksize(blocksize)
            .create(name, capacity)?;

        Ok(volume.into())
    }
}

impl From<zfs::Volume> for Volume {
    fn from(volume: zfs::Volume) -> Self {
        Self {
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
        }
    }
}
