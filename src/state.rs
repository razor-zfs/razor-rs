use std::io;

use indexmap::IndexMap;
use once_cell::sync::Lazy;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};

use crate::zfs;

static ZFS: Lazy<Mutex<Zfs>> = Lazy::new(|| Mutex::new(Zfs::default()));

#[derive(Debug, Default)]
pub struct Zfs {
    pools: IndexMap<zfs::Guid, zfs::ZPool>,
    datasets: IndexMap<zfs::Name, zfs::Dataset>,
}

impl Zfs {
    pub fn get() -> MutexGuard<'static, Self> {
        ZFS.lock()
    }

    pub fn pools() -> MappedMutexGuard<'static, IndexMap<zfs::Guid, zfs::ZPool>> {
        let all = Self::get();
        MutexGuard::map(all, |all| &mut all.pools)
    }

    pub fn datasets() -> MappedMutexGuard<'static, IndexMap<zfs::Name, zfs::Dataset>> {
        let all = Self::get();
        MutexGuard::map(all, |all| &mut all.datasets)
    }

    pub fn load(&mut self) -> io::Result<()> {
        self.load_impl(None)
    }

    fn load_impl(&mut self, _dataset: impl IntoIterator<Item = zfs::Name>) -> io::Result<()> {
        let text = "zfs get -pH -o all all";
        self.load_from_zfs_get(text);
        Ok(())
    }

    fn load_from_zfs_get(&mut self, text: impl AsRef<str>) {
        let datasets = zfs::property::parse_zfs_get(text)
            .into_iter()
            .map(|(name, bunch)| (zfs::Name::from(name), zfs::Dataset::from(bunch)))
            .collect();
        self.datasets = datasets;
    }
}
