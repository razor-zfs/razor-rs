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

pub use razor_property as property;
pub use razor_property::InvalidProperty as PropertyError;
pub use zfs_server::service::ZfsRpcService;

pub mod tracing_server;
pub mod zfs_client;
pub mod zfs_server;
pub mod zfsrpc_proto;
pub mod zpool_server;

#[derive(Debug)]
pub enum VolumeProperty {
    CheckSum(property::CheckSum),
    Compression(property::Compression),
    VolMode(property::VolMode),
}

#[derive(Debug)]
pub enum FilesystemProperty {
    OnOff(property::OnOff),
    OnOffNoAuto(property::OnOffNoAuto),
    CheckSum(property::CheckSum),
    Compression(property::Compression),
}
