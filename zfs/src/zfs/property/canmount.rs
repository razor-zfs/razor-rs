#![allow(clippy::use_self)]

use std::fmt;
use std::str;

// use libzfs::zfs_canmount_type_t::*;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanMount {
    Off,
    #[default]
    On,
    NoAuto,
}

impl CanMount {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Off => "off",
            Self::On => "on",
            Self::NoAuto => "noauto",
        }
    }
}

impl AsRef<str> for CanMount {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CanMount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for CanMount {
    type Err = error::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(Self::Off),
            "on" => Ok(Self::On),
            "noauto" => Ok(Self::NoAuto),
            other => Err(error::InvalidProperty::invalid_value(other)),
        }
    }
}

// TODO: write macro for all u and i
impl From<u64> for CanMount {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            _ => Self::NoAuto,
        }
    }
}

impl From<CanMount> for u64 {
    fn from(value: CanMount) -> Self {
        match value {
            CanMount::Off => 0,
            CanMount::On => 1,
            CanMount::NoAuto => 2,
        }
    }
}

impl From<bool> for CanMount {
    fn from(v: bool) -> Self {
        if v {
            Self::On
        } else {
            Self::Off
        }
    }
}
