#![allow(clippy::use_self)]

use std::fmt;
use std::str;

// use libzfs::zfs_canmount_type_t::*;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MountPoint {
    #[default]
    None,
    Legacy,
    Path(String),
}

impl MountPoint {
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Legacy => "legacy",
            Self::Path(path) => path,
        }
    }
}

impl AsRef<str> for MountPoint {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for MountPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for MountPoint {
    type Err = error::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "legacy" => Ok(Self::Legacy),
            path if path.starts_with('/') => Ok(Self::Path(path.to_string())),
            other => Err(error::InvalidProperty::invalid_value(other)),
        }
    }
}
