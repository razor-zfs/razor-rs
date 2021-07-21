use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NvPair {
    pub nvpair: *mut sys::nvpair_t,
    pub pair_name: String,
    pub pair_value: ContextType,
}

impl NvPair {
    pub fn default() -> NvPair {
        NvPair {
            nvpair: std::ptr::null_mut(),
            pair_name: "".to_string(),
            pair_value: ContextType::Empty,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContextType {
    U16(u16),
    U32(u32),
    Str(String),
    U8Arr(Vec<u8>),
    U16Arr(Vec<u16>),
    U32Arr(Vec<u32>),
    U64Arr(Vec<u64>),
    I8Arr(Vec<i8>),
    I16Arr(Vec<i16>),
    I32Arr(Vec<i32>),
    I64Arr(Vec<i64>),
    BooleanArr(Vec<bool>),
    StrArr(Vec<String>),
    DoubleArr(Vec<f64>),
    Empty,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum NvPairType {
    Dontcare,
    Unknown,
    Boolean,
    Byte,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    String,
    ByteArray,
    Int16Array,
    Uint16Array,
    Int32Array,
    Uint32Array,
    Int64Array,
    Uint64Array,
    StringArray,
    Hrtime,
    Nvlist,
    NvlistArray,
    BooleanValue,
    Int8,
    Uint8,
    BooleanArray,
    Int8Array,
    Uint8Array,
    Double,
}

impl From<sys::data_type_t> for NvPairType {
    fn from(mtype: sys::data_type_t) -> Self {
        match mtype {
            sys::data_type_t::DATA_TYPE_DONTCARE => Self::Dontcare,
            sys::data_type_t::DATA_TYPE_UNKNOWN => Self::Unknown,
            sys::data_type_t::DATA_TYPE_BOOLEAN => Self::Boolean,
            sys::data_type_t::DATA_TYPE_BYTE => Self::Byte,
            sys::data_type_t::DATA_TYPE_INT16 => Self::Int16,
            sys::data_type_t::DATA_TYPE_UINT16 => Self::Uint16,
            sys::data_type_t::DATA_TYPE_INT32 => Self::Int32,
            sys::data_type_t::DATA_TYPE_UINT32 => Self::Uint32,
            sys::data_type_t::DATA_TYPE_INT64 => Self::Int64,
            sys::data_type_t::DATA_TYPE_UINT64 => Self::Uint64,
            sys::data_type_t::DATA_TYPE_STRING => Self::String,
            sys::data_type_t::DATA_TYPE_BYTE_ARRAY => Self::ByteArray,
            sys::data_type_t::DATA_TYPE_INT16_ARRAY => Self::Int16Array,
            sys::data_type_t::DATA_TYPE_UINT16_ARRAY => Self::Uint16Array,
            sys::data_type_t::DATA_TYPE_INT32_ARRAY => Self::Int32Array,
            sys::data_type_t::DATA_TYPE_UINT32_ARRAY => Self::Uint32Array,
            sys::data_type_t::DATA_TYPE_INT64_ARRAY => Self::Int64Array,
            sys::data_type_t::DATA_TYPE_UINT64_ARRAY => Self::Uint64Array,
            sys::data_type_t::DATA_TYPE_STRING_ARRAY => Self::StringArray,
            sys::data_type_t::DATA_TYPE_HRTIME => Self::Hrtime,
            sys::data_type_t::DATA_TYPE_NVLIST => Self::Nvlist,
            sys::data_type_t::DATA_TYPE_NVLIST_ARRAY => Self::NvlistArray,
            sys::data_type_t::DATA_TYPE_BOOLEAN_VALUE => Self::BooleanValue,
            sys::data_type_t::DATA_TYPE_INT8 => Self::Int8,
            sys::data_type_t::DATA_TYPE_UINT8 => Self::Uint8,
            sys::data_type_t::DATA_TYPE_BOOLEAN_ARRAY => Self::BooleanArray,
            sys::data_type_t::DATA_TYPE_INT8_ARRAY => Self::Int8Array,
            sys::data_type_t::DATA_TYPE_UINT8_ARRAY => Self::Uint8Array,
            sys::data_type_t::DATA_TYPE_DOUBLE => Self::Double,
            _ => Self::Unknown,
        }
    }
}
