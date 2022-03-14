use std::net::IpAddr;

use anyhow::Context;
use tokio_stream::StreamExt;

use crate::error::{Fixme, ZfsError};

use super::proto::{
    self, zfs_rpc_client::ZfsRpcClient, BasicDatasetRequest, CreateFilesystemRequest,
    CreateVolumeRequest, Empty, Filesystem, MountFilesystemRequest, Volume,
};
use super::{FilesystemProperty, VolumeProperty};
use tonic::transport::Channel;

impl From<VolumeProperty> for proto::VolumeProperty {
    fn from(p: VolumeProperty) -> Self {
        match p {
            VolumeProperty::CheckSum(val) => val.into(),
            VolumeProperty::Compression(val) => val.into(),
            VolumeProperty::VolMode(val) => val.into(),
        }
    }
}
impl From<FilesystemProperty> for proto::FilesystemProperty {
    fn from(p: FilesystemProperty) -> Self {
        match p {
            FilesystemProperty::CheckSum(val) => val.into(),
            FilesystemProperty::Compression(val) => val.into(),
            FilesystemProperty::OnOff(val) => val.into(),
            FilesystemProperty::OnOffNoAuto(val) => val.into(),
        }
    }
}

#[derive(Debug)]
pub struct Client {
    client: ZfsRpcClient<Channel>,
}

impl Client {
    #[deprecated(note = "panics if failed to connected to 0.0.0.0:{ip}")]
    pub async fn new(port: String) -> Self {
        let client = ZfsRpcClient::connect(format!("http://0.0.0.0:{}", port))
            .await
            .expect("Failed to connect to zfs server");
        Self { client }
    }

    #[deprecated(note = "panics if failed to connected to {host}:{ip}")]
    pub async fn with_ip(host: IpAddr, port: String) -> Self {
        let client = ZfsRpcClient::connect(format!("http://{}:{}", host, port))
            .await
            .expect("Failed to connect to zfs server");
        Self { client }
    }

    pub async fn try_with_ip(host: IpAddr, port: String) -> anyhow::Result<Self> {
        let client = ZfsRpcClient::connect(format!("http://{}:{}", host, port))
            .await
            .context("Failed to connect to zfs server")?;
        Ok(Self { client })
    }

    pub async fn list(&mut self) -> anyhow::Result<String> {
        let request = Empty {};

        let datasets = self.client.dataset_list(request).await?.into_inner();

        let resp = format!("{datasets:?}");
        Ok(resp)
    }

    pub async fn create_filesystem(
        &mut self,
        path: impl ToString,
        properties: Vec<Option<FilesystemProperty>>,
    ) -> anyhow::Result<Filesystem, ZfsError> {
        let path = path.to_string();
        let properties: Vec<_> = properties
            .into_iter()
            .filter_map(|item| item.map(From::from))
            .collect();

        if !properties.is_empty() {
            return Err(ZfsError::NotImplemented(Fixme::Mango2456));
        }
        let request = CreateFilesystemRequest {
            name: path,
            properties,
        };

        let fs = self.client.create_filesystem(request).await?.into_inner();

        Ok(fs)
    }

    pub async fn get_filesystem(&mut self, name: impl ToString) -> anyhow::Result<String> {
        let name = name.to_string();
        let request = BasicDatasetRequest { name };

        let fs = self.client.get_filesystem(request).await?.into_inner();

        let resp = format!("{fs:?}");
        Ok(resp)
    }

    pub async fn destroy_filesystem(&mut self, name: impl ToString) -> anyhow::Result<()> {
        let name = name.to_string();
        let request = BasicDatasetRequest { name };

        self.client.destroy_filesystem(request).await?;

        Ok(())
    }

    pub async fn mount_filesystem(
        &mut self,
        name: impl ToString,
        mountpoint: impl ToString,
    ) -> anyhow::Result<()> {
        let name = name.to_string();
        let mountpoint = mountpoint.to_string();
        let request = MountFilesystemRequest { name, mountpoint };

        self.client.mount_filesystem(request).await?;

        Ok(())
    }

