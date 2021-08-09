use std::fmt;
use std::ops;
use std::str::FromStr;

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TimeStamp(DateTime<Utc>);

impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for TimeStamp {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>()
            .map(|seconds| Utc.timestamp(seconds, 0))
            .map(TimeStamp)
            .map_err(|_| super::InvalidProperty::invalid_value(s))
    }
}

impl ops::Deref for TimeStamp {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
