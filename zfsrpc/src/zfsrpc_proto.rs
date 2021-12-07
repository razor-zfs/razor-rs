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
