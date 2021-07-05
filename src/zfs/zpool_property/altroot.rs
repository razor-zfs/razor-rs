use std::fmt;
use std::ops;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Altroot(Option<String>);

impl fmt::Display for Altroot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(x) => x.fmt(f),
            None => "-".fmt(f),
        }
    }
}

impl FromStr for Altroot {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self(None)),
            path => Ok(Self(Some(path.to_string()))),
        }
    }
}

impl ops::Deref for Altroot {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0.as_ref().unwrap()
    }
}
