mod properties;

pub mod tonic_zfsrpc {
    tonic::include_proto!("zfsrpc");
}

pub mod tonic_zfstracer {
    tonic::include_proto!("zfstracer");
}
