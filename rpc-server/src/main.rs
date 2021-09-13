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

use std::time;

use tonic::transport::Server;

use razor_zfsrpc as zfsrpc;
use zfsrpc::zfs_server::service;
use zfsrpc::zfsrpc_proto::tonic_zfsrpc::zfs_rpc_server::ZfsRpcServer;
use zfsrpc::zfsrpc_proto::tonic_zfstracer::zfs_tracer_server::ZfsTracerServer;

use razor_tracing as tracing;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let rpc = service::ZfsRpcService::default();
    let tracer = tracing::init()?;
    Server::builder()
        .timeout(time::Duration::from_secs(
            service::ZfsRpcService::DEFAULT_TIMEOUT,
        ))
        .add_service(ZfsRpcServer::new(rpc))
        .add_service(ZfsTracerServer::new(tracer))
        .serve(addr)
        .await?;

    Ok(())
}
