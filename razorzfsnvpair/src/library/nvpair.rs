use super::*;
use std::ffi::CString;

#[derive(Clone, Debug, PartialEq)]
pub struct NvPair {
    pub pair_name: CString,
    pub pair_value: ContextType,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ContextType {
    U8Arr(Vec<u8>),
    U16Arr(Vec<u16>),
    U32Arr(Vec<u32>),
    U64Arr(Vec<u64>),
    I8Arr(Vec<i8>),
    I16Arr(Vec<i16>),
    I32Arr(Vec<i32>),
    I64Arr(Vec<i64>),
    BooleanArr(Vec<sys::boolean_t>),
    StrArr(Vec<CString>),
    DoubleArr(Vec<f64>),
    Empty,
}
