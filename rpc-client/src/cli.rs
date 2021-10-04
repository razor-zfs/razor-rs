use razor_zfsrpc::client::Client;

#[allow(unused)]
use tracing::{debug, error, info, trace, warn};

use structopt::StructOpt;

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
        #[structopt(help = "Filesystem name", long, short)]
        name: String,
    },

    #[structopt(about = "Destroy filesystem", aliases = &["dfs", "destroy-fs"], display_order(31))]
    DestroyFilesystem {
        #[structopt(help = "Filesystem name", long, short)]
        name: String,
    },

    #[structopt(
        about = "Create new volume",
        aliases = &["cv"],
        display_order = 20)]
    CreateVolume {
        #[structopt(long, short, help = "Volume name")]
        name: String,
        #[structopt(long, short, help = "Volume capacity")]
        capacity: u64,
        // #[structopt(short, long, multiple = false, help = "")]
        // properties: Vec<>,
    },

    #[structopt(about = "Get volume properties", aliases = &["gv", "get-vol"], display_order(21))]
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
        #[structopt(long, short, help = "Trace level", env = "RUST_LOG")]
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
                //properties,
            } => {
                client.create_volume(name, capacity, vec![]).await?;
                String::default()
            }
            Command::DestroyFilesystem { name } => {
                client.destroy_filesystem(name).await?;
                String::default()
            }
            Command::DestroyVolume { name } => {
                client.destroy_volume(name).await?;
                String::default()
            }
            Command::SetTraceLevel { level } => {
                if let Some(trace_level) = level {
                    client.set_trace_level(trace_level).await?;
                }

                String::default()
            }
        };

        info!(?resp);

        Ok(())
    }
}
