use super::error;

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Filesystem,
    Snapshot,
    Volume,
    Pool,
    Bookmark,
    Unknown,
}

impl Type {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Filesystem => "filesystem",
            Self::Volume => "volume",
            Self::Snapshot => "snapshot",
            Self::Bookmark => "bookmark",
            Self::Pool => "pool",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for Type {
    type Err = error::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "filesystem" => Ok(Self::Filesystem),
            "volume" => Ok(Self::Volume),
            "snapshot" => Ok(Self::Snapshot),
            "bookmark" => Ok(Self::Bookmark),
            "pool" => Ok(Self::Pool),
            "unknown" => Ok(Self::Unknown),
            other => Err(error::InvalidProperty::invalid_value(other)),
        }
    }
}

// TODO: create macto for all u and i
impl From<u64> for Type {
    fn from(value: u64) -> Self {
        match value {
            1 => Self::Filesystem,
            2 => Self::Snapshot,
            4 => Self::Volume,
            8 => Self::Pool,
            16 => Self::Bookmark,
            _ => Self::Unknown,
        }
    }
}
