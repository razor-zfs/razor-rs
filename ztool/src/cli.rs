use clap::{Parser, Subcommand};
use razor_zpool_client::{Client, Property};

use tracing::{info, trace};

const ABOUT: &str = "zpool rpc CLI tool";

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
    #[clap(about = "Create new zpool")]
    Create {
        #[clap(help = "zpool name")]
        name: String,
        #[clap(help = "Creation method", possible_values = &["raidz", "mirror"])]
        method: Option<razor_zpool_client::Method>,
        #[clap(long, help = "Available disks")]
        disks: Vec<String>,
        #[clap(short, long, help = "ashift", default_value = "12")]
        ashift: u32,
        #[clap(short, long, help = "mountpoint", default_value = "none")]
        mountpoint: String,
        #[clap(short, long, help = "cachefile", default_value = "none")]
        cachefile: String,
    },
    #[clap(about = "Create new zpool")]
    Destroy {
        #[clap(help = "zpool name")]
        name: String,
    },
    #[clap(about = "Get Ebs path by id")]
    GetEbsPath {
        #[clap(help = "EBS ID (e.g. nvme-Amazon_Elastic_Block_Store_volxxxxxxxxxxxxxxxxx)")]
        ebs_id: String,
    },
}

impl Cli {
    pub(crate) async fn execute() -> anyhow::Result<()> {
        let this = Self::parse();
        trace!("{:?}", this);

        let mut client = Client::new("localhost".into(), this.port).await;

        let resp: String = match this.command {
            Command::Create {
                name,
                method,
                disks,
                ashift,
                mountpoint,
                cachefile,
            } => {
                let properties = vec![
                    Property::Ashift(ashift),
                    Property::Mountpoint(mountpoint),
                    Property::Cachefile(cachefile),
                ];
                client.create(&name, method, disks, properties).await?
            }

            Command::Destroy { name } => {
                client.destroy(&name).await?;
                String::from("Ok")
            }

            Command::GetEbsPath { ebs_id } => {
                let path = client.get_ebs_path(&ebs_id).await?;
                format!("{:?}", path)
            }
        };

        info!(?resp);

        Ok(())
    }
}
