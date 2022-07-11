use std::fmt;
use std::str;

use serde::{Deserialize, Serialize};

use crate::error;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimeStamp(time::OffsetDateTime);

impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl str::FromStr for TimeStamp {
    type Err = error::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unix = s
            .parse::<i64>()
            .map_err(|_| error::InvalidProperty::invalid_value(s))?;
        let timestamp = time::OffsetDateTime::from_unix_timestamp(unix)
            .map_err(error::InvalidProperty::invalid_value)?;
        Ok(Self(timestamp))
    }
}
