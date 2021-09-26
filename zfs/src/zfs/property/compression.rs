use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Compression {
    On,
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
    ZstdFast,
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
            Self::ZstdFast => "zstd-fast",
        }
    }
}

impl fmt::Display for Compression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for Compression {
    type Err = super::InvalidProperty;

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
            "zstd-fast" => Ok(Self::ZstdFast),
            other => Err(super::InvalidProperty::invalid_value(other)),
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

// TODO: create macto for all u and i
impl From<u64> for Compression {
    fn from(value: u64) -> Self {
        match value {
            1 => Self::On,
            2 => Self::Off,
            3 => Self::Lzjb,
            10 => Self::Gzip6,
            5 => Self::Gzip1,
            6 => Self::Gzip2,
            7 => Self::Gzip3,
            8 => Self::Gzip4,
            9 => Self::Gzip5,
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

// TODO: on new version of ZFS check numerics for Gzip and ZstdFast
impl From<Compression> for u64 {
    fn from(value: Compression) -> Self {
        match value {
            Compression::On => 1,
            Compression::Off => 2,
            Compression::Lzjb => 3,
            Compression::Gzip6 => 4,
            Compression::Gzip1 => 5,
            Compression::Gzip2 => 6,
            Compression::Gzip3 => 7,
            Compression::Gzip4 => 8,
            Compression::Gzip5 => 9,
            Compression::Gzip7 => 11,
            Compression::Gzip8 => 12,
            Compression::Gzip9 => 13,
            Compression::Zle => 14,
            Compression::Lz4 => 15,
            Compression::Zstd => 16,
            Compression::Gzip => todo!(),
            Compression::ZstdFast => todo!(),
        }
    }
}
