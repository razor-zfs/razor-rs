pub(crate) use client::{Method, Property};
use razor_zfsrpc::zpool_client as client;

use tokio::process::Command;
use walkdir::WalkDir;

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
    const EBS: &'static str = "Amazon_Elastic_Block_Store";
    const ROOT_DEV: &'static str = "nvme0n1";

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

        let mut disks = vec![];
        let disks = match vendor {
            Vendor::Azure => {
                disks.push("/replixio/dev/disk/azure/scsi1/lun2".into());
                disks.push("/replixio/dev/disk/azure/scsi1/lun3".into());
                disks.push("/replixio/dev/disk/azure/scsi1/lun4".into());
                disks.push("/replixio/dev/disk/azure/scsi1/lun5".into());
                disks.push("/replixio/dev/disk/azure/scsi1/lun6".into());
                disks
            }

            Vendor::Aws => {
                // disks.push("/dev/nvme1n1".into());
                // disks
                WalkDir::new(Self::DEVICES_PATH)
                    .into_iter()
                    .fold(disks, |mut disks, entry| {
                        let entry = entry.unwrap();
                        if entry.path().to_str().unwrap().contains(Self::EBS)
                            && !entry
                                .path()
                                .canonicalize()
                                .unwrap()
                                .ends_with(Self::ROOT_DEV)
                        {
                            let disk = entry.path().to_str().unwrap().to_string();
                            disks.push(disk);
                        }
                        disks
                    })
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
