use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum OnOff {
    Off,
    On,
}

impl fmt::Display for OnOff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Off => "off",
            Self::On => "on",
        };
        text.fmt(f)
    }
}

impl FromStr for OnOff {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(Self::Off),
            "on" => Ok(Self::On),
            _ => Err(super::InvalidProperty::InvalidValue),
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
