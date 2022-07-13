#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

use clap::Parser;
use razor_cli as cli;

use cli::zfs::Create;

#[derive(Debug, Parser)]
enum Command {
    /// Version of various ZFS components
    Version,
    /// Create new ZFS dataset
    Create {
        #[clap(flatten)]
        create: Create,
    },
}

impl Command {
    fn exec(self) -> anyhow::Result<String> {
        let text = match self {
            Self::Version => format!("CLI: {}", clap::crate_version!()),
            Self::Create { create } => create.exec()?,
        };

        Ok(text)
    }
}

fn main() -> anyhow::Result<()> {
    Command::parse().exec().map(|text| println!("{text}"))
}
