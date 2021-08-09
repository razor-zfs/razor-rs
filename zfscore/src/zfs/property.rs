use std::ops;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use thiserror::Error;

mod source;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Property<T> {
    pub(crate) value: T,
}

impl<T> Property<T> {
    pub fn new(value: T) -> Self {
        Property { value }
    }

    pub fn value(&self) -> &T {
        &self.value
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
    pub(crate) fn _no_such_property(prop: impl ToString) -> Self {
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
