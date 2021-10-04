use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum InvalidProperty {
    #[error("No such property ({0})")]
    NoSuchProperty(String),
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
