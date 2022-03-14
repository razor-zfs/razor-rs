use super::*;

impl proto::CreateBookmarkRequest {
    pub(crate) async fn execute(self) -> ZfsRpcResult<proto::Bookmark> {
        task::spawn_blocking(|| zfs::Zfs::create_bookmark(self.snapshot, self.bookmark))
            .await
            .map_err(join_to_status)?
            .map(proto::Bookmark::from)
            .map(Response::new)
            .map_err(zfs_to_status)
    }
}

impl proto::Bookmark {
    pub(crate) fn _get(name: &str) -> Result<Self, ZfsError> {
        let bookmark = zfs::Zfs::get_bookmark(name).map(Into::into)?;
        Ok(bookmark)
    }
}

impl From<zfs::Bookmark> for proto::Bookmark {
    fn from(bookmark: zfs::Bookmark) -> Self {
        Self {
            name: Some(bookmark.name().into()),
            guid: Some(bookmark.guid().into()),
            creation: Some(bookmark.creation().into()),
            createtxg: Some(bookmark.createtxg().into()),
        }
    }
}
