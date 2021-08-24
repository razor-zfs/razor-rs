use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
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

impl FromStr for OnOffNoAuto {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            "noauto" => Ok(Self::NoAuto),
            other => Err(super::InvalidProperty::invalid_value(other)),
        }
    }
}

impl From<bool> for OnOffNoAuto {
    fn from(v: bool) -> Self {
        if v {
            OnOffNoAuto::On
        } else {
            OnOffNoAuto::Off
        }
    }
}
