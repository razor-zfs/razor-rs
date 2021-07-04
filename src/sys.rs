// use std::convert::TryFrom;
use std::io;
use std::str::FromStr;

use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub(crate) use bunch::Bunch;

mod bunch;
mod cmd;

const ZFS: &str = "/usr/sbin/zfs";
const ZPOOL: &str = "/usr/sbin/zpool";
const ZFS_GET_DELIMITER: char = '\t';

pub(crate) struct ZfsImpl;

impl ZfsImpl {
    pub(crate) async fn zfs_get_all() -> io::Result<String> {
        cmd::Cmd::new(ZFS, &["get", "-pH", "-o", "all", "all"])
            .exec()
            .await
    }

    pub(crate) async fn zpool_get_all() -> io::Result<String> {
        cmd::Cmd::new(ZPOOL, &["get", "-pH", "-o", "all", "all"])
            .exec()
            .await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawProperty {
    pub property: String,
    pub value: String,
    pub received: Option<String>,
    pub source: String,
}

impl RawProperty {
    fn new((property, value, received, source): (&str, &str, Option<&str>, &str)) -> Self {
        let property = property.to_string();
        let value = value.to_string();
        let received = received.map(|received| received.to_string());
        let source = source.to_string();
        Self {
            property,
            value,
            received,
            source,
        }
    }

    pub fn into_value(self) -> String {
        self.value
    }

    pub fn name(&self) -> &str {
        &self.property
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn received(&self) -> Option<&str> {
        self.received.as_deref()
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

impl From<(&str, &str, &str)> for RawProperty {
    fn from((property, value, source): (&str, &str, &str)) -> Self {
        Self::new((property, value, None, source))
    }
}

impl From<(&str, &str, &str, &str)> for RawProperty {
    fn from((property, value, received, source): (&str, &str, &str, &str)) -> Self {
        Self::new((property, value, Some(received), source))
    }
}

impl FromStr for RawProperty {
    type Err = MalformedRawPropertyText;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let text = text.trim();

        let four = text
            .splitn(4, ZFS_GET_DELIMITER)
            .collect_tuple::<(&str, &str, &str, &str)>()
            .map(Self::from);

        let three = text
            .splitn(4, ZFS_GET_DELIMITER)
            .collect_tuple::<(&str, &str, &str)>()
            .map(Self::from);

        four.or(three).ok_or(MalformedRawPropertyText)
    }
}

#[derive(Debug, Error)]
#[error("Malformed RAW Property Text")]
pub struct MalformedRawPropertyText;

pub(crate) fn parse_zfs_get(text: impl AsRef<str>) -> IndexMap<String, Bunch> {
    text.as_ref()
        .trim()
        .lines()
        // .inspect(|line| println!("parse_zfs_get('{}')", line))
        .filter_map(|line| line.split_once(ZFS_GET_DELIMITER))
        // .inspect(|(name, rest)| println!("{} -> '{}'", name, rest))
        .filter_map(text2props)
        // .inspect(|(name, prop)| println!("{} -> {:?}", name, prop))
        .fold(IndexMap::default(), |mut acc, (dataset, property)| {
            acc.entry(dataset).or_default().insert(property);
            acc
        })
}

pub(crate) fn parse_zpool_get(text: impl AsRef<str>) -> IndexMap<String, Bunch> {
    text.as_ref()
        .trim()
        .lines()
        .inspect(|line| println!("parse_zpool_get('{}')", line))
        .filter_map(|line| line.split_once(ZFS_GET_DELIMITER))
        .inspect(|(name, rest)| println!("{} -> '{}'", name, rest))
        .filter_map(text2props)
        .inspect(|(name, prop)| println!("{} -> {:?}", name, prop))
        .fold(IndexMap::default(), |mut acc, (pool, property)| {
            acc.entry(pool).or_default().insert(property);
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
        // println!("{:#?}", datasets);
        assert_eq!(datasets.len(), 12);
    }

    #[test]
    fn zpool_get() {
        let pools = parse_zpool_get(ZFS_GET);
        println!("{:#?}", pools);
        assert_eq!(pools.len(), 12);
    }
}