    pub async fn unmount_filesystem(&mut self, name: impl ToString) -> anyhow::Result<()> {
        let name = name.to_string();
        let request = BasicDatasetRequest { name };

        self.client.unmount_filesystem(request).await?;

        Ok(())
    }

    pub async fn create_volume(
        &mut self,
        name: impl ToString,
        capacity: u64,
        blocksize: u64,
        properties: Vec<Option<VolumeProperty>>,
    ) -> anyhow::Result<Volume, ZfsError> {
        let name = name.to_string();
        let properties: Vec<_> = properties
            .into_iter()
            .filter_map(|item| item.map(From::from))
            .collect();

        if !properties.is_empty() {
            return Err(ZfsError::NotImplemented(Fixme::Mango2456));
        }

        let request = CreateVolumeRequest {
            name,
            capacity,
            blocksize,
            properties,
        };

        let volume = self.client.create_volume(request).await?.into_inner();

        Ok(volume)
    }

    pub async fn get_volume(&mut self, name: impl ToString) -> anyhow::Result<String> {
        let name = name.to_string();
        let request = BasicDatasetRequest { name };

        let volume = self.client.get_volume(request).await?.into_inner();

        let resp = format!("{volume:?}");
        Ok(resp)
    }

    pub async fn destroy_volume(&mut self, name: impl ToString) -> anyhow::Result<()> {
        let name = name.to_string();
        let request = BasicDatasetRequest { name };

        self.client.destroy_volume(request).await?;

        Ok(())
    }

    pub async fn create_snapshot(
        &mut self,
        name: String,
        recursive: bool,
    ) -> anyhow::Result<proto::Snapshot> {
        let request = proto::CreateSnapshotRequest { name, recursive };

        self.client
            .create_snapshot(request)
            .await
            .map(|response| response.into_inner())
            .context("Create snapshot")
    }

    pub async fn list_snapshots(
        &mut self,
        _name: Option<String>,
    ) -> anyhow::Result<proto::Snapshot> {
        anyhow::bail!("Not implemented yet")
    }

    pub async fn show_snapshot(&mut self, name: String) -> anyhow::Result<proto::Snapshot> {
        let request = proto::BasicDatasetRequest { name };
        self.client
            .get_snapshot(request)
            .await
            .map(|response| response.into_inner())
            .context("Show Snapshot failed")
    }

    pub async fn destroy_snapshot(&mut self, name: String) -> anyhow::Result<()> {
        let request = proto::BasicDatasetRequest { name };
        self.client
            .destroy_snapshot(request)
            .await
            .map(|_| ())
            .context("Destroy snapshot failed")
    }

    pub async fn create_bookmark(
        &mut self,
        snapshot: impl ToString,
        bookmark: impl ToString,
    ) -> anyhow::Result<proto::Bookmark> {
        let snapshot = snapshot.to_string();
        let bookmark = bookmark.to_string();
        let request = proto::CreateBookmarkRequest { snapshot, bookmark };
        self.client
            .create_bookmark(request)
            .await
            .map(|response| response.into_inner())
            .context("Create bookmark failed")
    }

    pub async fn send_snapshot(
        &mut self,
        source: String,
        from: Option<String>,
    ) -> anyhow::Result<tonic::Streaming<proto::SendSegment>> {
        let from = from.unwrap_or_default();
        let request = proto::SendRequest { from, source };

        self.client
            .send(request)
            .await
            .map(|response| response.into_inner())
            .context("Send Snapshot")
    }

    pub async fn recv_snapshot(
        &mut self,
        snapshot: String,
        input: impl tokio_stream::Stream<Item = Vec<u8>> + std::marker::Send + 'static,
        // input: impl tokio::io::AsyncReadExt,
    ) -> anyhow::Result<()> {
        let mut sequence = 0;
        let request = input.map(move |buffer| {
            let segment = proto::SendSegment {
                name: snapshot.clone(),
                sequence,
                buffer,
            };
            sequence += 1;
            segment
        });

        self.client
            .recv(request)
            .await
            .map(|_response| ())
            .context("Receive Snapshot")
    }
}
