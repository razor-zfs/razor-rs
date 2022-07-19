#![allow(clippy::use_self)]

use std::fmt;
use std::str;

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum VolMode {
    #[default]
    Default,
    Full,
    Geom,
    Dev,
    None,
}

impl VolMode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Default => "default",
            Self::Full => "full",
            Self::Geom => "geom",
            Self::Dev => "dev",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for VolMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for VolMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for VolMode {
    type Err = error::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            "full" => Ok(Self::Full),
            "geom" => Ok(Self::Geom),
            "dev" => Ok(Self::Dev),
            "none" => Ok(Self::None),
            other => Err(error::InvalidProperty::invalid_value(other)),
        }
    }
}

macro_rules! numeric {
    ($numeric:ty) => {
        impl From<$numeric> for VolMode {
            fn from(value: $numeric) -> Self {
                match value {
                    0 => Self::Default,
                    1 => Self::Full,
                    2 => Self::Dev,
                    3 => Self::None,
                    _ => Self::Default,
                }
            }
        }

        impl From<VolMode> for $numeric {
            fn from(value: VolMode) -> Self {
                match value {
                    VolMode::Default => 0,
                    VolMode::Full | VolMode::Geom => 1,
                    VolMode::Dev => 2,
                    VolMode::None => 3,
                }
            }
        }
    };
}

numeric!(i8);
numeric!(i16);
numeric!(i32);
numeric!(i64);
numeric!(u8);
numeric!(u16);
numeric!(u32);
numeric!(u64);
