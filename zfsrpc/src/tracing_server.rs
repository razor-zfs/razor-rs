use tonic::{Request, Response, Status};

use super::zfsrpc_proto::tonic_zfstracer::zfs_tracer_server::ZfsTracer;
use super::zfsrpc_proto::tonic_zfstracer::{Level, TraceLevel};

use razor_tracing::Tracer;

#[tonic::async_trait]
impl<S> ZfsTracer for Tracer<S>
where
    S: tracing::Subscriber,
{
    async fn set_tracing_level(
        &self,
        request: Request<TraceLevel>,
    ) -> Result<Response<()>, Status> {
        let level = request.into_inner().level;
        let level = Level::from_i32(level)
            .ok_or_else(|| Status::invalid_argument(format!("Unknown level {level}")))?
            .as_str();
        self.reload(level)
            .map(Response::new)
            .map_err(|e| Status::internal(e.to_string()))
    }
}

impl Level {
    fn as_str(&self) -> &'static str {
        match self {
            Level::Trace => "trace",
            Level::Debug => "debug",
            Level::Info => "info",
            Level::Warn => "warn",
            Level::Error => "error",
        }
    }
}
