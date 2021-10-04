use super::zfsrpc_proto::tonic_zfsrpc::{
    zfs_rpc_client::ZfsRpcClient, BasicDatasetRequest, CreateVolumeRequest, Empty, VolumeProperty,
};

#[allow(unused)]
use super::zfsrpc_proto::tonic_zfstracer::{
    trace_level::Level, zfs_tracer_client::ZfsTracerClient, TraceLevel, Variant,
};

use tonic::transport::Channel;

#[allow(unused)]
use tracing::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct Client {
    client: ZfsRpcClient<Channel>,
    tracer_client: ZfsTracerClient<Channel>,
}

impl Client {
    pub async fn new(port: String) -> Self {
        let tracer_client = ZfsTracerClient::connect(format!("http://0.0.0.0:{}", port))
            .await
            .unwrap();
        let client = ZfsRpcClient::connect(format!("http://0.0.0.0:{}", port))
            .await
            .unwrap();
        Self {
            client,
            tracer_client,
        }
    }

    pub async fn list(&mut self) -> anyhow::Result<String> {
        let request = Empty {};
        let request = tonic::Request::new(request);
        let resp = self.client.dataset_list(request).await?;
        let resp = resp.into_inner();

        let resp = format!("{:?}", resp);
        Ok(resp)
    }

    pub async fn get_filesystem(&mut self, name: String) -> anyhow::Result<String> {
        let request = BasicDatasetRequest { name };
        let request = tonic::Request::new(request);

        let fs = self.client.get_filesystem(request).await?;
        let fs = fs.into_inner();

        let resp = format!("{:?}", fs);
        Ok(resp)
    }

    pub async fn destroy_filesystem(&mut self, name: String) -> anyhow::Result<()> {
        let request = BasicDatasetRequest { name };
        let request = tonic::Request::new(request);

        self.client.destroy_filesystem(request).await?;

        Ok(())
    }

    pub async fn create_volume(
        &mut self,
        name: String,
        capacity: u64,
        properties: Vec<VolumeProperty>,
    ) -> anyhow::Result<()> {
        let request = CreateVolumeRequest {
            name,
            capacity,
            properties,
        };
        let request = tonic::Request::new(request);

        self.client.create_volume(request).await?;

        Ok(())
    }

    pub async fn get_volume(&mut self, name: String) -> anyhow::Result<String> {
        let request = BasicDatasetRequest { name };
        let request = tonic::Request::new(request);

        let vol = self.client.get_volume(request).await?;
        let vol = vol.into_inner();

        let resp = format!("{:?}", vol);
        Ok(resp)
    }

    pub async fn destroy_volume(&mut self, name: String) -> anyhow::Result<()> {
        let request = BasicDatasetRequest { name };
        let request = tonic::Request::new(request);

        self.client.destroy_volume(request).await?;

        Ok(())
    }

    pub async fn set_trace_level(&mut self, level: impl ToString) -> anyhow::Result<()> {
        let level = match level.to_string().to_lowercase().as_ref() {
            "trace" => Level::Trace(Variant {}),
            "debug" => Level::Debug(Variant {}),
            "info" => Level::Info(Variant {}),
            "warn" => Level::Warn(Variant {}),
            "error" => Level::Error(Variant {}),
            _ => unreachable!(),
        };
        let request = TraceLevel { level: Some(level) };
        let request = tonic::Request::new(request);

        self.tracer_client.set_tracing_level(request).await?;

        Ok(())
    }
}
