use tonic::transport::Channel;

use super::zfsrpc_proto::tonic_zfstracer::{
    trace_level::Level, zfs_tracer_client::ZfsTracerClient, TraceLevel, Variant,
};

#[derive(Debug)]
pub struct Client {
    tracer_client: ZfsTracerClient<Channel>,
}

impl Client {
    pub async fn new(port: String) -> Self {
        let tracer_client = ZfsTracerClient::connect(format!("http://0.0.0.0:{}", port))
            .await
            .expect("Failed to connect to tracer server");
        Self { tracer_client }
    }

    pub async fn set_trace_level(&mut self, level: impl ToString) -> anyhow::Result<()> {
        let level = match level.to_string().to_lowercase().as_ref() {
            "trace" => Level::Trace(Variant {}),
            "debug" => Level::Debug(Variant {}),
            "info" => Level::Info(Variant {}),
            "warn" => Level::Warn(Variant {}),
            "error" => Level::Error(Variant {}),
            level => anyhow::bail!("unknown-tracing level {}", level),
        };
        let request = TraceLevel { level: Some(level) };
        let request = tonic::Request::new(request);

        self.tracer_client.set_tracing_level(request).await?;

        Ok(())
    }
}
