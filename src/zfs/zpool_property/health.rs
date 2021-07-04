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
            Health::Online => "online",
            Health::Degraded => "degraded",
            Health::Faulted => "faulted",
            Health::Offline => "offline",
            Health::Removed => "removed",
            Health::Unavail => "unavail",
        };

        text.fmt(f)
    }
}

impl FromStr for Health {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "online" => Ok(Self::Online),
            "degraded" => Ok(Self::Degraded),
            "faulted" => Ok(Self::Faulted),
            "offline" => Ok(Self::Offline),
            "removed" => Ok(Self::Removed),
            "unavail" => Ok(Self::Unavail),
            _ => Err(super::InvalidProperty::InvalidValue),
        }
    }
}
