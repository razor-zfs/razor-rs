use super::*;

impl proto::CreateSnapshotRequest {
    pub(crate) async fn execute(self) -> ZfsRpcResult<proto::Snapshot> {
        task::spawn_blocking(|| zfs::Zfs::snapshot().create(self.name))
            .await
            .map_err(join_to_status)?
            .map(proto::Snapshot::from)
            .map(Response::new)
            .map_err(zfs_to_status)
    }
}

impl proto::Snapshot {
    pub(crate) fn get(name: &str) -> Result<Self, ZfsError> {
        let snapshot = zfs::Zfs::get_snapshot(name).map(Into::into)?;
        Ok(snapshot)
    }
}

impl From<zfs::Snapshot> for proto::Snapshot {
    fn from(snapshot: zfs::Snapshot) -> Self {
        Self {
            name: Some(snapshot.name().into()),
            guid: Some(snapshot.guid().into()),
            objsetid: Some(snapshot.objsetid().into()),
        }
    }
}
