use std::convert::TryFrom;
use std::io;
use std::sync::Arc;

use indexmap::IndexMap;
use once_cell::sync::Lazy;
use thiserror::Error;
use tokio::sync::Mutex;

use crate::sys;
use crate::zfs;

static ZFS: Lazy<Arc<Mutex<Zfs>>> = Lazy::new(|| Arc::new(Mutex::new(Zfs::default())));

#[derive(Debug, Error)]
pub enum MalformedZFS {
    #[error("Malformed Pool")]
    MalformedZpool,
    #[error("Malformed Dataset")]
    MalformedZvol,
    #[error(transparent)]
    IOError(#[from] io::Error),
}

#[derive(Debug, Default)]
pub struct Zfs {
    pools: IndexMap<zfs::Guid, zfs::Pool>,
    datasets: IndexMap<zfs::Name, zfs::Dataset>,
}

impl Zfs {
    pub fn get() -> Arc<Mutex<Self>> {
        Arc::clone(&*ZFS)
    }

    pub fn pools(&self) -> &IndexMap<zfs::Guid, zfs::Pool> {
        &self.pools
    }

    pub fn datasets(&self) -> &IndexMap<zfs::Name, zfs::Dataset> {
        &self.datasets
    }

    pub async fn load(&mut self) -> Result<(), MalformedZFS> {
        self.load_zfs(None).await?;
        self.load_zpool(None).await?;
        Ok(())
    }

    async fn load_zfs(
        &mut self,
        _dataset: impl IntoIterator<Item = zfs::Name>,
    ) -> Result<&IndexMap<zfs::Name, zfs::Dataset>, MalformedZFS> {
        let text = sys::ZfsImpl::zfs_get_all().await?;
        self.load_from_zfs_get(text)
    }

    async fn load_zpool(
        &mut self,
        _zpool: impl IntoIterator<Item = zfs::Name>,
    ) -> Result<&IndexMap<zfs::Guid, zfs::Pool>, MalformedZFS> {
        let text = sys::ZfsImpl::zpool_get_all().await?;

        self.load_from_zpool_get(text)
    }

    fn load_from_zfs_get(
        &mut self,
        text: impl AsRef<str>,
    ) -> Result<&IndexMap<zfs::Name, zfs::Dataset>, MalformedZFS> {
        let mut datasets = IndexMap::new();

        for (name, properties) in sys::parse_zfs_get(text) {
            println!("Processing {} with {} properties", name, properties);
            let name = zfs::Name::from(name);
            // datasets.insert(name, properties);

            match zfs::Dataset::try_from(properties) {
                Ok(dataset) => {
                    datasets.insert(name, dataset);
                }
                Err(err) => println!("{}", err),
            }
        }

        self.datasets = datasets;

        Ok(&self.datasets)
    }

    fn load_from_zpool_get(
        &mut self,
        text: impl AsRef<str>,
    ) -> Result<&IndexMap<zfs::Guid, zfs::Pool>, MalformedZFS> {
        let mut pools = IndexMap::new();

        for (name, properties) in sys::parse_zpool_get(text) {
            println!("Processing {} with {} properties", name, properties);

            let guid = properties
                .get("guid")
                .ok_or_else(|| MalformedZFS::MalformedZpool)?
                .value();
            let guid = guid.parse().map_err(|_| MalformedZFS::MalformedZpool)?;

            match zfs::Pool::try_from(properties) {
                Ok(pool) => {
                    pools.insert(guid, pool);
                }
                Err(err) => println!("{}", err),
            }
        }

        self.pools = pools;

        Ok(&self.pools)
    }
}
