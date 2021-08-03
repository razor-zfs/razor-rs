/*use super::libnvpair;
use super::zfs_property;
use super::Result;

pub struct Dataset {}

impl Dataset {
    pub fn builder() -> Result<DatasetBuilder> {
        DatasetBuilder::default()
    }
}

pub struct DatasetBuilder {
    nvlist: libnvpair::NvList,
}

impl DatasetBuilder {
    pub fn default() -> Result<Self> {
        Ok(DatasetBuilder {
            nvlist: libnvpair::NvList::nvlist_alloc(libnvpair::NvFlag::UniqueName)?,
        })
    }

    pub fn blocksize(mut self, blocksize: u64) -> Result<Self> {
        Ok(self)
    }

    pub fn name<T>(mut self, name: T) -> Result<Self>
    where
        T: AsRef<str>,
    {
        Ok(self)
    }

    pub fn compressratio(mut self, compressratio: u64) -> Result<Self> {
        Ok(self)
    }
}
*/
