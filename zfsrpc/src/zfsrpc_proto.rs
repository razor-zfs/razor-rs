mod properties;

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
