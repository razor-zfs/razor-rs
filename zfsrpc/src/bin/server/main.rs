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

use std::time;

use tonic::transport::Server;
use zfsrpc::server::service;
use zfsrpc::zfsrpc_proto::tonic_zfsrpc::zfs_rpc_server::ZfsRpcServer;
use zfsrpc::zfsrpc_proto::tonic_zfstracer::zfs_tracer_server::ZfsTracerServer;

pub mod zfs_tracing {

    use anyhow;

    use tonic::{Code, Request, Response, Status};
    use tracing_subscriber::{fmt, reload::Handle, EnvFilter};
    use zfsrpc::zfsrpc_proto::tonic_zfstracer::zfs_tracer_server::ZfsTracer;
    use zfsrpc::zfsrpc_proto::tonic_zfstracer::{trace_level, Empty, TraceLevel};

    //const LOG_DIR: &str = "/var/log";
    //const LOG_PREFIX: &str = "zfstrace";

    pub fn init() -> anyhow::Result<Tracer<impl tracing::Subscriber>> {
        // fs::create_dir_all(LOG_DIR).context("Failed to create log dir")?;

        // let file_appender = tracing_appender::rolling::daily(LOG_DIR, LOG_PREFIX);
        // let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        // let collector = tracing_subscriber::registry()
        //     .with(EnvFilter::from_default_env().add_directive(tracing::Level::WARN.into()))
        //     //.with(fmt::Subscriber::new().with_writer(io::stdout))
        //     .with(fmt::Subscriber::builder().with_writer(non_blocking).finish());
        // tracing::subscriber::set_global_default(collector)
        //     .expect("Unable to set a global collector");

        //let filter = EnvFilter::new(tracing::Level::WARN.into()).add_directive(from_default_env().into());
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));

        let builder = fmt()
            .with_env_filter(filter)
            .with_timer(fmt::time::ChronoUtc::default())
            .with_filter_reloading();

        let handler = builder.reload_handle();

        builder.init();

        Ok(Tracer { handler })
    }

    #[derive(Debug)]
    pub struct Tracer<S> {
        handler: Handle<EnvFilter, S>,
    }

    impl<S> Tracer<S>
    where
        S: tracing::Subscriber,
    {
        pub fn reload(&self, level: trace_level::Level) -> anyhow::Result<()> {
            let str = match level {
                trace_level::Level::Trace(_) => "trace",
                trace_level::Level::Debug(_) => "debug",
                trace_level::Level::Info(_) => "info",
                trace_level::Level::Warn(_) => "warn",
                trace_level::Level::Error(_) => "error",
            };

            self.handler
                .reload(EnvFilter::new(str))
                .map_err(|e| e.into())
        }
    }

    #[tonic::async_trait]
    impl<S> ZfsTracer for Tracer<S>
    where
        S: tracing::Subscriber,
    {
        async fn set_tracing_level(
            &self,
            request: Request<TraceLevel>,
        ) -> Result<Response<Empty>, Status> {
            let request = request.into_inner();

            self.reload(
                request
                    .level
                    .ok_or_else(|| Status::new(Code::Internal, "trace level is missing"))?,
            )
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

            Ok(Response::new(Empty {}))
        }
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let rpc = service::ZfsRpcService::default();
    let tracer = zfs_tracing::init()?;
    Server::builder()
        .timeout(time::Duration::from_secs(
            service::ZfsRpcService::DEFAULT_TIMEOUT,
        ))
        .add_service(ZfsRpcServer::new(rpc))
        .add_service(ZfsTracerServer::new(tracer))
        .serve(addr)
        .await?;

    Ok(())
}
