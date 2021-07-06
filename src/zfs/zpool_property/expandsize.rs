use std::fmt;
use std::ops;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Expandsize(Option<u64>);

impl fmt::Display for Expandsize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(x) => x.fmt(f),
            None => "-".fmt(f),
        }
    }
}

impl FromStr for Expandsize {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self(None)),
            other => other
                .parse::<u64>()
                .map_err(|_| super::InvalidProperty::invalid_value(s))
                .map(|size| Self(Some(size))),
        }
    }
}

impl ops::Deref for Expandsize {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}
