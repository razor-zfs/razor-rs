use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Health {
    Online,
    Degraded,
    Faulted,
    Offline,
    Removed,
    Unavail,
}

impl Health {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Online => "online",
            Self::Degraded => "degraded",
            Self::Faulted => "faulted",
            Self::Offline => "offline",
            Self::Removed => "removed",
            Self::Unavail => "unavail",
        }
    }
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for Health {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ONLINE" => Ok(Self::Online),
            "DEGRADED" => Ok(Self::Degraded),
            "FAULTED" => Ok(Self::Faulted),
            "OFFLINE" => Ok(Self::Offline),
            "REMOVED" => Ok(Self::Removed),
            "UNAVAIL" => Ok(Self::Unavail),
            other => Err(super::InvalidProperty::invalid_value(other)),
        }
    }
}
