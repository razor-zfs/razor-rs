use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Version {
    Version(u64),
    Empty,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Version(version) => version.fmt(f),
            Self::Empty => "-".fmt(f),
        }
    }
}

impl FromStr for Version {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Empty),
            other => other
                .parse::<u64>()
                .map_err(|_| super::InvalidProperty::invalid_value(s))
                .map(|version| Self::Version(version)),
        }
    }
}
