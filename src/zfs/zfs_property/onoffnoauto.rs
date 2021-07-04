use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum OnOffNoAuto {
    On,
    Off,
    NoAuto,
}

impl fmt::Display for OnOffNoAuto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::On => "on",
            Self::Off => "off",
            Self::NoAuto => "noauto",
        };
        text.fmt(f)
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
