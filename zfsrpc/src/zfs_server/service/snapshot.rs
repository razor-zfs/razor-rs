use super::*;

impl CreateSnapshotRequest {
    pub(crate) async fn execute(self) -> ZfsRpcResult<Snapshot> {
        task::spawn_blocking(|| zfs::Zfs::snapshot().create(self.name))
            .await
            .map_err(join_to_status)?
            .map(Snapshot::from)
            .map(Response::new)
            .map_err(zfs_to_status)
    }
}

impl Snapshot {
    pub(crate) fn get(name: &str) -> Result<Self, ZfsError> {
        let snapshot = zfs::Zfs::get_snapshot(name).map(Into::into)?;
        Ok(snapshot)
    }
}

impl From<zfs::Snapshot> for Snapshot {
    fn from(snapshot: zfs::Snapshot) -> Self {
        Self {
            name: Some(snapshot.name().into()),
            guid: Some(snapshot.guid().into()),
            objsetid: Some(snapshot.objsetid().into()),
        }
    }
}
