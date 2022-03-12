use std::path::{Path, PathBuf};

use itertools::Itertools;
use tonic::{Request, Response, Status};
use tracing::{debug, debug_span, error, info, warn};

use crate::zfs_server::error::ZfsError;
use crate::zfsrpc_proto::SendRequest;
use crate::zfsrpc_proto::SendSegment;
use crate::zfsrpc_proto::Snapshot;
use crate::zfsrpc_proto::Volume;
// use crate::zfsrpc_proto::ZfsType;

use super::zfsrpc_proto::tonic_zfsrpc::zfs_rpc_server::ZfsRpc;
use super::zfsrpc_proto::tonic_zfsrpc::{
    BasicDatasetRequest, CreateFilesystemRequest, CreateSnapshotRequest, CreateVolumeRequest,
    ListDatasetsRequest, MountFilesystemRequest,
};
use super::zfsrpc_proto::tonic_zfsrpc::{Datasets, Empty, Filesystem};

mod error;
pub mod service;

type ZfsRpcResult<T, E = ::tonic::Status> = ::std::result::Result<::tonic::Response<T>, E>;

#[tonic::async_trait]
impl ZfsRpc for service::ZfsRpcService {
    type SendStream = service::SendStream;

    async fn dataset_list(&self, _request: Request<Empty>) -> Result<Response<Datasets>, Status> {
        let datasets = service::list()?;

        Ok(Response::new(datasets))
    }

    async fn list_datasets(&self, request: Request<ListDatasetsRequest>) -> ZfsRpcResult<Datasets> {
        let _request = request.into_inner();
        let response = service::list().map(Response::new)?;
        Ok(response)
    }

    async fn create_volume(
        &self,
        request: Request<CreateVolumeRequest>,
    ) -> Result<Response<Volume>, Status> {
        let request = request.into_inner();
        let name = request.name.clone();
        let span = debug_span!("create_volume");
        let _guard = span.entered();
        debug!(?request);

        let vol = Volume::create(
            request.name,
            request.capacity,
            request.blocksize,
            request.properties,
        )
        .or_else(|err| {
            if let ZfsError::AlreadyExists(err) = err {
                warn!("Volume {} already exists: {:?}", name, err);
                Ok(Volume::get(name)?)
            } else {
                Err(err)
            }
        })?;

        Ok(Response::new(vol))
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

    async fn destroy_volume(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        let span = debug_span!("destroy_volume");
        let _guard = span.entered();
        debug!(?request);

        let path = request.name;
        let res = service::destroy(path.clone());
        match res {
            Ok(_) => info!("Volume {} successfully deleted", path),
            Err(ZfsError::NotFound(err)) => {
                warn!("Volume {} not found (already deleted?) : {:?}", path, err)
            }
            Err(err) => {
                error!("{:?}", err);
                return Err(err.into());
            }
        }

        Ok(Response::new(Empty {}))
    }

    async fn create_filesystem(
        &self,
        request: Request<CreateFilesystemRequest>,
    ) -> Result<Response<Filesystem>, Status> {
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

        path_iter
            .scan(pool, |path, dir| {
                path.push(dir);
                Some(path.clone())
            })
            .map(|path| {
                debug!("Creating filesystem at {:?}", path);
                Filesystem::create(
                    path.to_string_lossy().to_string(),
                    request.properties.clone(),
                )
                .map(|_| ())
                .or_else(|e| {
                    warn!("{:?}", e);
                    if let ZfsError::AlreadyExists(e) = e {
                        warn!("Filesystem {:?} already exists : {:?}", path, e);
                        Ok(())
                    } else {
                        Err(e)
                    }
                })
            })
            .try_collect()?;

        Ok(Response::new(Filesystem::get(request.name)?))
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

    async fn destroy_filesystem(
        &self,
        request: Request<BasicDatasetRequest>,
    ) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        let span = debug_span!("destroy_filesystem");
        let _guard = span.entered();
        debug!(?request);

        let path = request.name;
        let res = service::destroy(path.clone());
        match res {
            Ok(_) => info!("Filesystem {} successfully deleted", path),
            Err(ZfsError::NotFound(err)) => {
                warn!(
                    "Filesystem {} not found (already deleted?) : {:?}",
                    path, err
                )
            }
            Err(err) => {
                error!("{:?}", err);
                return Err(err.into());
            }
        }

        Ok(Response::new(Empty {}))
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

    async fn create_snapshot(
        &self,
        request: Request<CreateSnapshotRequest>,
    ) -> ZfsRpcResult<Snapshot> {
        let response = request.into_inner().execute().await?;
        Ok(response)
    }

    async fn get_snapshot(&self, request: Request<BasicDatasetRequest>) -> ZfsRpcResult<Snapshot> {
        let name = request.into_inner().name;
        let response = Snapshot::get(&name).map(Response::new)?;
        Ok(response)
    }

    async fn destroy_snapshot(&self, request: Request<BasicDatasetRequest>) -> ZfsRpcResult<Empty> {
        let _request = request.into_inner();
        todo!()
    }

    async fn send(&self, request: Request<SendRequest>) -> ZfsRpcResult<Self::SendStream> {
        request.into_inner().exec().await
    }
}
