use std::io::Cursor;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use razor_zfsrpc_client::{
    client::Client as ZfsClient, property, FilesystemProperty, VolumeProperty,
};
use tokio::fs;
use tokio::io::{self, AsyncReadExt};

#[allow(unused)]
use tracing::{debug, error, info, trace, warn};

const ABOUT: &str = "zfs rpc CLI tool";

#[derive(Debug, Parser)]
#[clap(about = ABOUT)]
pub(crate) struct Cli {
    #[clap(long, short, help = "Connect to server port", default_value = "50051")]
    port: String,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(
        about = "List of all datasets",
        aliases = &["list"])]
    ZfsList,

    #[clap(
        about = "Create new filesystem",
        aliases = &["cfs"],
        display_order = 20)]
    CreateFilesystem {
        #[clap(help = "Filesystem name")]
        name: String,
        #[clap(
            long,
            help = "Controls whether the access time for files is updated when they are read",
            possible_values = &["on", "off"]
        )]
        atime: Option<property::OnOff>,
        #[clap(
            long,
            help = "If this property is set to off, the file system cannot be mounted, and is ignored by zfs mount -a",
            possible_values = &["on", "off", "noauto"]
        )]
        canmount: Option<property::OnOffNoAuto>,
        #[clap(
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
        #[clap(long,
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
        #[clap(
            long,
            help = "Controls whether device nodes can be opened on this file system",
            possible_values = &["on", "off"]
        )]
        devices: Option<property::OnOff>,
        #[clap(
            long,
            help = "Controls whether processes can be executed from within this file system",
            possible_values = &["on", "off"]
        )]
        exec: Option<property::OnOff>,
        #[clap(
            long,
            help = "Controls whether the file system should be mounted with nbmand (Non Blocking mandatory locks)",
            possible_values = &["on", "off"]
        )]
        nbmand: Option<property::OnOff>,
        #[clap(
            long,
            help = "Allow mounting on a busy directory or a directory which already contains files or directories",
            possible_values = &["on", "off"]
        )]
        overlay: Option<property::OnOff>,
        #[clap(
            long,
            help = "Controls whether this dataset can be modified",
            possible_values = &["on", "off"]
        )]
        readonly: Option<property::OnOff>,
        #[clap(
            long,
            help = "Controls the manner in which the access time is updated when atime=on is set",
            possible_values = &["on", "off"]
        )]
        relatime: Option<property::OnOff>,
        #[clap(
            long,
            help = "Controls whether the setuid bit is respected for the file system",
            possible_values = &["on", "off"]
        )]
        setuid: Option<property::OnOff>,
        #[clap(
            long,
            help = "Controls whether regular files should be scanned for viruses when a file is opened and closed",
            possible_values = &["on", "off"]
        )]
        vscan: Option<property::OnOff>,
        #[clap(
            long,
            help = "Controls whether the dataset is managed from a non-global zone",
            possible_values = &["on", "off"]
        )]
        zoned: Option<property::OnOff>,
    },

    #[clap(about = "Get filesystem properties", aliases = &["gfs", "get-fs"], display_order(30))]
    GetFilesystem {
        #[clap(help = "Filesystem name")]
        name: String,
    },

    #[clap(about = "Destroy filesystem", aliases = &["dfs", "destroy-fs"], display_order(31))]
    DestroyFilesystem {
        #[clap(help = "Filesystem name")]
        name: String,
    },
    #[clap(about = "Mounting filesystem", aliases = &["mfs", "mount-fs"], display_order(32))]
    MountFilesystem {
        #[clap(help = "Filesystem name")]
        name: String,
        #[clap(help = "filesystem mountpoint")]
        mountpoint: String,
    },
    #[clap(about = "Unmounting filesystem", aliases = &["umfs", "unmount-fs"], display_order(32))]
    UnmountFilesystem {
        #[clap(help = "Filesystem name")]
        name: String,
    },
    #[clap(
        about = "Create new volume",
        aliases = &["cv"],
        display_order = 20)]
    CreateVolume {
        #[clap(help = "Volume name")]
        name: String,
        #[clap(
            long,
            help = "The capacity can only be set to a multiple of volblocksize"
        )]
        capacity: u64,
        #[clap(
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
        #[clap(long,
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
        #[clap(long,
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
        #[clap(long,
            help = "Any power of 2 from 512 bytes to 128 Kbytes is valid",
            default_value = "8192",
            aliases = &["bs"],
        )]
        blocksize: u64,
    },

    #[clap(
        about = "Get volume properties", aliases = &["gv", "get-vol"], display_order(21))]
    GetVolume {
        #[clap(help = "Volume name")]
        name: String,
    },

    #[clap(about = "Destroy volume", aliases = &["dv", "destroy-vol"], display_order(22))]
    DestroyVolume {
        #[clap(help = "Volume name")]
        name: String,
    },

    #[clap(about = "Create snapshot", visible_aliases = &["cs", "create-snap"])]
    CreateSnapshot {
        #[clap(help = "Snapshot name")]
        name: String,
        #[clap(help = "Recursive snapshot", long, short)]
        recursive: bool,
    },

    #[clap(about = "Destroy snapshot", visible_aliases = &["ds", "destroy-snap"])]
    DestroySnapshot {
        #[clap(help = "Snapshot name")]
        name: String,
    },

    #[clap(about = "List snapshots", visible_aliases = &["ls", "list-snap"])]
    ListSnapshots {
        #[clap(help = "Dataset name")]
        name: Option<String>,
    },
    #[clap(about = "Show snapshot", visible_aliases = &["ss", "show-snap"])]
    ShowSnapshot {
        #[clap(help = "Snapshot name")]
        name: String,
    },

    #[clap(about = "Create bookmark", visible_aliases = &["cb", "bookmark"])]
    CreateBookmark {
        #[clap(help = "Source (snapshot or bookmark) name")]
        snapshot: String,
        #[clap(help = "New bookmark name")]
        bookmark: String,
    },

    #[clap(about = "Show bookmark", visible_aliases = &["sb"])]
    ShowBookmark {
        #[clap(help = "Bookmark name")]
        bookmark: String,
    },

    #[clap(about = "Destroy bookmark", visible_aliases = &["db"])]
    DestroyBookmark {
        #[clap(help = "Bookmark name")]
        bookmark: String,
    },

    #[clap(about = "Send snapshot")]
    Send {
        #[clap(help = "Source snapshot name")]
        source: String,
        #[clap(help = "Output data file")]
        output: PathBuf,
        #[clap(
            help = "Incremental send starting point - can be snapshot or bookmark",
            long,
            short
        )]
        incremental: Option<String>,
    },

    #[clap(about = "Receive snapshot", visible_alias = "recv")]
    Receive {
        #[clap(help = "Target snapshot name")]
        snapshot: String,
        #[clap(help = "Output data file")]
        input: PathBuf,
    },
}

