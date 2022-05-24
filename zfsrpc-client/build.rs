use std::io;

const PROTO_DIR: &str = "../proto";

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed={PROTO_DIR}");

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&["zfsrpc.proto"], &[PROTO_DIR])
}
