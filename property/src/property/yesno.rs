use std::fmt;
use std::str;

use serde::{Deserialize, Serialize};

use crate::error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum YesNo {
    Yes,
    No,
}

impl YesNo {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Yes => "yes",
            Self::No => "no",
        }
    }
}

impl fmt::Display for YesNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for YesNo {
    type Err = error::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yes" => Ok(Self::Yes),
            "no" => Ok(Self::No),
            other => Err(error::InvalidProperty::invalid_value(other)),
        }
    }
}

impl From<YesNo> for bool {
    fn from(onoff: YesNo) -> Self {
        match onoff {
            YesNo::Yes => true,
            YesNo::No => false,
        }
    }
}

// TODO: write macro for all u and i
impl From<u64> for YesNo {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::No,
            _ => Self::Yes,
        }
    }
}

impl From<bool> for YesNo {
    fn from(v: bool) -> Self {
        match v {
            true => Self::Yes,
            false => Self::No,
        }
    }
}
