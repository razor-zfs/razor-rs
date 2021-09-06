fn main() {
    tonic_build::compile_protos("proto/zfsrpc.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    tonic_build::compile_protos("proto/zfstracer.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
