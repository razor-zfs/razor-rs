use razor_zfs::error::DatasetError;
use thiserror::Error;

mod properties;

pub mod tonic_zfsrpc {
    #![allow(unreachable_pub, clippy::use_self)]
    tonic::include_proto!("zfsrpc");
}

pub mod tonic_zpoolrpc {
    #![allow(unreachable_pub, clippy::use_self)]
    tonic::include_proto!("zpoolrpc");
}

pub mod tonic_zfstracer {
    #![allow(unreachable_pub, clippy::use_self)]
    tonic::include_proto!("zfstracer");
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum PropErr {
    #[error("invalid argument")]
    InvalidArgument,
    #[error(transparent)]
    ZfsError(#[from] DatasetError),
}
