use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

impl From<OnOff> for bool {
    fn from(onoff: OnOff) -> Self {
        match onoff {
            OnOff::Off => false,
            OnOff::On => true,
        }
    }
}
