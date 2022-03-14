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

use pb::zfs_tracer_client::ZfsTracerClient;
use pb::Level;
use pb::TraceLevel;

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

    pub async fn set_trace_level(&mut self, level: Level) -> Result<(), tonic::Status> {
        let level = level as i32;
        let request = TraceLevel { level };
        let _empty = self.client.set_tracing_level(request).await?.into_inner();

        Ok(())
    }
}
