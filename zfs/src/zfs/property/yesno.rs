#![allow(clippy::use_self)]

use std::fmt;
use std::str;

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum YesNo {
    #[default]
    No,
    Yes,
}

impl YesNo {
    pub fn as_str(&self) -> &str {
        match self {
            Self::No => "no",
            Self::Yes => "yes",
        }
    }
}

impl fmt::Display for YesNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for YesNo {
    type Err = error::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "no" => Ok(Self::No),
            "yes" => Ok(Self::Yes),
            other => Err(error::InvalidProperty::invalid_value(other)),
        }
    }
}

impl From<YesNo> for bool {
    fn from(onoff: YesNo) -> Self {
        match onoff {
            YesNo::No => false,
            YesNo::Yes => true,
        }
    }
}

impl From<bool> for YesNo {
    fn from(v: bool) -> Self {
        match v {
            false => Self::No,
            true => Self::Yes,
        }
    }
}

macro_rules! numeric {
    ($numeric:ty) => {
        impl From<$numeric> for YesNo {
            fn from(value: $numeric) -> Self {
                match value {
                    0 => Self::No,
                    _ => Self::Yes,
                }
            }
        }

        impl From<YesNo> for $numeric {
            fn from(value: YesNo) -> Self {
                match value {
                    YesNo::No => 0,
                    YesNo::Yes => 1,
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
