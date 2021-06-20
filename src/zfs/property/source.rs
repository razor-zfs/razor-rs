use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    #[serde(rename = "-")]
    None,
    Default,
    Local,
    Inherited(String),
    Temporary,
    Received,
}

const INHERITED_FROM: &str = "inherited from ";

impl FromStr for Source {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "-" => Ok(Self::None),
            "default" => Ok(Self::Default),
            "local" => Ok(Self::Local),
            "temporary" => Ok(Self::Temporary),
            "received" => Ok(Self::Received),
            other if other.starts_with(INHERITED_FROM) => other
                .strip_prefix(INHERITED_FROM)
                .map(|source| Self::Inherited(source.to_string()))
                .ok_or_else(|| other.to_string()),
            other => Err(other.to_string()),
        }
    }
}
