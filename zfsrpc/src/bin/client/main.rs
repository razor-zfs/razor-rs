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

use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

use zfsrpc::zfsrpc_proto::zfs_rpc_client::ZfsRpcClient;
use zfsrpc::zfsrpc_proto::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(fmt::time::ChronoUtc::default())
        .init();

    let mut client = ZfsRpcClient::connect("http://0.0.0.0:50051").await?;

    // let capacity = Some(10 * 1024 * 1024 * 1024);
    let pool: String = "dpool".to_string();
    let name: String = "Vol2".to_string();
    //let canmount = dataset_properties::CanMount::off().into();
    //let atime = dataset_properties::ATime::off().into();

    //let properties = vec![canmount, atime];
    // let properties = Vec::new();

    //let request = CreateFilesystemRequest {
    // let request = CreateVolumeRequest {
    //     capacity,
    //      pool: pool.clone(),
    //      name: name.clone(),
    //      properties,
    //  };

    //let request = tonic::Request::new(request);
    // client.create_volume(request).await?;

    let request = BasicDatasetRequest { pool, name };

    let request = tonic::Request::new(request);

    //client.destroy_dataset(request).await?;
    let fs = client.get_filesystem(request).await?;

    info!(?fs);

    Ok(())
}