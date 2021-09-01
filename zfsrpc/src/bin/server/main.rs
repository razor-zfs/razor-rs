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
use zfsrpc::server::service;
use zfsrpc::zfsrpc_proto::zfs_rpc_server::ZfsRpcServer;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let rpc = service::ZfsRpcService::default();
    Server::builder()
        .timeout(time::Duration::from_secs(
            service::ZfsRpcService::DEFAULT_TIMEOUT,
        ))
        .add_service(ZfsRpcServer::new(rpc))
        .serve(addr)
        .await?;

    Ok(())
}
