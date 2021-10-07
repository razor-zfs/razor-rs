pub(crate) use client::{Method, Property};
use razor_zfsrpc::zpool_client as client;

use tokio::process::Command;
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

    fn is_ebs_device(dev: &DirEntry) -> bool {
        dev.file_name()
            .to_str()
            .map_or(false, |s| s.starts_with(Self::EBS_PREFIX))
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
    ) -> anyhow::Result<()> {
        let zspan = tracing::debug_span!("ztool-client");
        let _entered = zspan.entered();

        let vendor = if Command::new("ls")
            .arg("/replixio/dev/disk/azure")
            .output()
            .await
            .unwrap()
            .status
            .success()
        {
            Vendor::Azure
        } else {
            Vendor::Aws
        };

        debug!(?vendor);

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
                // disks.push("/dev/nvme1n1".into());
                // disks
                WalkDir::new(Self::DEVICES_PATH)
                    .into_iter()
                    .filter_entry(|entry| {
                        Self::is_ebs_device(entry) && !Self::is_root_device(entry)
                    })
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.path().to_str().map(|entry| entry.to_string()))
                    })
                    .collect()
            }
        };

        debug!(?disks);

        self.inner.create(name, method, disks, properties).await?;

        Ok(())
    }

    pub(crate) async fn destroy(&mut self, name: &str) -> anyhow::Result<()> {
        self.inner.destroy(name).await?;
        Ok(())
    }
}
