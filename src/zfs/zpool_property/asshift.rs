use std::fmt;
use std::ops;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Asshift(u64);

fn check_range(shift: u64) -> Result<u64, super::InvalidProperty> {
    if shift >= 9 && shift <= 16 {
        Ok(shift)
    } else {
        Err(super::InvalidProperty::InvalidValue)
    }
}

impl fmt::Display for Asshift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Asshift {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>()
            .map_err(|_| super::InvalidProperty::InvalidValue)
            .and_then(check_range)
            .map(Asshift)
    }
}

impl ops::Deref for Asshift {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
