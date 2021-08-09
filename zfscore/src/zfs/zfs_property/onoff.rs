use std::str::FromStr;
use std::{convert::TryFrom, fmt};

use serde::{Deserialize, Serialize};

use crate::zfs::DatasetError;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum OnOff {
    Off,
    On,
}

impl OnOff {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Off => "off",
            Self::On => "on",
        }
    }
}

impl fmt::Display for OnOff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for OnOff {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(Self::Off),
            "on" => Ok(Self::On),
            other => Err(super::InvalidProperty::invalid_value(other)),
        }
    }
}

impl From<bool> for OnOff {
    fn from(v: bool) -> Self {
        if v {
            OnOff::On
        } else {
            OnOff::Off
        }
    }
}

// TODO: write macro for all u and i
impl TryFrom<u64> for OnOff {
    type Error = DatasetError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OnOff::Off),
            1 => Ok(OnOff::On),
            _ => Err(DatasetError::InvalidArgument),
        }
    }
}

impl From<OnOff> for bool {
    fn from(onoff: OnOff) -> Self {
        match onoff {
            OnOff::Off => false,
            OnOff::On => true,
        }
    }
}
