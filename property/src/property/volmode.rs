use std::fmt;
use std::str;

use serde::{Deserialize, Serialize};

use crate::error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VolMode {
    Default,
    Full,
    Geom,
    Dev,
    None,
    Unknown,
}

impl VolMode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Default => "default",
            Self::Full => "full",
            Self::Geom => "geom",
            Self::Dev => "dev",
            Self::None => "none",
            Self::Unknown => "unknown",
        }
    }
}

impl AsRef<str> for VolMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for VolMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for VolMode {
    type Err = error::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            "full" => Ok(Self::Full),
            "geom" => Ok(Self::Geom),
            "dev" => Ok(Self::Dev),
            "none" => Ok(Self::None),
            other => Err(error::InvalidProperty::invalid_value(other)),
        }
    }
}

// TODO: * create macro for all u and i
//       * in C geom == full
impl From<u64> for VolMode {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Full,
            2 => Self::Dev,
            3 => Self::None,
            _ => Self::Unknown,
        }
    }
}

// TODO: consoder changing it to try into
impl From<VolMode> for u64 {
    fn from(value: VolMode) -> Self {
        match value {
            VolMode::Default => 0,
            VolMode::Full | VolMode::Geom => 1,
            VolMode::Dev => 2,
            VolMode::None => 3,
            VolMode::Unknown => unimplemented!(),
        }
    }
}
