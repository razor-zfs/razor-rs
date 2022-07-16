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

mod summary;

#[derive(Debug, Parser)]
enum Command {
    /// Version of various ZFS components
    Version,
    /// Create new ZFS dataset
    Summary {
        /// Show summary of this pool
        pool: String,
    },
}

impl Command {
    fn exec(self) -> anyhow::Result<String> {
        let text = match self {
            Self::Version => format!("CLI: {}", clap::crate_version!()),
            Self::Summary { pool } => summary::summary(pool)?,
        };

        Ok(text)
    }
}

fn main() -> anyhow::Result<()> {
    Command::parse().exec().map(|text| println!("{text}"))
}
