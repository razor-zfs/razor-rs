use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub enum Bootfs {
    BootableDataset(String),
    Empty,
}

impl fmt::Display for Bootfs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BootableDataset(dataset) => dataset.fmt(f),
            Self::Empty => "-".fmt(f),
        }
    }
}

impl FromStr for Bootfs {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Empty),
            dataset => Ok(Self::BootableDataset(dataset.to_string())),
        }
    }
}
