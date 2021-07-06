use std::fmt;
use std::ops;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bootfs(Option<String>);

impl fmt::Display for Bootfs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(x) => x.fmt(f),
            None => "-".fmt(f),
        }
    }
}

impl FromStr for Bootfs {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self(None)),
            dataset => Ok(Self(Some(dataset.to_string()))),
        }
    }
}

impl ops::Deref for Bootfs {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}
