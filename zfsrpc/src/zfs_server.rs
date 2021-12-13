use tonic::{Request, Response, Status};

#[allow(unused)]
use tracing::{debug, debug_span, error, info, trace, warn};

use super::zfsrpc_proto::tonic_zfsrpc::zfs_rpc_server::ZfsRpc;
use super::zfsrpc_proto::tonic_zfsrpc::{
    BasicDatasetRequest, CreateFilesystemRequest, CreateVolumeRequest,
};
use super::zfsrpc_proto::tonic_zfsrpc::{Datasets, Empty, Filesystem, Volume};

mod error;
pub mod service;

#[tonic::async_trait]
impl ZfsRpc for service::ZfsRpcService {
    async fn create_volume(
        &self,
        request: Request<CreateVolumeRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        let name = request.name.clone();
        let span = debug_span!("create_volume");
        let _guard = span.entered();
        debug!(?request);

        let res = Volume::create(
            request.name,
            request.capacity,
            request.blocksize,
            request.properties,
        );
        let res = inspector::ResultInspector::inspect(res, |_| {
            info!("Volume {} Created successfully", name)
        });
        inspector::ResultInspector::inspect_err(res, |err| error!("{:?}", err))?;

        Ok(Response::new(Empty {}))
    }

    async fn create_filesystem(
        &self,
        request: Request<CreateFilesystemRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        let name = request.name.clone();
        let span = debug_span!("create_filesystem");
        let _guard = span.entered();
        debug!(?request);

        let res = Filesystem::create(request.name, 0, request.properties);
        let res = inspector::ResultInspector::inspect(res, |_| {
            info!("fileystem {} Created successfully", name)
        });
        inspector::ResultInspector::inspect_err(res, |err| error!("{:?}", err))?;

        Ok(Response::new(Empty {}))
    }

    async fn get_volume(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Volume>, Status> {
        let request = request.into_inner();
        let span = debug_span!("get_volume");
        let _guard = span.entered();
        debug!(?request);

        Ok(Response::new(Volume::get(request.name)?))
    }

    async fn get_filesystem(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Filesystem>, Status> {
        let request = request.into_inner();
        let span = debug_span!("get_filesystem");
        let _guard = span.entered();
        debug!(?request);

        Ok(Response::new(Filesystem::get(request.name)?))
    }

    async fn destroy_volume(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        let span = debug_span!("destroy_volume");
        let _guard = span.entered();
        debug!(?request);

        service::destroy(request.name)?;

        Ok(Response::new(Empty {}))
    }

    async fn destroy_filesystem(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        let span = debug_span!("destroy_filesystem");
        let _guard = span.entered();
        debug!(?request);

        service::destroy(request.name)?;

        Ok(Response::new(Empty {}))
    }

    async fn dataset_list(&self, _request: Request<Empty>) -> Result<Response<Datasets>, Status> {
        let datasets = service::list()?;

        Ok(Response::new(datasets))
    }

    async fn mount_filesystem(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        debug!(?request);

        Filesystem::mount(request.name).await?;

        Ok(Response::new(Empty {}))
    }

    async fn unmount_filesystem(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        debug!(?request);

        Filesystem::unmount(request.name).await?;

        Ok(Response::new(Empty {}))
    }
}
