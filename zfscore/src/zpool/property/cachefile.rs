use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Cachefile {
    File(String),
    Empty,
}

impl fmt::Display for Cachefile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::File(file) => file.fmt(f),
            Self::Empty => "-".fmt(f),
        }
    }
}

impl FromStr for Cachefile {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Empty),
            file => Ok(Self::File(file.to_string())),
        }
    }
}
