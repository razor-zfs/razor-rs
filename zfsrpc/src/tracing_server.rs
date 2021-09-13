use tonic::{Code, Request, Response, Status};

use super::zfsrpc_proto::tonic_zfstracer::zfs_tracer_server::ZfsTracer;
use super::zfsrpc_proto::tonic_zfstracer::{trace_level, Empty, TraceLevel};

use razor_tracing::Tracer;

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

        let str = match request
            .level
            .ok_or_else(|| Status::new(Code::Internal, "trace level is missing"))?
        {
            trace_level::Level::Trace(_) => "trace",
            trace_level::Level::Debug(_) => "debug",
            trace_level::Level::Info(_) => "info",
            trace_level::Level::Warn(_) => "warn",
            trace_level::Level::Error(_) => "error",
        };

        self.reload(str.to_string())
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        Ok(Response::new(Empty {}))
    }
}
