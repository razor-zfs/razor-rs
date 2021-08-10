use std::fmt;
use std::ops;

use indexmap::IndexMap;
use itertools::Itertools;
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

impl fmt::Display for Bunch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let props = format!("{} properties", self.0.len());
        let keys = self.0.keys().take(3).join(",");
        f.debug_tuple("Bunch").field(&props).field(&keys).finish()
    }
}

impl ops::Deref for Bunch {
    type Target = IndexMap<String, super::RawProperty>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Bunch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
