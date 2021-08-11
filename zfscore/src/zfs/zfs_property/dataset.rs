use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Filesystem,
    Volume,
    Snapshot,
    Bookmark,
}

impl Type {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Filesystem => "filesystem",
            Self::Volume => "volume",
            Self::Snapshot => "snapshot",
            Self::Bookmark => "bookmark",
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
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
            other => Err(super::InvalidProperty::invalid_value(other)),
        }
    }
}
