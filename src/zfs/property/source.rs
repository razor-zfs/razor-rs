use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    #[serde(rename = "-")]
    None,
    Default,
    Local,
    Inherited,
    Temporary,
    Received,
}

#[derive(Debug, Error)]
#[error("Invalid source variant (`{0}`)")]
pub struct InvalidSource(String);

impl<T: AsRef<str>> From<T> for InvalidSource {
    fn from(text: T) -> Self {
        Self(text.as_ref().to_string())
    }
}

impl FromStr for Source {
    type Err = InvalidSource;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "-" => Ok(Self::None),
            "default" => Ok(Self::Default),
            "local" => Ok(Self::Local),
            "inherited" => Ok(Self::Inherited),
            "temporary" => Ok(Self::Temporary),
            "received" => Ok(Self::Received),
            other => Err(InvalidSource::from(other)),
        }
    }
}
