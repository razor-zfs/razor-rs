use std::fmt;
use std::ops;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ashift(u64);

fn check_range(shift: u64) -> Result<u64, super::InvalidProperty> {
    if (9..=16).contains(&shift) {
        Ok(shift)
    } else {
        Err(super::InvalidProperty::invalid_value(shift))
    }
}

impl fmt::Display for Ashift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Ashift {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>()
            .map_err(|_| super::InvalidProperty::invalid_value(s))
            .and_then(check_range)
            .map(Ashift)
    }
}

impl ops::Deref for Ashift {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
