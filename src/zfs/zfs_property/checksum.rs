use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

// checksum=on|off|fletcher2|fletcher4|sha256|noparity|sha512|skein|edonr
#[derive(Debug, Serialize, Deserialize)]
pub enum CheckSum {
    On,
    Off,
    Fletcher2,
    Fletcher4,
    Sha256,
    NoParity,
    Sha512,
    Skein,
    Edonr,
}

impl fmt::Display for CheckSum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::On => "on",
            Self::Off => "off",
            Self::Fletcher2 => "fletcher2",
            Self::Fletcher4 => "fletcher4",
            Self::Sha256 => "sha256",
            Self::NoParity => "noparity",
            Self::Sha512 => "sha512",
            Self::Skein => "skein",
            Self::Edonr => "edonr",
        };
        text.fmt(f)
    }
}

impl FromStr for CheckSum {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            "fletcher2" => Ok(Self::Fletcher2),
            "fletcher4" => Ok(Self::Fletcher4),
            "sha256" => Ok(Self::Sha256),
            "noparity" => Ok(Self::NoParity),
            "sha512" => Ok(Self::Sha512),
            "skein" => Ok(Self::Skein),
            "edonr" => Ok(Self::Edonr),
            _ => Err(super::InvalidProperty::InvalidValue),
        }
    }
}
