use std::path::{Path, PathBuf};

use tonic::{Request, Response, Status};

use tracing::{debug, debug_span, error, info, warn};

use crate::zfs_server::error::ZfsError;

use super::zfsrpc_proto::tonic_zfsrpc::zfs_rpc_server::ZfsRpc;
use super::zfsrpc_proto::tonic_zfsrpc::{
    BasicDatasetRequest, CreateFilesystemRequest, CreateVolumeRequest, MountFilesystemRequest,
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

        match res {
            Ok(_) => info!("Created volume {}", name),
            Err(ZfsError::AlreadyExists(e)) => {
                warn!("Volume {} already exists : {:?}", name, e);
            }
            Err(_) => return Err(Status::internal("Internal error")),
        };

        Ok(Response::new(Empty {}))
    }

    async fn create_filesystem(
        &self,
        request: Request<CreateFilesystemRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        let path = request.name.clone();
        let span = debug_span!("create_filesystem");
        let _guard = span.entered();
        debug!(?request);

        let mut path_iter = Path::new(&path).iter();
        if path_iter.clone().count() <= 1 {
            error!("No dataset found in path {}", path);
            return Err(Status::not_found("No dataset found in path"));
        };

        // The first element in the path is the pool name
        // e.g.: pool/dataset/subdataset
        let pool = path_iter
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Status::not_found("No zpool found in path"))?;

        let results: Vec<_> = path_iter
            .scan(pool, |path, dir| {
                path.push(dir);
                Some(path.clone())
            })
            .map(|path| {
                debug!("Creating filesystem at {:?}", path);
                Filesystem::create(
                    path.to_string_lossy().to_string(),
                    0,
                    request.properties.clone(),
                )
            })
            .collect();

        for result in results {
            match result {
                Ok(_) => info!("Filesystem {} Created successfully", path),
                Err(ZfsError::AlreadyExists(err)) => {
                    warn!("filesystem {} already exists : {:?}", path, err)
                }
                Err(err) => {
                    error!("{:?}", err);
                    return Err(err.into());
                }
            }
        }

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
        request: Request<MountFilesystemRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        debug!(?request);

        Filesystem::mount(request.name, request.mountpoint).await?;

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
