use razor_zfsrpc::zfsrpc_proto::tonic_zfsrpc::{
    zfs_rpc_client::ZfsRpcClient, BasicDatasetRequest, CreateVolumeRequest, Empty, VolumeProperty,
};

use razor_zfsrpc::zfsrpc_proto::tonic_zfstracer::{
    trace_level::Level, zfs_tracer_client::ZfsTracerClient, TraceLevel, Variant,
};

use tonic::transport::Channel;
#[allow(unused)]
use tracing::{debug, error, info, trace, warn};

#[allow(unused)]
use razor_zfs::zfs;

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

// another client?
impl Cli {
    pub(crate) async fn execute() -> anyhow::Result<()> {
        let this = Self::from_args();
        trace!("{:?}", this);

        let mut api = Api::new(this.port).await;

        let resp: String = match this.command {
            Command::ZfsList => api.list().await?,
            Command::GetVolume { name } => api.get_volume(name).await?,
            Command::GetFilesystem { name } => api.get_filesystem(name).await?,
            Command::CreateVolume {
                name,
                capacity,
                //properties,
            } => {
                api.create_volume(name, capacity, vec![]).await?;
                String::default()
            }
            Command::DestroyFilesystem { name } => {
                api.destroy_filesystem(name).await?;
                String::default()
            }
            Command::DestroyVolume { name } => {
                api.destroy_volume(name).await?;
                String::default()
            }
            Command::SetTraceLevel { level } => {
                if let Some(trace_level) = level {
                    let trace_level: &str = trace_level.as_ref();
                    let trace_level = match trace_level.to_lowercase().as_ref() {
                        "trace" => Level::Trace(Variant {}),
                        "debug" => Level::Debug(Variant {}),
                        "info" => Level::Info(Variant {}),
                        "warn" => Level::Warn(Variant {}),
                        "error" => Level::Error(Variant {}),
                        _ => unreachable!(),
                    };
                    api.set_trace_level(trace_level).await?;
                }

                String::default()
            }
        };

        info!(?resp);

        Ok(())
    }
}

// Move to lib crate
#[derive(Debug)]
pub struct Api {
    client: ZfsRpcClient<Channel>,
    tracer_client: ZfsTracerClient<Channel>,
}

impl Api {
    pub async fn new(port: String) -> Self {
        let tracer_client = ZfsTracerClient::connect(format!("http://0.0.0.0:{}", port))
            .await
            .unwrap();
        let client = ZfsRpcClient::connect(format!("http://0.0.0.0:{}", port))
            .await
            .unwrap();
        Self {
            client,
            tracer_client,
        }
    }

    pub async fn list(&mut self) -> anyhow::Result<String> {
        let request = Empty {};
        let request = tonic::Request::new(request);
        let resp = self.client.dataset_list(request).await?;
        let resp = resp.into_inner();

        let resp = format!("{:?}", resp);
        Ok(resp)
    }

    pub async fn get_filesystem(&mut self, name: String) -> anyhow::Result<String> {
        let request = BasicDatasetRequest { name };
        let request = tonic::Request::new(request);

        let fs = self.client.get_filesystem(request).await?;
        let fs = fs.into_inner();

        let resp = format!("{:?}", fs);
        Ok(resp)
    }

    pub async fn destroy_filesystem(&mut self, name: String) -> anyhow::Result<()> {
        let request = BasicDatasetRequest { name };
        let request = tonic::Request::new(request);

        self.client.destroy_filesystem(request).await?;

        Ok(())
    }

    pub async fn create_volume(
        &mut self,
        name: String,
        capacity: u64,
        properties: Vec<VolumeProperty>,
    ) -> anyhow::Result<()> {
        let request = CreateVolumeRequest {
            name,
            capacity,
            properties,
        };
        let request = tonic::Request::new(request);

        self.client.create_volume(request).await?;

        Ok(())
    }

    pub async fn get_volume(&mut self, name: String) -> anyhow::Result<String> {
        let request = BasicDatasetRequest { name };
        let request = tonic::Request::new(request);

        let vol = self.client.get_volume(request).await?;
        let vol = vol.into_inner();

        let resp = format!("{:?}", vol);
        Ok(resp)
    }

    pub async fn destroy_volume(&mut self, name: String) -> anyhow::Result<()> {
        let request = BasicDatasetRequest { name };
        let request = tonic::Request::new(request);

        self.client.destroy_volume(request).await?;

        Ok(())
    }

    pub async fn set_trace_level(&mut self, level: Level) -> anyhow::Result<()> {
        let request = TraceLevel { level: Some(level) };
        let request = tonic::Request::new(request);

        self.tracer_client.set_tracing_level(request).await?;

        Ok(())
    }
}
