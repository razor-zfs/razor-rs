use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::sys;

pub use source::{InvalidSource, Source};

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

impl<T> TryFrom<sys::RawProperty> for Property<T>
where
    T: FromStr,
{
    type Error = InvalidProperty;

    fn try_from(raw: sys::RawProperty) -> Result<Self, Self::Error> {
        let name = raw.property;
        let source = raw
            .source
            .parse()
            .map_err(|_| InvalidProperty::InvalidSource)?;
        let value = raw
            .value
            .parse()
            .map_err(|_| InvalidProperty::InvalidValue)?;
        let property = Self {
            name,
            source,
            value,
        };
        Ok(property)
    }
}

#[derive(Debug, Error)]
pub enum InvalidProperty {
    #[error("No such property")]
    NoSuchProperty,
    #[error("Invalid source")]
    InvalidSource,
    #[error("Invalid value")]
    InvalidValue,
}
