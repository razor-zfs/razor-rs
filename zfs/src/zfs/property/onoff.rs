use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
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
            Self::On
        } else {
            Self::Off
        }
    }
}

// TODO: write macro for all u and i
impl From<u64> for OnOff {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Off,
            _ => Self::On,
        }
    }
}

impl From<OnOff> for u64 {
    fn from(value: OnOff) -> Self {
        match value {
            OnOff::Off => 0,
            OnOff::On => 1,
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
