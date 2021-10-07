use super::zpool_client as client;
use client::Client;

#[allow(unused)]
use tracing::{debug, error, info, trace, warn};

use structopt::StructOpt;

const ABOUT: &str = "zpool rpc CLI tool";

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
    #[structopt(about = "Create new zpool")]
    Create {
        #[structopt(help = "zpool name")]
        name: String,
        #[structopt(help = "Creation method", possible_values = &["raidz", "mirror"])]
        method: client::Method,
        #[structopt(help = "Available disks")]
        disks: Vec<String>,
        #[structopt(short, long, help = "ashift", default_value = "12")]
        ashift: u32,
        #[structopt(long, aliases = &["mp"], help = "mountpoint", default_value = "none")]
        mountpoint: String,
    },
    #[structopt(about = "Create new zpool")]
    Destroy {
        #[structopt(help = "zpool name")]
        name: String,
    },
}

#[allow(unused)]
impl Cli {
    pub(crate) async fn execute() -> anyhow::Result<()> {
        let this = Self::from_args();
        trace!("{:?}", this);

        let mut client = Client::new(this.port).await;

        let resp: String = match this.command {
            Command::Create {
                name,
                method,
                disks,
                ashift,
                mountpoint,
            } => {
                let properties = vec![];
                client.create(&name, method, disks, properties).await?
            }

            Command::Destroy { name } => {
                client.destroy(&name).await?;
                String::from("Ok")
            }
        };

        info!(?resp);

        Ok(())
    }
}
