use tonic::{Code, Request, Response, Status};

#[allow(unused)]
use tracing::{debug, error, info, trace, warn};

use crate::zfs_server::service::Dataset;

use super::zfsrpc_proto::tonic_zfsrpc::zfs_rpc_server::ZfsRpc;
use super::zfsrpc_proto::tonic_zfsrpc::{
    BasicDatasetRequest, CreateFilesystemRequest, CreateVolumeRequest,
};
use super::zfsrpc_proto::tonic_zfsrpc::{Datasets, Empty, Filesystem, Volume};

pub mod service;

#[tonic::async_trait]
impl ZfsRpc for service::ZfsRpcService {
    async fn create_volume(
        &self,
        request: Request<CreateVolumeRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        debug!(?request);

        Volume::create(request.name, request.capacity, request.properties)
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn create_filesystem(
        &self,
        request: Request<CreateFilesystemRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        debug!(?request);

        Filesystem::create(request.name, 0, request.properties)
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
            Volume::get(request.name).map_err(|e| Status::new(Code::Internal, e.to_string()))?,
        ))
    }

    async fn get_filesystem(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Filesystem>, Status> {
        let request = request.into_inner();
        debug!(?request);

        Ok(Response::new(
            Filesystem::get(request.name)
                .map_err(|e| Status::new(Code::Internal, e.to_string()))?,
        ))
    }

    async fn destroy_volume(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        debug!(?request);

        Volume::destroy(request.name).map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn destroy_filesystem(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        debug!(?request);

        Filesystem::destroy(request.name)
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn dataset_list(&self, _request: Request<Empty>) -> Result<Response<Datasets>, Status> {
        let datasets = service::list().map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        Ok(Response::new(datasets))
    }
}
