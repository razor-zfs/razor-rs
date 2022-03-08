mod properties;

pub use tonic_zfsrpc::BasicDatasetRequest;
pub use tonic_zfsrpc::CreateFilesystemRequest;
pub use tonic_zfsrpc::CreateSnapshotRequest;
pub use tonic_zfsrpc::CreateVolumeRequest;
pub use tonic_zfsrpc::Dataset;
pub use tonic_zfsrpc::Filesystem;
pub use tonic_zfsrpc::Snapshot;
pub use tonic_zfsrpc::Volume;
pub use tonic_zfsrpc::ZfsType;

pub mod tonic_zfsrpc {
    #![allow(clippy::return_self_not_must_use)]
    #![allow(unreachable_pub, clippy::use_self)]
    tonic::include_proto!("zfsrpc");
}

pub mod tonic_zpoolrpc {
    #![allow(clippy::return_self_not_must_use)]
    #![allow(unreachable_pub, clippy::use_self)]
    tonic::include_proto!("zpoolrpc");
}

pub mod tonic_zfstracer {
    #![allow(clippy::return_self_not_must_use)]
    #![allow(unreachable_pub, clippy::use_self)]
    tonic::include_proto!("zfstracer");
}
