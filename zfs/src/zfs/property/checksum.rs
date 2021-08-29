use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

// checksum=on|off|fletcher2|fletcher4|sha256|noparity|sha512|skein|edonr
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
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

impl CheckSum {
    pub fn as_str(&self) -> &str {
        match self {
            Self::On => "on",
            Self::Off => "off",
            Self::Fletcher2 => "fletcher2",
            Self::Fletcher4 => "fletcher4",
            Self::Sha256 => "sha256",
            Self::NoParity => "noparity",
            Self::Sha512 => "sha512",
            Self::Skein => "skein",
            Self::Edonr => "edonr",
        }
    }
}

impl fmt::Display for CheckSum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for CheckSum {
    type Err = super::DatasetError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
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
            other => Err(super::InvalidProperty::invalid_value(other).into()),
        }
    }
}

impl From<bool> for CheckSum {
    fn from(v: bool) -> Self {
        if v {
            Self::On
        } else {
            Self::Off
        }
    }
}

// TODO: create macto for all u and i
impl From<u64> for CheckSum {
    fn from(value: u64) -> Self {
        match value {
            1 => Self::On,
            2 => Self::Off,
            6 => Self::Fletcher2,
            7 => Self::Fletcher4,
            8 => Self::Sha256,
            10 => Self::NoParity,
            11 => Self::Sha512,
            12 => Self::Skein,
            13 => Self::Edonr,
            _ => Self::Off,
        }
    }
}
