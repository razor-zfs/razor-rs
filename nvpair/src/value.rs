use razor_libnvpair as libnvpair;

use super::{NvList, NvPair};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Char(char),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    String(String),
    Double(f64),
    NvList(NvList),
    BooleanArray(Vec<bool>),
    U8Array(Vec<u8>),
    U16Array(Vec<u16>),
    U32Array(Vec<u32>),
    U64Array(Vec<u64>),
    I8Array(Vec<i8>),
    I16Array(Vec<i16>),
    I32Array(Vec<i32>),
    I64Array(Vec<i64>),
    StringArray(Vec<String>),
    DoubleArray(Vec<f64>),
    NvListArray(Vec<NvList>),
}

pub fn to_value(nvpair: &NvPair) -> Value {
    use libnvpair::data_type_t::*;
    match nvpair.r#type() {
        DATA_TYPE_DONTCARE => todo!(),
        DATA_TYPE_UNKNOWN => todo!(),

        // DATA_TYPE_BOOLEAN => Value::Boolean(nvpair.boolean()),
        DATA_TYPE_BYTE => todo!(),

        DATA_TYPE_INT16 => Value::I16(nvpair.int16()),
        DATA_TYPE_UINT16 => Value::U16(nvpair.uint16()),
        DATA_TYPE_INT32 => Value::I32(nvpair.int32()),
        DATA_TYPE_UINT32 => Value::U32(nvpair.uint32()),
        DATA_TYPE_INT64 => Value::I64(nvpair.int64()),
        DATA_TYPE_UINT64 => Value::U64(nvpair.uint64()),
        DATA_TYPE_STRING => Value::String(nvpair.string().into_owned()),

        DATA_TYPE_BYTE_ARRAY => todo!(),

        DATA_TYPE_INT16_ARRAY => Value::I16Array(nvpair.int16_array().to_vec()),
        DATA_TYPE_UINT16_ARRAY => Value::U16Array(nvpair.uint16_array().to_vec()),
        DATA_TYPE_INT32_ARRAY => Value::I32Array(nvpair.int32_array().to_vec()),
        DATA_TYPE_UINT32_ARRAY => Value::U32Array(nvpair.uint32_array().to_vec()),
        DATA_TYPE_INT64_ARRAY => Value::I64Array(nvpair.int64_array().to_vec()),
        DATA_TYPE_UINT64_ARRAY => Value::U64Array(nvpair.uint64_array().to_vec()),
        DATA_TYPE_STRING_ARRAY => Value::StringArray(
            nvpair
                .string_array()
                .into_iter()
                .map(|text| text.into_owned())
                .collect(),
        ),

        DATA_TYPE_HRTIME => todo!(),
        DATA_TYPE_NVLIST => todo!(),
        DATA_TYPE_NVLIST_ARRAY => todo!(),

        DATA_TYPE_BOOLEAN_VALUE => Value::Boolean(nvpair.boolean() == libnvpair::boolean_t::B_TRUE),
        DATA_TYPE_INT8 => Value::I8(nvpair.int8()),
        DATA_TYPE_UINT8 => Value::U8(nvpair.uint8()),
        DATA_TYPE_BOOLEAN_ARRAY => Value::BooleanArray(
            nvpair
                .boolean_array()
                .iter()
                .map(|item| *item == libnvpair::boolean_t::B_TRUE)
                .collect(),
        ),
        DATA_TYPE_INT8_ARRAY => Value::I8Array(nvpair.int8_array().to_vec()),
        DATA_TYPE_UINT8_ARRAY => Value::U8Array(nvpair.uint8_array().to_vec()),

        DATA_TYPE_DOUBLE => todo!(),
        _ => unreachable!(),
    }
}