impl Cli {
    pub(crate) async fn execute() -> anyhow::Result<()> {
        let this = Self::parse();
        trace!("{:?}", this);

        let mut client = ZfsClient::try_with_ip([0; 4].into(), this.port).await?;

        let text = match this.command {
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

                let vol = client
                    .create_volume(&name, capacity, blocksize, properties)
                    .await?;
                format!("Volume {} created: \n{:?}", name, vol)
            }
            Command::DestroyFilesystem { name } => {
                client.destroy_filesystem(&name).await?;
                format!("Filesystem {} destroyed", name)
            }
            Command::DestroyVolume { name } => {
                client.destroy_volume(&name).await?;
                format!("Volume {} destroyed", name)
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

                let fs = client.create_filesystem(&name, properties).await?;
                format!("Filesystem {} created\n{:?}", name, fs)
            }
            Command::MountFilesystem { name, mountpoint } => {
                client.mount_filesystem(&name, &mountpoint).await?;
                format!("Filesystem {} is mounted", name)
            }
            Command::UnmountFilesystem { name } => {
                client.unmount_filesystem(&name).await?;
                format!("Filesystem {} is unmounted", name)
            }
            Command::CreateSnapshot { name, recursive } => client
                .create_snapshot(name, recursive)
                .await
                .map(|snapshot| format!("{snapshot:?}"))?,
            Command::DestroySnapshot { name } => client
                .destroy_snapshot(name)
                .await
                .map(|_| "Snapshot destroyed".to_string())?,
            Command::ListSnapshots { name } => client
                .list_snapshots(name)
                .await
                .map(|snapshot| format!("{snapshot:?}"))?,
            Command::ShowSnapshot { name } => client
                .show_snapshot(name)
                .await
                .map(|snapshot| format!("{snapshot:?}"))?,
            Command::CreateBookmark { snapshot, bookmark } => client
                .create_bookmark(snapshot, bookmark)
                .await
                .map(|bookmark| format!("{bookmark:#?}"))?,
            Command::ShowBookmark { bookmark } => client
                .show_bookmark(bookmark)
                .await
                .map(|bookmark| format!("{bookmark:#?}"))?,
            Command::DestroyBookmark { bookmark } => client
                .destroy_bookmark(bookmark)
                .await
                .map(|()| "Bookmark destroyed".to_string())?,
            Command::Send {
                source,
                output,
                incremental,
            } => process_send(&mut client, source, output, incremental).await?,
            Command::Receive { snapshot, input } => {
                process_recv(&mut client, snapshot, input).await?
            }
        };

        println!("{text}");

        Ok(())
    }
}

async fn process_send(
    client: &mut ZfsClient,
    source: String,
    output: PathBuf,
    incremental: Option<String>,
) -> anyhow::Result<String> {
    let mut segments = client.send_snapshot(source, incremental).await?;
    let mut output = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(output)
        .await?;

    while let Some(segment) = segments.message().await? {
        let sequence = segment.sequence;
        let mut buffer = Cursor::new(segment.buffer);
        debug!("Processing segment {sequence}");
        io::copy(&mut buffer, &mut output).await?;
    }

    Ok(String::from("Finished processing send"))
}

async fn process_recv(
    client: &mut ZfsClient,
    snapshot: String,
    input: PathBuf,
) -> anyhow::Result<String> {
    let mut input = fs::OpenOptions::new().read(true).open(input).await?;

    let segments = async_stream::stream! {
        loop {
            let mut buffer = Vec::with_capacity(128 * 1024);
            if let Ok(count) = input.read_buf(&mut buffer).await {
                if count > 0 {
                    yield buffer;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    };

    client.recv_snapshot(snapshot, segments).await?;

    Ok(String::from("finish processin receive"))
}
