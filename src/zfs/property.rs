use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

pub(crate) use source::Source;

mod source;

pub type Guid = Property<u64>;
pub type Name = Property<String>;
pub type Available = Property<u128>;
pub type CompressRatio = Property<f64>;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Property<T> {
    name: String,
    source: Source,
    #[serde(bound = "T: Display + FromStr, <T as FromStr>::Err: Display")]
    #[serde_as(as = "DisplayFromStr")]
    value: T,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Filesystem,
    Volume,
    Snapshot,
    Bookmark,
}
