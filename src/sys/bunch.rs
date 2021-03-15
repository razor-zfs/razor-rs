use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Bunch(IndexMap<String, super::RawProperty>);

impl Bunch {
    pub fn insert(&mut self, property: super::RawProperty) {
        let name = property.property.clone();
        self.0.insert(name, property);
    }
}

impl IntoIterator for Bunch {
    type Item = (String, super::RawProperty);
    type IntoIter = indexmap::map::IntoIter<String, super::RawProperty>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::ops::Deref for Bunch {
    type Target = IndexMap<String, super::RawProperty>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
