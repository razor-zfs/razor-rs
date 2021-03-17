use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Filesystem,
    Volume,
    Snapshot,
    Bookmark,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Filesystem => "filesystem",
            Type::Volume => "volume",
            Type::Snapshot => "snapshot",
            Type::Bookmark => "bookmark",
        };
        text.fmt(f)
    }
}

impl FromStr for Type {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "filesystem" => Ok(Self::Filesystem),
            "volume" => Ok(Self::Volume),
            "snapshot" => Ok(Self::Snapshot),
            "bookmark" => Ok(Self::Bookmark),
            _ => Err(super::InvalidProperty::InvalidValue),
        }
    }
}
