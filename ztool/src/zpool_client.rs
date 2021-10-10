pub(crate) use client::{Method, Property};
use razor_zfsrpc::zpool_client as client;

use std::path::Path;

use walkdir::{DirEntry, WalkDir};

#[allow(unused)]
use tracing::{debug, error, info, trace, warn};

#[derive(Debug)]
enum Vendor {
    Azure,
    Aws,
}

#[derive(Debug)]
pub(crate) struct Client {
    inner: client::Client,
}

impl Client {
    const DEVICES_PATH: &'static str = "/replixio/dev/disk/by-id";
    const EBS_PREFIX: &'static str = "nvme-Amazon_Elastic_Block_Store";
    const ROOT_DEV_PREFIX: &'static str = "nvme0";
    const AZURE_PATH: &'static str = "/replixio/dev/disk/azure";

    fn is_ebs_device(dev: &DirEntry) -> bool {
        dev.file_name()
            .to_str()
            .map_or(false, |s| s.starts_with(Self::EBS_PREFIX))
    }

    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map_or(false, |s| s.starts_with('.'))
    }

    fn is_root_device(dev: &DirEntry) -> bool {
        let f = || {
            dev.path()
                .canonicalize()
                .ok()?
                .as_path()
                .file_name()?
                .to_str()?
                .starts_with(Self::ROOT_DEV_PREFIX)
                .then(|| ())
        };

        f().is_some()
    }

    pub(crate) async fn new(port: String) -> Self {
        let inner = client::Client::new(port).await;
        Self { inner }
    }

    pub(crate) async fn create(
        &mut self,
        name: &str,
        method: Method,
        _disks: Vec<String>,
        properties: Vec<Property>,
    ) -> anyhow::Result<String> {
        let zspan = tracing::debug_span!("ztool-client");
        let _entered = zspan.entered();

        let vendor = if Path::new(Self::AZURE_PATH).exists() {
            Vendor::Azure
        } else {
            Vendor::Aws
        };

        let disks = match vendor {
            Vendor::Azure => {
                vec![
                    "/replixio/dev/disk/azure/scsi1/lun2".into(),
                    "/replixio/dev/disk/azure/scsi1/lun3".into(),
                    "/replixio/dev/disk/azure/scsi1/lun4".into(),
                    "/replixio/dev/disk/azure/scsi1/lun5".into(),
                    "/replixio/dev/disk/azure/scsi1/lun6".into(),
                ]
            }

            Vendor::Aws => {
                for entry in WalkDir::new(Self::DEVICES_PATH) {
                    debug!(?entry);
                }

                WalkDir::new(Self::DEVICES_PATH).contents_first(true)
                    .into_iter()
                    .filter_entry(|entry| {
                        debug!(?entry);
                        if entry.path_is_symlink()
                            && Self::is_ebs_device(entry)
                            && !Self::is_root_device(entry)
                            && !Self::is_hidden(entry)
                        {
                            info!("Pasing entry {:?}", entry);
                            entry.path().to_str().is_some()
                        } else {
                            warn!("Entry {:?} was filtered out!", entry);
                            false
                        }
                    })
                    .inspect(|entry| debug!(?entry))
                    .filter_map(|entry| {
                        if let Ok(entry) = entry {
                            let entry = entry.path().to_str()?.to_string();
                            info!("Entry {:?} will be collected", entry);
                            Some(entry)
                        } else {
                            error!("Not able to stringify entry {:?}", entry);
                            None
                        }
                    })
                    .inspect(|entry| debug!(?entry))
                    .collect()
            }
        };

        debug!(?disks);

        if !disks.is_empty() {
            self.inner.create(name, method, disks, properties).await
        } else {
            Err(anyhow::anyhow!("failed to find disks!"))
        }
    }

    pub(crate) async fn destroy(&mut self, name: &str) -> anyhow::Result<()> {
        self.inner.destroy(name).await?;
        Ok(())
    }
}
