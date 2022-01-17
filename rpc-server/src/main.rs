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

use shadow_rs::shadow;
use tonic::transport::Server;
use tracing::info;

use razor_zfsrpc as zfsrpc;
use zfsrpc::zfs_server::service;
use zfsrpc::zfsrpc_proto::tonic_zfsrpc::zfs_rpc_server::ZfsRpcServer;
use zfsrpc::zfsrpc_proto::tonic_zfstracer::zfs_tracer_server::ZfsTracerServer;
use zfsrpc::zfsrpc_proto::tonic_zpoolrpc::zpool_rpc_server::ZpoolRpcServer;
use zfsrpc::zpool_server;

shadow!(build);

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:50051".parse()?;
    let tracer = razor_tracing::init()?;
    let rpc = service::ZfsRpcService::default();
    let zpool_rpc = zpool_server::ZpoolRpcService::default();

    tracing::info!("Razor Server start version: {}", VERSION);
    log_build_facts();

    Server::builder()
        .add_service(ZfsRpcServer::new(rpc))
        .add_service(ZfsTracerServer::new(tracer))
        .add_service(ZpoolRpcServer::new(zpool_rpc))
        .serve(addr)
        .await?;

    Ok(())
}

fn log_build_facts() {
    info!("debug:{}", shadow_rs::is_debug());
    info!("branch:{}", shadow_rs::branch());
    info!("tag:{}", shadow_rs::tag());
    info!("git_clean:{}", shadow_rs::git_clean());
    info!("git_status_file:{}", shadow_rs::git_status_file());

    info!("{}", build::version());
    info!("{}", build::BRANCH);
    info!("{}", build::SHORT_COMMIT);
    info!("{}", build::COMMIT_HASH);
    info!("{}", build::COMMIT_DATE);
    info!("{}", build::COMMIT_AUTHOR);
    info!("{}", build::COMMIT_EMAIL);

    info!("{}", build::BUILD_OS);
    info!("{}", build::RUST_VERSION);
    info!("{}", build::RUST_CHANNEL);
    info!("{}", build::CARGO_VERSION);
    info!("{}", build::PKG_VERSION);
    // info!("{}", build::CARGO_TREE);

    info!("{}", build::PROJECT_NAME);
    info!("{}", build::BUILD_TIME);
    info!("{}", build::BUILD_RUST_CHANNEL);
    info!("{}", build::GIT_CLEAN);
    info!("{}", build::GIT_STATUS_FILE);
}
