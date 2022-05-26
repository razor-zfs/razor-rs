use std::fmt;
use std::str;

use serde::{Deserialize, Serialize};

use crate::error;

// checksum=on|off|fletcher2|fletcher4|sha256|noparity|sha512|skein|edonr
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

impl str::FromStr for CheckSum {
    type Err = error::InvalidProperty;

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
            other => Err(error::InvalidProperty::invalid_value(other)),
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

// TODO: create macro for all u and i
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

impl From<CheckSum> for u64 {
    fn from(value: CheckSum) -> Self {
        match value {
            CheckSum::On => 1,
            CheckSum::Off => 2,
            CheckSum::Fletcher2 => 6,
            CheckSum::Fletcher4 => 7,
            CheckSum::Sha256 => 8,
            CheckSum::NoParity => 10,
            CheckSum::Sha512 => 11,
            CheckSum::Skein => 12,
            CheckSum::Edonr => 13,
        }
    }
}
