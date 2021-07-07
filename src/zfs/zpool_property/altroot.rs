use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

//#[derive(Debug, Serialize, Deserialize)]
//pub struct Altroot(Option<String>);

#[derive(Debug, Serialize, Deserialize)]
pub enum Altroot {
    Directory(String),
    Empty,
}

impl fmt::Display for Altroot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Directory(path) => path.fmt(f),
            Self::Empty => "-".fmt(f),
        }
    }
}

impl FromStr for Altroot {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Empty),
            path => Ok(Self::Directory(path.to_string())),
        }
    }
}
