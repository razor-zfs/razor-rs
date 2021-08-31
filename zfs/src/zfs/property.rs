use std::ffi::CString;

use serde::{Deserialize, Serialize};

use super::core;
use crate::error::{DatasetError, InvalidProperty};

pub use checksum::CheckSum as CheckSumAlgo;
pub use compression::Compression as CompressionAlgo;
pub use dataset::Type as DsType;
pub use onoff::OnOff;
pub use onoffnoauto::OnOffNoAuto;
pub use timestamp::TimeStamp;
pub use volmode::VolMode as VolModeId;
pub use yesno::YesNo;

mod checksum;
mod compression;
mod dataset;
mod onoff;
mod onoffnoauto;
mod timestamp;
mod volmode;
mod yesno;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Guid {
    value: u64,
}
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct Name {
    value: CString,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Available {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct CompressRatio {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Type {
    value: DsType,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Used {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct LogicalUsed {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Referenced {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct LogicalReferenced {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct CreateTxg {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Creation {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Volsize {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct VolBlockSize {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Written {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct ObjSetId {
    value: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Atime {
    value: OnOff,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct CanMount {
    value: OnOffNoAuto,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Mounted {
    value: YesNo,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct CheckSum {
    value: CheckSumAlgo,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Volmode {
    value: VolModeId,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Compression {
    value: CompressionAlgo,
}

impl Atime {
    pub fn new(value: OnOff) -> Self {
        Self { value }
    }

    pub fn default(dataset: CString) -> Self {
        Self::new(OnOff::from(core::default_atime(dataset)))
    }

    pub fn value(&self) -> OnOff {
        self.value
    }
}

impl Type {
    pub fn new(value: DsType) -> Self {
        Self { value }
    }

    pub fn default() -> Self {
        Self::new(core::default_type().into())
    }

    pub fn value(&self) -> DsType {
        self.value
    }
}

impl Available {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn default() -> Self {
        Self::new(core::default_available())
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl LogicalUsed {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn default() -> Self {
        Self::new(core::default_logicalused())
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl CanMount {
    pub fn new(value: OnOffNoAuto) -> Self {
        Self { value }
    }

    // TODO: implement the same for volsize, quota, refquota, reservation, refreservation
    //          filesystem_limit, snapshot_limit, filesystem_count, snapshot_count
    pub fn default() -> Self {
        Self::new(core::default_canmount().into())
    }

    pub fn value(&self) -> OnOffNoAuto {
        self.value
    }
}

impl Mounted {
    pub fn new(value: YesNo) -> Self {
        Self { value }
    }

    pub(super) fn default(dataset: CString) -> Self {
        Self::new(core::default_mounted(dataset).into())
    }

    pub fn value(&self) -> YesNo {
        self.value
    }
}

impl CheckSum {
    pub fn new(value: CheckSumAlgo) -> Self {
        Self { value }
    }

    pub fn default() -> Self {
        Self::new(core::default_checksum().into())
    }

    pub fn value(&self) -> CheckSumAlgo {
        self.value
    }
}

impl Volmode {
    pub fn new(value: VolModeId) -> Self {
        Self { value }
    }

    pub fn default() -> Self {
        Self::new(core::default_volmode().into())
    }

    pub fn value(&self) -> VolModeId {
        self.value
    }
}

impl Compression {
    pub fn new(value: CompressionAlgo) -> Self {
        Self { value }
    }

    pub fn default() -> Self {
        Self::new(core::default_compression().into())
    }

    pub fn value(&self) -> CompressionAlgo {
        self.value
    }
}

impl Guid {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Name {
    pub fn new(value: CString) -> Self {
        Self { value }
    }

    // TODO: remove clone
    pub fn value(&self) -> CString {
        self.value.clone()
    }
}

impl Creation {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl CreateTxg {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl CompressRatio {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Used {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Referenced {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl LogicalReferenced {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl ObjSetId {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}
