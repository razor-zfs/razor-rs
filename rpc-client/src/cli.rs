//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//
#[allow(unused)]
use razor_zfsrpc::{property, zfs_client::Client, PropertyError, VolumeProperty};

use structopt::StructOpt;
#[allow(unused)]
use tracing::{debug, error, info, trace, warn};

const ABOUT: &str = "zfs rpc CLI tool";

#[derive(Debug, StructOpt)]
#[structopt(about = ABOUT)]
pub(crate) struct Cli {
    #[structopt(long, short, about = "Connect to server port", default_value = "50051")]
    port: String,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(
        about = "List of all datasets",
        aliases = &["list"])]
    ZfsList,

    #[structopt(about = "Get filesystem properties", aliases = &["gfs", "get-fs"], display_order(30))]
    GetFilesystem {
        #[structopt(help = "Filesystem name")]
        name: String,
    },

    #[structopt(about = "Destroy filesystem", aliases = &["dfs", "destroy-fs"], display_order(31))]
    DestroyFilesystem {
        #[structopt(help = "Filesystem name")]
        name: String,
    },

    #[structopt(
        about = "Create new volume",
        aliases = &["cv"],
        display_order = 20)]
    CreateVolume {
        #[structopt(help = "Volume name")]
        name: String,
        #[structopt(
            long,
            help = "The volsize can only be set to a multiple of volblocksize"
        )]
        capacity: u64,
        #[structopt(
            long,
            help = "Volume checksum capability",
            possible_values = &[
                "On",
                "Off",
                "Fletcher2",
                "Fletcher4",
                "Sha256",
                "NoParity",
                "Sha512",
                "Skein",
                "Edonr",
            ]
        )]
        checksum: Option<property::CheckSum>,
        #[structopt(long,
            help = "Volume compression capability",
            possible_values = &[
                "On",
                "Off",
                "Lzjb",
                "Gzip",
                "Gzip1",
                "Gzip2",
                "Gzip3",
                "Gzip4",
                "Gzip5",
                "Gzip6",
                "Gzip7",
                "Gzip8",
                "Gzip9",
                "Zle",
                "Lz4",
                "Zstd",
                "ZstdFast",
            ]
        )]
        compression: Option<property::Compression>,
        #[structopt(long,
            help = "Volume volmode",
            possible_values = &[
                "Default",
                "Full",
                "Geom",
                "Dev",
                "None",
                "Unknown", 
            ]
        )]
        volmode: Option<property::VolMode>,
        #[structopt(long,
            help = "Any power of 2 from 512 bytes to 128 Kbytes is valid",
            aliases = &["bs"],
        )]
        blocksize: u32,
    },

    #[structopt(
        about = "Get volume properties", aliases = &["gv", "get-vol"], display_order(21))]
    GetVolume {
        #[structopt(help = "Volume name", long, short)]
        name: String,
    },

    #[structopt(about = "Destroy volume", aliases = &["dv", "destroy-vol"], display_order(22))]
    DestroyVolume {
        #[structopt(help = "Volume name", long, short)]
        name: String,
    },

    #[structopt(about = "Set server trace level", aliases = &["tl", "trace-level"], display_order(90))]
    SetTraceLevel {
        #[structopt(
            long,
            short,
            env = "RUST_LOG",
            help = "Trace level",
            possible_values = &[
                "trace",
                "debug",
                "info",
                "warn",
                "error"
                ]
            )]
        level: Option<String>,
    },
}

impl Cli {
    pub(crate) async fn execute() -> anyhow::Result<()> {
        let this = Self::from_args();
        trace!("{:?}", this);

        let mut client = Client::new(this.port).await;

        let resp: String = match this.command {
            Command::ZfsList => client.list().await?,
            Command::GetVolume { name } => client.get_volume(name).await?,
            Command::GetFilesystem { name } => client.get_filesystem(name).await?,
            Command::CreateVolume {
                name,
                capacity,
                checksum,
                compression,
                volmode,
                blocksize: _blocksize,
            } => {
                let mut properties: Vec<VolumeProperty> = vec![];
                if let Some(checksum) = checksum {
                    properties.push(VolumeProperty::CheckSum(checksum))
                }
                if let Some(compression) = compression {
                    properties.push(VolumeProperty::Compression(compression))
                }

                if let Some(volmode) = volmode {
                    properties.push(VolumeProperty::VolMode(volmode))
                }

                client.create_volume(&name, capacity, vec![]).await?;
                format!("Volume {} created", name)
            }
            Command::DestroyFilesystem { name } => {
                client.destroy_filesystem(&name).await?;
                format!("Filesystem {} destroyed", name)
            }
            Command::DestroyVolume { name } => {
                client.destroy_volume(&name).await?;
                format!("Volume {} destroyed", name)
            }
            Command::SetTraceLevel { level } => {
                if let Some(trace_level) = level {
                    client.set_trace_level(&trace_level).await?;
                    format!("Trace level was set to {}", trace_level)
                } else {
                    String::from("level is missing")
                }
            }
        };

        info!(?resp);

        Ok(())
    }
}
