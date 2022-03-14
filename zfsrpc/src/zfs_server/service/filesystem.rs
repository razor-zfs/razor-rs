use proto::filesystem_property;

use super::*;

impl proto::Filesystem {
    pub(crate) fn add_property(
        fs: zfs::FilesystemBuilder,
        property: filesystem_property::Property,
    ) -> Result<zfs::FilesystemBuilder, zfs::DatasetError> {
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
        properties: impl IntoIterator<Item = proto::FilesystemProperty>,
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

impl From<zfs::Filesystem> for proto::Filesystem {
    fn from(fs: zfs::Filesystem) -> Self {
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
