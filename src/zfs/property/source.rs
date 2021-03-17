use std::str::FromStr;

use serde::{Deserialize, Serialize};

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

impl FromStr for Source {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "-" => Ok(Self::None),
            "default" => Ok(Self::Default),
            "local" => Ok(Self::Local),
            "inherited" => Ok(Self::Inherited),
            "temporary" => Ok(Self::Temporary),
            "received" => Ok(Self::Received),
            other => Err(other.to_string()),
        }
    }
}
