#![allow(clippy::use_self)]

use std::fmt;
use std::str;

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Compression {
    On,
    #[default]
    Off,
    Lzjb,
    Gzip,
    Gzip1,
    Gzip2,
    Gzip3,
    Gzip4,
    Gzip5,
    Gzip6,
    Gzip7,
    Gzip8,
    Gzip9,
    Zle,
    Lz4,
    Zstd,
}

impl Compression {
    pub fn as_str(&self) -> &str {
        match self {
            Self::On => "on",
            Self::Off => "off",
            Self::Lzjb => "lzjb",
            Self::Gzip => "gzip",
            Self::Gzip1 => "gzip-1",
            Self::Gzip2 => "gzip-2",
            Self::Gzip3 => "gzip-3",
            Self::Gzip4 => "gzip-4",
            Self::Gzip5 => "gzip-5",
            Self::Gzip6 => "gzip-6",
            Self::Gzip7 => "gzip-7",
            Self::Gzip8 => "gzip-8",
            Self::Gzip9 => "gzip-9",
            Self::Zle => "zle",
            Self::Lz4 => "lz4",
            Self::Zstd => "zstd",
        }
    }
}

impl fmt::Display for Compression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl AsRef<str> for Compression {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl str::FromStr for Compression {
    type Err = error::InvalidProperty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            "lzjb" => Ok(Self::Lzjb),
            "gzip" => Ok(Self::Gzip),
            "gzip-1" => Ok(Self::Gzip1),
            "gzip-2" => Ok(Self::Gzip2),
            "gzip-3" => Ok(Self::Gzip3),
            "gzip-4" => Ok(Self::Gzip4),
            "gzip-5" => Ok(Self::Gzip5),
            "gzip-6" => Ok(Self::Gzip6),
            "gzip-7" => Ok(Self::Gzip7),
            "gzip-8" => Ok(Self::Gzip8),
            "gzip-9" => Ok(Self::Gzip9),
            "zle" => Ok(Self::Zle),
            "lz4" => Ok(Self::Lz4),
            "zstd" => Ok(Self::Zstd),
            other => Err(error::InvalidProperty::invalid_value(other)),
        }
    }
}

impl From<bool> for Compression {
    fn from(v: bool) -> Self {
        if v {
            Self::On
        } else {
            Self::Off
        }
    }
}

macro_rules! numeric {
    ($numeric:ty) => {
        impl From<$numeric> for Compression {
            fn from(value: $numeric) -> Self {
                match value {
                    1 => Self::On,
                    2 => Self::Off,
                    3 => Self::Lzjb,
                    5 => Self::Gzip1,
                    6 => Self::Gzip2,
                    7 => Self::Gzip3,
                    8 => Self::Gzip4,
                    9 => Self::Gzip5,
                    10 => Self::Gzip6,
                    11 => Self::Gzip7,
                    12 => Self::Gzip8,
                    13 => Self::Gzip9,
                    14 => Self::Zle,
                    15 => Self::Lz4,
                    16 => Self::Zstd,
                    _ => Self::Off,
                }
            }
        }

        impl From<Compression> for $numeric {
            fn from(value: Compression) -> Self {
                match value {
                    Compression::On => 1,
                    Compression::Off => 2,
                    Compression::Lzjb => 3,
                    Compression::Gzip => 10,
                    Compression::Gzip1 => 5,
                    Compression::Gzip2 => 6,
                    Compression::Gzip3 => 7,
                    Compression::Gzip4 => 8,
                    Compression::Gzip5 => 9,
                    Compression::Gzip6 => 10,
                    Compression::Gzip7 => 11,
                    Compression::Gzip8 => 12,
                    Compression::Gzip9 => 13,
                    Compression::Zle => 14,
                    Compression::Lz4 => 15,
                    Compression::Zstd => 16,
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
