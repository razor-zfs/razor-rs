#[allow(unused)]
use razor_zfsrpc::{
    property, zfs_client::Client, FilesystemProperty, PropertyError, VolumeProperty,
};

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

    #[structopt(
        about = "Create new filesystem",
        aliases = &["cfs"],
        display_order = 20)]
    CreateFilesystem {
        #[structopt(help = "Filesystem name")]
        name: String,
        #[structopt(
            long,
            help = "Controls whether the access time for files is updated when they are read",
            possible_values = &["on", "off"]
        )]
        atime: Option<property::OnOff>,
        #[structopt(
            long,
            help = "If this property is set to off, the file system cannot be mounted, and is ignored by zfs mount -a",
            possible_values = &["on", "off", "noauto"]
        )]
        canmount: Option<property::OnOffNoAuto>,
        #[structopt(
            long,
            help = "Controls the checksum used to verify data integrity",
            possible_values = &[
                "on",
                "off",
                "fletcher2",
                "fletcher4",
                "sha256",
                "noparity",
                "sha512",
                "skein",
                "edonr",
            ]
        )]
        checksum: Option<property::CheckSum>,
        #[structopt(long,
            help = "Controls the compression algorithm used for this dataset",
            possible_values = &[
                "on",
                "off",
                "lzjb",
                "gzip",
                "gzip1",
                "gzip2",
                "gzip3",
                "gzip4",
                "gzip5",
                "gzip6",
                "gzip7",
                "gzip8",
                "gzip9",
                "zle",
                "lz4",
                "zstd",
                "zstdfast",
            ]
        )]
        compression: Option<property::Compression>,
        #[structopt(
            long,
            help = "Controls whether device nodes can be opened on this file system",
            possible_values = &["on", "off"]
        )]
        devices: Option<property::OnOff>,
        #[structopt(
            long,
            help = "Controls whether processes can be executed from within this file system",
            possible_values = &["on", "off"]
        )]
        exec: Option<property::OnOff>,
        #[structopt(
            long,
            help = "Controls whether the file system should be mounted with nbmand (Non Blocking mandatory locks)",
            possible_values = &["on", "off"]
        )]
        nbmand: Option<property::OnOff>,
        #[structopt(
            long,
            help = "Allow mounting on a busy directory or a directory which already contains files or directories",
            possible_values = &["on", "off"]
        )]
        overlay: Option<property::OnOff>,
        #[structopt(
            long,
            help = "Controls whether this dataset can be modified",
            possible_values = &["on", "off"]
        )]
        readonly: Option<property::OnOff>,
        #[structopt(
            long,
            help = "Controls the manner in which the access time is updated when atime=on is set",
            possible_values = &["on", "off"]
        )]
        relatime: Option<property::OnOff>,
        #[structopt(
            long,
            help = "Controls whether the setuid bit is respected for the file system",
            possible_values = &["on", "off"]
        )]
        setuid: Option<property::OnOff>,
        #[structopt(
            long,
            help = "Controls whether regular files should be scanned for viruses when a file is opened and closed",
            possible_values = &["on", "off"]
        )]
        vscan: Option<property::OnOff>,
        #[structopt(
            long,
            help = "Controls whether the dataset is managed from a non-global zone",
            possible_values = &["on", "off"]
        )]
        zoned: Option<property::OnOff>,
    },

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
            help = "The capacity can only be set to a multiple of volblocksize"
        )]
        capacity: u64,
        #[structopt(
            long,
            help = "Controls the checksum used to verify data integrity",
            possible_values = &[
                "on",
                "off",
                "fletcher2",
                "fletcher4",
                "sha256",
                "noparity",
                "sha512",
                "skein",
                "edonr",
            ],
        )]
        checksum: Option<property::CheckSum>,
        #[structopt(long,
            help = "Controls the compression algorithm used for this dataset",
            possible_values = &[
                "on",
                "off",
                "lzjb",
                "gzip",
                "gzip1",
                "gzip2",
                "gzip3",
                "gzip4",
                "gzip5",
                "gzip6",
                "gzip7",
                "gzip8",
                "gzip9",
                "zle",
                "lz4",
                "zstd",
                "zstdfast",
            ]
        )]
        compression: Option<property::Compression>,
        #[structopt(long,
            help = "This property specifies how volumes should be exposed to the OS",
            possible_values = &[
                "default",
                "full",
                "geom",
                "dev",
                "none",
                "unknown",
            ]
        )]
        volmode: Option<property::VolMode>,
        #[structopt(long,
            help = "Any power of 2 from 512 bytes to 128 Kbytes is valid",
            default_value = "8192",
            aliases = &["bs"],
        )]
        blocksize: u64,
    },

    #[structopt(
        about = "Get volume properties", aliases = &["gv", "get-vol"], display_order(21))]
    GetVolume {
        #[structopt(help = "Volume name", long, short)]
        name: String,
    },

    #[structopt(about = "Destroy volume", aliases = &["dv", "destroy-vol"], display_order(22))]
    DestroyVolume {
        #[structopt(help = "Volume name")]
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
                blocksize,
            } => {
                let properties = vec![
                    checksum.map(VolumeProperty::CheckSum),
                    compression.map(VolumeProperty::Compression),
                    volmode.map(VolumeProperty::VolMode),
                ];

                client
                    .create_volume(&name, capacity, blocksize, properties)
                    .await?;
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
            Command::CreateFilesystem {
                name,
                atime,
                canmount,
                checksum,
                compression,
                devices,
                exec,
                nbmand,
                overlay,
                readonly,
                relatime,
                setuid,
                vscan,
                zoned,
            } => {
                let properties = vec![
                    atime.map(FilesystemProperty::OnOff),
                    canmount.map(FilesystemProperty::OnOffNoAuto),
                    checksum.map(FilesystemProperty::CheckSum),
                    compression.map(FilesystemProperty::Compression),
                    devices.map(FilesystemProperty::OnOff),
                    exec.map(FilesystemProperty::OnOff),
                    nbmand.map(FilesystemProperty::OnOff),
                    overlay.map(FilesystemProperty::OnOff),
                    readonly.map(FilesystemProperty::OnOff),
                    readonly.map(FilesystemProperty::OnOff),
                    relatime.map(FilesystemProperty::OnOff),
                    setuid.map(FilesystemProperty::OnOff),
                    vscan.map(FilesystemProperty::OnOff),
                    zoned.map(FilesystemProperty::OnOff),
                ];

                client.create_filesystem(&name, properties).await?;
                format!("Filesystem {} created", name)
            }
        };

        info!(?resp);

        Ok(())
    }
}
