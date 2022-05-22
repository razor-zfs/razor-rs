use std::fmt;
use std::str;

use serde::{Deserialize, Serialize};

use crate::error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnOffNoAuto {
    On,
    Off,
    NoAuto,
}

impl OnOffNoAuto {
    pub fn as_str(&self) -> &str {
        match self {
            Self::On => "on",
            Self::Off => "off",
            Self::NoAuto => "noauto",
        }
    }
}

impl fmt::Display for OnOffNoAuto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for OnOffNoAuto {
    type Err = error::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            "noauto" => Ok(Self::NoAuto),
            other => Err(error::InvalidProperty::invalid_value(other)),
        }
    }
}

// TODO: write macro for all u and i
impl From<u64> for OnOffNoAuto {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            _ => Self::NoAuto,
        }
    }
}

impl From<OnOffNoAuto> for u64 {
    fn from(value: OnOffNoAuto) -> Self {
        match value {
            OnOffNoAuto::Off => 0,
            OnOffNoAuto::On => 1,
            OnOffNoAuto::NoAuto => 2,
        }
    }
}

impl From<bool> for OnOffNoAuto {
    fn from(v: bool) -> Self {
        if v {
            Self::On
        } else {
            Self::Off
        }
    }
}
