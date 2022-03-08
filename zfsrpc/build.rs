use std::io;

const PROTO_DIR: &str = "../proto";

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed={PROTO_DIR}");

    tonic_build::configure().format(false).compile(
        &["zfsrpc.proto", "zfstracer.proto", "zpool.proto"],
        &[PROTO_DIR],
    )?;
    Ok(())
}
