#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

pub mod pb {
    #![allow(clippy::return_self_not_must_use)]
    #![allow(unreachable_pub, clippy::use_self)]
    tonic::include_proto!("zfstracer");
}

use tonic::transport::{Channel, Error};

use pb::trace_level::Level;
use pb::zfs_tracer_client::ZfsTracerClient;
use pb::TraceLevel;
use pb::Variant;

#[derive(Debug)]
pub struct Client {
    client: ZfsTracerClient<Channel>,
}

impl Client {
    pub async fn new(port: String) -> Result<Self, Error> {
        ZfsTracerClient::connect(format!("http://0.0.0.0:{}", port))
            .await
            .map(|client| Self { client })
    }

    pub async fn set_trace_level(&mut self, level: impl ToString) -> Result<(), tonic::Status> {
        let level = match level.to_string().to_lowercase().as_ref() {
            "trace" => Level::Trace(Variant {}),
            "debug" => Level::Debug(Variant {}),
            "info" => Level::Info(Variant {}),
            "warn" => Level::Warn(Variant {}),
            "error" => Level::Error(Variant {}),
            level => {
                return Err(tonic::Status::invalid_argument(format!(
                    "unknown tracing level {level}"
                )))
            }
        };
        let request = TraceLevel { level: Some(level) };
        let _empty = self.client.set_tracing_level(request).await?.into_inner();

        Ok(())
    }
}
