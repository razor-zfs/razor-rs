use tonic::{Code, Request, Response, Status};

#[allow(unused)]
use tracing::{debug, error, info, trace, warn};

use super::zfsrpc_proto::tonic_zfsrpc::zfs_rpc_server::ZfsRpc;
use super::zfsrpc_proto::tonic_zfsrpc::{
    BasicDatasetRequest, CreateFilesystemRequest, CreateVolumeRequest,
};
use super::zfsrpc_proto::tonic_zfsrpc::{Empty, Filesystem, Volume};

pub mod service;

#[tonic::async_trait]
impl ZfsRpc for service::ZfsRpcService {
    async fn create_volume(
        &self,
        request: Request<CreateVolumeRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        debug!(?request);

        service::Volume::create(request.name, request.capacity, request.properties)
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn create_filesystem(
        &self,
        request: Request<CreateFilesystemRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        debug!(?request);

        service::Filesystem::create(request.name, request.properties)
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn get_volume(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Volume>, Status> {
        let request = request.into_inner();
        debug!(?request);

        Ok(Response::new(
            service::Volume::get(request.name)
                .map_err(|e| Status::new(Code::Internal, e.to_string()))?
                .into(),
        ))
    }

    async fn get_filesystem(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Filesystem>, Status> {
        let request = request.into_inner();
        debug!(?request);

        Ok(Response::new(
            service::Filesystem::get(request.name)
                .map_err(|e| Status::new(Code::Internal, e.to_string()))?
                .into(),
        ))
    }

    async fn destroy_volume(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        debug!(?request);

        service::Volume::destroy(request.name)
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn destroy_filesystem(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        debug!(?request);

        service::Filesystem::destroy(request.name)
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        Ok(Response::new(Empty {}))
    }
}
