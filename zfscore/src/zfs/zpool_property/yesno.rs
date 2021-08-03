use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum YesNo {
    Yes,
    No,
}

impl YesNo {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Yes => "yes",
            Self::No => "no",
        }
    }
}

impl fmt::Display for YesNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for YesNo {
    type Err = super::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yes" => Ok(Self::Yes),
            "no" => Ok(Self::No),
            other => Err(super::InvalidProperty::invalid_value(other)),
        }
    }
}

impl From<YesNo> for bool {
    fn from(onoff: YesNo) -> Self {
        match onoff {
            YesNo::Yes => true,
            YesNo::No => false,
        }
    }
}
