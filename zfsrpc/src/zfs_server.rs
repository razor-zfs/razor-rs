use std::path::{Path, PathBuf};

use itertools::Itertools;
use tonic::{Request, Response};
use tracing::{debug, debug_span, error, info, warn};

use super::zfsrpc_proto::tonic_zfsrpc::zfs_rpc_server::ZfsRpc;

use crate::zfs_server::error::ZfsError;
use crate::zfsrpc_proto as proto;

mod error;
pub mod service;

type ZfsRpcResult<T, E = ::tonic::Status> = ::std::result::Result<::tonic::Response<T>, E>;

#[tonic::async_trait]
impl ZfsRpc for service::ZfsRpcService {
    type SendStream = service::SendStream;

    async fn dataset_list(&self, _request: Request<proto::Empty>) -> ZfsRpcResult<proto::Datasets> {
        let datasets = service::list()?;

        Ok(Response::new(datasets))
    }

    async fn list_datasets(
        &self,
        request: Request<proto::ListDatasetsRequest>,
    ) -> ZfsRpcResult<proto::Datasets> {
        let _request = request.into_inner();
        let response = service::list().map(Response::new)?;
        Ok(response)
    }

    async fn create_volume(
        &self,
        request: Request<proto::CreateVolumeRequest>,
    ) -> ZfsRpcResult<proto::Volume> {
        let request = request.into_inner();
        let name = request.name.clone();
        let span = debug_span!("create_volume");
        let _guard = span.entered();
        debug!(?request);

        let vol = proto::Volume::create(
            request.name,
            request.capacity,
            request.blocksize,
            request.properties,
        )
        .or_else(|err| {
            if let ZfsError::AlreadyExists(err) = err {
                warn!("Volume {} already exists: {:?}", name, err);
                Ok(proto::Volume::get(name)?)
            } else {
                Err(err)
            }
        })?;

        Ok(Response::new(vol))
    }

    async fn get_volume(
        &self,
        request: Request<proto::BasicDatasetRequest>,
    ) -> ZfsRpcResult<proto::Volume> {
        let request = request.into_inner();
        let span = debug_span!("get_volume");
        let _guard = span.entered();
        debug!(?request);

        Ok(Response::new(proto::Volume::get(request.name)?))
    }

    async fn destroy_volume(
        &self,
        request: Request<proto::BasicDatasetRequest>,
    ) -> ZfsRpcResult<proto::Empty> {
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

        Ok(Response::new(proto::Empty {}))
    }

    async fn create_filesystem(
        &self,
        request: Request<proto::CreateFilesystemRequest>,
    ) -> ZfsRpcResult<proto::Filesystem> {
        let request = request.into_inner();
        let path = request.name.clone();
        let span = debug_span!("create_filesystem");
        let _guard = span.entered();
        debug!(?request);

        let mut path_iter = Path::new(&path).iter();
        if path_iter.clone().count() <= 1 {
            error!("No dataset found in path {}", path);
            return Err(tonic::Status::not_found("No dataset found in path"));
        };

        // The first element in the path is the pool name
        // e.g.: pool/dataset/subdataset
        let pool = path_iter
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| tonic::Status::not_found("No zpool found in path"))?;

        path_iter
            .scan(pool, |path, dir| {
                path.push(dir);
                Some(path.clone())
            })
            .map(|path| {
                debug!("Creating filesystem at {:?}", path);
                proto::Filesystem::create(
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

        Ok(Response::new(proto::Filesystem::get(request.name)?))
    }

    async fn get_filesystem(
        &self,
        request: Request<proto::BasicDatasetRequest>,
    ) -> ZfsRpcResult<proto::Filesystem> {
        let request = request.into_inner();
        let span = debug_span!("get_filesystem");
        let _guard = span.entered();
        debug!(?request);

        Ok(Response::new(proto::Filesystem::get(request.name)?))
    }

    async fn destroy_filesystem(
        &self,
        request: Request<proto::BasicDatasetRequest>,
    ) -> ZfsRpcResult<proto::Empty> {
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

        Ok(Response::new(proto::Empty {}))
    }

    async fn mount_filesystem(
        &self,
        request: Request<proto::MountFilesystemRequest>,
    ) -> ZfsRpcResult<proto::Empty> {
        let request = request.into_inner();
        debug!(?request);

        proto::Filesystem::mount(request.name, request.mountpoint).await?;

        Ok(Response::new(proto::Empty {}))
    }

    async fn unmount_filesystem(
        &self,
        request: Request<proto::BasicDatasetRequest>,
    ) -> ZfsRpcResult<proto::Empty> {
        let request = request.into_inner();
        debug!(?request);

        proto::Filesystem::unmount(request.name).await?;

        Ok(Response::new(proto::Empty {}))
    }

    async fn create_snapshot(
        &self,
        request: Request<proto::CreateSnapshotRequest>,
    ) -> ZfsRpcResult<proto::Snapshot> {
        let response = request.into_inner().execute().await?;
        Ok(response)
    }

    async fn get_snapshot(
        &self,
        request: Request<proto::BasicDatasetRequest>,
    ) -> ZfsRpcResult<proto::Snapshot> {
        let name = request.into_inner().name;
        let response = proto::Snapshot::get(&name).map(Response::new)?;
        Ok(response)
    }

    async fn destroy_snapshot(
        &self,
        request: Request<proto::BasicDatasetRequest>,
    ) -> ZfsRpcResult<proto::Empty> {
        let _request = request.into_inner();
        todo!()
    }

    async fn send(&self, request: Request<proto::SendRequest>) -> ZfsRpcResult<Self::SendStream> {
        request.into_inner().execute().await
    }

    async fn recv(
        &self,
        request: Request<tonic::Streaming<proto::SendSegment>>,
    ) -> ZfsRpcResult<proto::Empty> {
        let input = request.into_inner();
        service::recv(input).await
    }
}
