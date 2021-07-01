use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Health {
    Online,
    Degraded,
    Faulted,
    Offline,
    Removed,
    Unavail,
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Health::Online => "Online",
            Health::Degraded => "Degraded",
            Health::Faulted => "Faulted",
            Health::Offline => "Offline",
            Health::Removed => "Removed",
            Health::Unavail => "Unavail",
        };

        text.fmt(f)
    }
}

impl FromStr for Health {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Online" => Ok(Self::Online),
            "Degraded" => Ok(Self::Degraded),
            "Faulted" => Ok(Self::Faulted),
            "Offline" => Ok(Self::Offline),
            "Removed" => Ok(Self::Removed),
            "Unavail" => Ok(Self::Unavail),
            _ => Err(super::InvalidProperty::InvalidValue),
        }
    }
}
