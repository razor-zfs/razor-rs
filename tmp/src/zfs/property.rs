use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::ops;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::sys;
use source::Source;

mod source;

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
        let source = raw.source.parse().map_err(InvalidProperty::InvalidSource)?;
        let value = raw
            .value
            .parse()
            .map_err(|_| InvalidProperty::invalid_value(&raw.value))?;
        let name = raw.property;
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
    #[error("Invalid source ({0})")]
    InvalidSource(String),
    #[error("Invalid value ({0})")]
    InvalidValue(String),
}

impl InvalidProperty {
    pub(crate) fn no_such_property(prop: impl ToString) -> Self {
        Self::NoSuchProperty(prop.to_string())
    }

    pub(crate) fn invalid_value(value: impl ToString) -> Self {
        Self::InvalidValue(value.to_string())
    }
}

impl<T> ops::Deref for Property<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

pub fn extract_from_bunch<T>(
    bunch: &mut sys::Bunch,
    key: &str,
) -> Result<Property<T>, InvalidProperty>
where
    T: FromStr,
{
    let prop = bunch
        .remove(key)
        .ok_or_else(|| InvalidProperty::no_such_property(key))?
        .try_into()?;
    Ok(prop)
}
