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

pub mod client;
mod traits;
mod zfsrpc_proto;

pub use razor_property as property;

pub(crate) use zfsrpc_proto::tonic_zfsrpc as proto;

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
