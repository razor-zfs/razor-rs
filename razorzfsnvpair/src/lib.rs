use razorzfsnvpair_sys as sys;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Datatype unknown")]
pub struct InvalidNvPairDataTypeUnknown;

#[derive(Debug, Error)]
#[error("Datatype dont care")]
pub struct InvalidNvPairDataTypeDontCare;

pub enum NvPairData {
    DataTypeDontcare(InvalidNvPairDataTypeUnknown),
    DataTypeUnknown(InvalidNvPairDataTypeDontCare),
    DataTypeBoolean(bool),
    DataTypeByte(u8),
    DataTypeInt16(i16),
    DataTypeUint16(u16),
    DataTypeInt32(i32),
    DataTypeUint32(u32),
    DataTypeInt64(i64),
    DataTypeUint64(u64),
    DataTypeString(String),
    DataTypeByteArray(Vec<u8>),
    DataTypeInt16Array(Vec<i16>),
    DataTypeUint16Array(Vec<u16>),
    DataTypeInt32Array(Vec<i32>),
    DataTypeUint32Array(Vec<u32>),
    DataTypeInt64Array(Vec<i64>),
    DataTypeUint64Array(Vec<u64>),
    DataTypeStringArray(Vec<String>),
    DataTypeHrtime(u64),
    DataTypeNvlist(NvList),
    DataTypeNvlistArray(Vec<NvList>),
    DataTypeBooleanValue(bool),
    DataTypeInt8(i8),
    DataTypeUint8(u8),
    DataTypeBooleanArray(Vec<bool>),
    DataTypeInt8Array(Vec<i8>),
    DataTypeUint8Array(Vec<u8>),
    DataTypeDouble(f64),
}
pub struct NvPair {
    key: String,
    value: NvPairData,
}

pub struct NvList {}
