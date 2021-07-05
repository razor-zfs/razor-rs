use std::convert::TryFrom;
use std::io;
use std::sync::Arc;

use indexmap::IndexMap;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::sys;
use crate::zfs;

static ZFS: Lazy<Arc<Mutex<Zfs>>> = Lazy::new(|| Arc::new(Mutex::new(Zfs::default())));

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

    pub async fn load(&mut self) -> io::Result<()> {
        self.load_zfs(None).await?;
        self.load_zpool(None).await?;
        Ok(())
    }

    async fn load_zfs(&mut self, _dataset: impl IntoIterator<Item = zfs::Name>) -> io::Result<()> {
        let text = sys::ZfsImpl::zfs_get_all().await?;
        self.load_from_zfs_get(text);
        Ok(())
    }

    async fn load_zpool(&mut self, _zpool: impl IntoIterator<Item = zfs::Name>) -> io::Result<()> {
        let text = sys::ZfsImpl::zpool_get_all().await?;
        self.load_from_zpool_get(text);
        Ok(())
    }

    fn load_from_zfs_get(&mut self, text: impl AsRef<str>) {
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
    }

    fn load_from_zpool_get(&mut self, text: impl AsRef<str>) {
        let mut pools = IndexMap::new();

        for (name, properties) in sys::parse_zpool_get(text) {
            println!("Processing {} with {} properties", name, properties);

            let guid = properties.get("guid").unwrap().value();
            let guid = guid.parse().unwrap();

            match zfs::Pool::try_from(properties) {
                Ok(pool) => {
                    pools.insert(guid, pool);
                }
                Err(err) => println!("{}", err),
            }
        }

        self.pools = pools;
    }
}
