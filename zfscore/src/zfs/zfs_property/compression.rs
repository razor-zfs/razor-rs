use std::str::FromStr;
use std::{convert::TryFrom, fmt};

use serde::{Deserialize, Serialize};

use crate::zfs::DatasetError;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
            Compression::On
        } else {
            Compression::Off
        }
    }
}

// TODO: create macto for all u and i
impl TryFrom<u64> for Compression {
    type Error = DatasetError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Compression::On),
            2 => Ok(Compression::Off),
            3 => Ok(Compression::Lzjb),
            10 => Ok(Compression::Gzip6),
            5 => Ok(Compression::Gzip1),
            6 => Ok(Compression::Gzip2),
            7 => Ok(Compression::Gzip3),
            8 => Ok(Compression::Gzip4),
            9 => Ok(Compression::Gzip5),
            11 => Ok(Compression::Gzip7),
            12 => Ok(Compression::Gzip8),
            13 => Ok(Compression::Gzip9),
            14 => Ok(Compression::Zle),
            15 => Ok(Compression::Lz4),
            16 => Ok(Compression::Zstd),
            _ => Err(DatasetError::InvalidArgument),
        }
    }
}
