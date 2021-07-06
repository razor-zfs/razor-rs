use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Failmode {
    Wait,
    Continue,
    Panic,
}

impl Failmode {
    pub fn as_str(&self) -> &str {
        match self {
            Failmode::Wait => "wait",
            Failmode::Continue => "continue",
            Failmode::Panic => "panic",
        }
    }
}

impl fmt::Display for Failmode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for Failmode {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "wait" => Ok(Self::Wait),
            "continue" => Ok(Self::Continue),
            "panic" => Ok(Self::Panic),
            other => Err(super::InvalidProperty::invalid_value(other)),
        }
    }
}
