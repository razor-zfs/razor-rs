use serde::{Deserialize, Serialize};

pub(crate) use source::Source;

mod source;

pub type Available = Property<u128>;
pub type CompressRatio = Property<f64>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Property<T> {
    name: String,
    source: Source,
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
