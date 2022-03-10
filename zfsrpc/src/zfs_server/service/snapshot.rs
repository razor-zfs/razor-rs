use super::*;

impl Snapshot {
    pub(crate) fn _create(name: &str) -> Result<(), ZfsError> {
        debug!(name);
        Ok(())
    }

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
