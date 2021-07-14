use std::ffi::CString;

use super::*;

mod impls;
mod impls_trait;

pub struct NvPair {
    pub name: CString,
    pub value: Value,
}

pub enum Value {
    BOOL(sys::boolean_t),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    String(CString),
    EMPTY,
}
