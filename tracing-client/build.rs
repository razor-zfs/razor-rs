use std::io;

const PROTO_DIR: &str = "../proto";

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed={PROTO_DIR}");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .format(true)
        .compile(&["zfstracer.proto"], &[PROTO_DIR])
}
