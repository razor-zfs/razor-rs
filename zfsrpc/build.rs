use std::io;

fn main() -> io::Result<()> {
    tonic_build::compile_protos("../proto/zfsrpc.proto")?;
    tonic_build::compile_protos("../proto/zfstracer.proto")?;
    tonic_build::compile_protos("../proto/zpool.proto")?;

    Ok(())
}
