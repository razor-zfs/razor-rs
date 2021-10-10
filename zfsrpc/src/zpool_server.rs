use tonic::{Code, Request, Response, Status};

#[allow(unused)]
use tracing::{debug, error, info, trace, warn};

pub(crate) mod zpool_cmds;

#[cfg(feature = "dirty")]
use zpool_cmds as zpool;

use super::zfsrpc_proto::tonic_zpoolrpc::zpool_rpc_server::ZpoolRpc;
use super::zfsrpc_proto::tonic_zpoolrpc::{CreateRequest, DestroyRequest, Empty};

#[derive(Debug, Default)]
pub struct ZpoolRpcService {}

#[tonic::async_trait]
impl ZpoolRpc for ZpoolRpcService {
    async fn create(&self, request: Request<CreateRequest>) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        debug!(?request);

        let properties = request
            .properties
            .into_iter()
            .filter(|p| p.property.is_some())
            .collect();

        zpool::create(&request.name, request.method, request.disks, properties)
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn destroy(&self, request: Request<DestroyRequest>) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        debug!(?request);

        zpool::destroy(&request.name)
            .await
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        Ok(Response::new(Empty {}))
    }
}
