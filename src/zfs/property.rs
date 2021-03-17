use std::convert::TryFrom;
use std::fmt;
use std::ops;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::sys;

pub use source::{InvalidSource, Source};

mod checksum;
mod dataset;
mod onoff;
mod onoffnoauto;
mod source;
mod timestamp;
mod yesno;

pub use checksum::CheckSum as CheckSumAlgo;
pub use dataset::Type as DatasetType;
pub use onoff::OnOff;
pub use onoffnoauto::OnOffNoAuto;
pub use timestamp::TimeStamp;
pub use yesno::YesNo;

pub type Guid = Property<u64>;
pub type Name = Property<String>;
pub type Available = Property<u64>;
pub type CompressRatio = Property<f64>;
pub type Type = Property<DatasetType>;
pub type Used = Property<u64>;
pub type LogicalUsed = Property<u64>;
pub type Referenced = Property<u64>;
pub type LogicalReferenced = Property<u64>;
pub type CreateTxg = Property<u64>;
pub type Creation = Property<TimeStamp>;
pub type Volsize = Property<u64>;
pub type VolBlockSize = Property<u64>;
pub type Written = Property<u64>;
pub type ObjSetId = Property<u64>;
pub type Atime = Property<OnOff>;
pub type CanMount = Property<OnOffNoAuto>;
pub type Mounted = Property<YesNo>;
pub type CheckSum = Property<CheckSumAlgo>;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Property<T> {
    name: String,
    source: Source,
    #[serde(bound = "T: fmt::Display + FromStr, <T as FromStr>::Err: fmt::Display")]
    #[serde_as(as = "DisplayFromStr")]
    value: T,
}

impl<T> Property<T> {
    pub fn value(&self) -> &T {
        &self.value
    }
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
    #[error("No such property ({0})")]
    NoSuchProperty(String),
    #[error("Invalid source")]
    InvalidSource,
    #[error("Invalid value")]
    InvalidValue,
}

impl InvalidProperty {
    pub(crate) fn no_such_property(prop: impl ToString) -> Self {
        Self::NoSuchProperty(prop.to_string())
    }
}

impl<T> ops::Deref for Property<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}
