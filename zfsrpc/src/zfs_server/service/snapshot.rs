use super::*;

impl Snapshot {
    pub fn create(name: &str) -> Result<()> {
        debug!(name);
        Ok(())
    }

    pub fn get(name: &str) -> Result<Self> {
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
