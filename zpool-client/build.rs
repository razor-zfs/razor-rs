use std::io;

fn main() -> io::Result<()> {
    tonic_build::configure()
        .compile(&["zpool.proto"], &["../proto"])?;

    Ok(())
}
