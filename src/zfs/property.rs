use std::str::FromStr;

use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub(crate) use bunch::Bunch;
pub(crate) use source::Source;

mod bunch;
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

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RawProperty {
    property: String,
    value: String,
    received: String,
    source: String,
}

impl RawProperty {
    fn new((property, value, received, source): (&str, &str, &str, &str)) -> Self {
        let property = property.to_string();
        let value = value.to_string();
        let received = received.to_string();
        let source = source.to_string();
        Self {
            property,
            value,
            received,
            source,
        }
    }
}

impl FromStr for RawProperty {
    type Err = MalformedRawPropertyText;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        text.trim()
            .splitn(4, super::ZFS_GET_DELIMITER)
            .collect_tuple()
            .map(Self::new)
            .ok_or(MalformedRawPropertyText)
    }
}

#[derive(Debug, Error)]
#[error("Malformed RAW Property Text")]
pub struct MalformedRawPropertyText;

pub(crate) fn parse_zfs_get(text: impl AsRef<str>) -> IndexMap<String, Bunch> {
    text.as_ref()
        .trim()
        .lines()
        .filter_map(|line| line.split_once(super::ZFS_GET_DELIMITER))
        .filter_map(text2props)
        .fold(IndexMap::default(), |mut acc, (dataset, property)| {
            acc.entry(dataset).or_default().insert(property);
            acc
        })
}

fn text2props((dataset, text): (&str, &str)) -> Option<(String, RawProperty)> {
    text.parse()
        .ok()
        .map(|property| (dataset.to_string(), property))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZFS_GET: &str = include_str!("zfs-get.out");

    #[test]
    fn zfs_get() {
        let datasets = parse_zfs_get(ZFS_GET);
        println!("{:#?}", datasets);
        assert_eq!(datasets.len(), 12);
    }
}
