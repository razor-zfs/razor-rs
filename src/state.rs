use std::io;

use indexmap::IndexMap;

use crate::zfs::{self, Dataset, Guid, Name, ZPool};

#[derive(Debug, Default)]
pub struct ZfsState {
    pools: IndexMap<Guid, ZPool>,
    datasets: IndexMap<Name, Dataset>,
}

impl ZfsState {
    pub fn load(&mut self) -> io::Result<()> {
        self.load_impl(None)
    }

    fn load_impl(&mut self, _dataset: impl IntoIterator<Item = Name>) -> io::Result<()> {
        let text = "zfs get -pH -o all all";
        self.load_from_zfs_get(text);
        Ok(())
    }

    fn load_from_zfs_get(&mut self, text: impl AsRef<str>) {
        let datasets = zfs::property::parse_zfs_get(text)
            .into_iter()
            .map(|(name, bunch)| (Name::from(name), Dataset::from(bunch)))
            .collect();
        self.datasets = datasets;
    }
}
