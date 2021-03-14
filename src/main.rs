#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

use std::io;

mod rest;

pub mod state;
pub mod zfs;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("razor {}", env!("CARGO_PKG_VERSION"));
    rest::serve().await;
    Ok(())
}
