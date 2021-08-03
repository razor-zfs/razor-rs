use std::fmt;
use std::ops;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Allocated(u64);

impl fmt::Display for Allocated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Allocated {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>()
            .map_err(|_| super::InvalidProperty::invalid_value(s))
            .map(Allocated)
    }
}

impl ops::Deref for Allocated {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
