use std::io;

fn main() -> io::Result<()> {
    tonic_build::configure()
        .format(false)
        .compile(&["zfsrpc.proto"], &["../proto"])?;

    Ok(())
}
