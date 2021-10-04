use tonic::{Request, Response, Status};

#[allow(unused)]
use tracing::{debug, error, info, trace, warn};

use super::zfsrpc_proto::tonic_zpoolrpc::zpool_rpc_server::ZpoolRpc;
use super::zfsrpc_proto::tonic_zpoolrpc::{CreateRequest, DestroyRequest, Empty};

struct ZpoolRpcService {}

#[tonic::async_trait]
impl ZpoolRpc for ZpoolRpcService {
    async fn create(&self, _request: Request<CreateRequest>) -> Result<Response<Empty>, Status> {
        todo!();
        // Ok(Response::new(Empty {}))
    }

    async fn destroy(&self, _request: Request<DestroyRequest>) -> Result<Response<Empty>, Status> {
        todo!();
        // Ok(Response::new(Empty {}))
    }
}
