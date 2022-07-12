use std::fmt;
use std::str;

use serde::{Deserialize, Serialize};

use crate::error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnOff {
    Off,
    On,
}

impl OnOff {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Off => "off",
            Self::On => "on",
        }
    }
}

impl AsRef<str> for OnOff {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for OnOff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for OnOff {
    type Err = error::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(Self::Off),
            "on" => Ok(Self::On),
            other => Err(error::InvalidProperty::invalid_value(other)),
        }
    }
}

impl From<bool> for OnOff {
    fn from(v: bool) -> Self {
        if v {
            Self::On
        } else {
            Self::Off
        }
    }
}

impl From<OnOff> for bool {
    fn from(onoff: OnOff) -> Self {
        match onoff {
            OnOff::Off => false,
            OnOff::On => true,
        }
    }
}

macro_rules! numeric {
    ($numeric:ty) => {
        impl From<$numeric> for OnOff {
            fn from(value: $numeric) -> Self {
                match value {
                    0 => Self::Off,
                    _ => Self::On,
                }
            }
        }

        impl From<OnOff> for $numeric {
            fn from(value: OnOff) -> Self {
                match value {
                    OnOff::Off => 0,
                    OnOff::On => 1,
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
