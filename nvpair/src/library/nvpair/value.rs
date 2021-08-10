use std::ffi;
use std::ptr;

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
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

impl NvPair {
    pub fn value(&self) -> Value {
        use sys::data_type_t::*;
        let nvp = self.raw;
        let data_type = unsafe { sys::nvpair_type(nvp) };
        match data_type {
            DATA_TYPE_DONTCARE => todo!(),
            DATA_TYPE_UNKNOWN => todo!(),
            DATA_TYPE_BOOLEAN => {
                let value = unsafe {
                    let mut value = sys::boolean_t::B_FALSE;
                    sys::nvpair_value_boolean_value(nvp, &mut value);
                    value == sys::boolean_t::B_TRUE
                };
                Value::Boolean(value)
            }

            DATA_TYPE_BYTE => todo!(),

            DATA_TYPE_INT16 => {
                let value = unsafe {
                    let mut value = 0;
                    sys::nvpair_value_int16(nvp, &mut value);
                    value
                };
                Value::I16(value)
            }

            DATA_TYPE_UINT16 => {
                let value = unsafe {
                    let mut value = 0;
                    sys::nvpair_value_uint16(nvp, &mut value);
                    value
                };
                Value::U16(value)
            }

            DATA_TYPE_INT32 => {
                let value = unsafe {
                    let mut value = 0;
                    sys::nvpair_value_int32(nvp, &mut value);
                    value
                };
                Value::I32(value)
            }

            DATA_TYPE_UINT32 => {
                let value = unsafe {
                    let mut value = 0;
                    sys::nvpair_value_uint32(nvp, &mut value);
                    value
                };
                Value::U32(value)
            }

            DATA_TYPE_INT64 => {
                let value = unsafe {
                    let mut value = 0;
                    sys::nvpair_value_int64(nvp, &mut value);
                    value
                };
                Value::I64(value)
            }

            DATA_TYPE_UINT64 => {
                let value = unsafe {
                    let mut value = 0;
                    sys::nvpair_value_uint64(nvp, &mut value);
                    value
                };
                Value::U64(value)
            }

            DATA_TYPE_STRING => {
                let value = unsafe {
                    let mut cstr = ptr::null_mut();
                    sys::nvpair_value_string(nvp, &mut cstr);
                    debug_assert!(!cstr.is_null());
                    let bytes = ffi::CStr::from_ptr(cstr).to_bytes();
                    String::from_utf8_lossy(bytes).to_string()
                };
                Value::String(value)
            }

            DATA_TYPE_BYTE_ARRAY => todo!(),

            DATA_TYPE_INT16_ARRAY => {
                let value = unsafe {
                    let mut len = 0;
                    let mut data = ptr::null_mut();
                    sys::nvpair_value_int16_array(nvp, &mut data, &mut len);
                    debug_assert!(!data.is_null());
                    let len = len as usize;
                    slice::from_raw_parts(data, len).to_vec()
                };
                Value::I16Array(value)
            }

            DATA_TYPE_UINT16_ARRAY => {
                let value = unsafe {
                    let mut len = 0;
                    let mut data = ptr::null_mut();
                    sys::nvpair_value_uint16_array(nvp, &mut data, &mut len);
                    debug_assert!(!data.is_null());
                    let len = len as usize;
                    slice::from_raw_parts(data, len).to_vec()
                };
                Value::U16Array(value)
            }

            DATA_TYPE_INT32_ARRAY => {
                let value = unsafe {
                    let mut len = 0;
                    let mut data = ptr::null_mut();
                    sys::nvpair_value_int32_array(nvp, &mut data, &mut len);
                    debug_assert!(!data.is_null());
                    let len = len as usize;
                    slice::from_raw_parts(data, len).to_vec()
                };
                Value::I32Array(value)
            }

            DATA_TYPE_UINT32_ARRAY => {
                let value = unsafe {
                    let mut len = 0;
                    let mut data = ptr::null_mut();
                    sys::nvpair_value_uint32_array(nvp, &mut data, &mut len);
                    debug_assert!(!data.is_null());
                    let len = len as usize;
                    slice::from_raw_parts(data, len).to_vec()
                };
                Value::U32Array(value)
            }

            DATA_TYPE_INT64_ARRAY => {
                let value = unsafe {
                    let mut len = 0;
                    let mut data = ptr::null_mut();
                    sys::nvpair_value_int64_array(nvp, &mut data, &mut len);
                    debug_assert!(!data.is_null());
                    let len = len as usize;
                    slice::from_raw_parts(data, len).to_vec()
                };
                Value::I64Array(value)
            }

            DATA_TYPE_UINT64_ARRAY => {
                let value = unsafe {
                    let mut len = 0;
                    let mut data = ptr::null_mut();
                    sys::nvpair_value_uint64_array(nvp, &mut data, &mut len);
                    debug_assert!(!data.is_null());
                    let len = len as usize;
                    slice::from_raw_parts(data, len).to_vec()
                };
                Value::U64Array(value)
            }

            DATA_TYPE_STRING_ARRAY => {
                let value = unsafe {
                    let mut len = 0;
                    let mut data = ptr::null_mut();
                    // let mut cstr = ptr::null_mut();
                    sys::nvpair_value_string_array(nvp, &mut data, &mut len);
                    debug_assert!(!data.is_null());
                    let len = len as usize;
                    slice::from_raw_parts(data, len)
                        .iter()
                        .map(|cstr| {
                            let bytes = ffi::CStr::from_ptr(*cstr).to_bytes();
                            String::from_utf8_lossy(bytes).to_string()
                        })
                        .collect()
                };
                Value::StringArray(value)
            }

            DATA_TYPE_HRTIME => todo!(),
            DATA_TYPE_NVLIST => todo!(),
            DATA_TYPE_NVLIST_ARRAY => todo!(),

            DATA_TYPE_BOOLEAN_VALUE => {
                let value = unsafe {
                    let mut value = sys::boolean_t::B_FALSE;
                    sys::nvpair_value_boolean_value(nvp, &mut value);
                    value == sys::boolean_t::B_TRUE
                };
                Value::Boolean(value)
            }

            DATA_TYPE_INT8 => {
                let value = unsafe {
                    let mut value = 0;
                    sys::nvpair_value_int8(nvp, &mut value);
                    value
                };
                Value::I8(value)
            }

            DATA_TYPE_UINT8 => {
                let value = unsafe {
                    let mut value = 0;
                    sys::nvpair_value_uint8(nvp, &mut value);
                    value
                };
                Value::U8(value)
            }

            DATA_TYPE_BOOLEAN_ARRAY => {
                let value = unsafe {
                    let mut len = 0;
                    let mut data = ptr::null_mut();
                    sys::nvpair_value_boolean_array(nvp, &mut data, &mut len);
                    debug_assert!(!data.is_null());
                    let len = len as usize;
                    slice::from_raw_parts(data, len)
                        .iter()
                        .map(|item| *item == sys::boolean_t::B_TRUE)
                        .collect()
                };
                Value::BooleanArray(value)
            }

            DATA_TYPE_INT8_ARRAY => {
                let value = unsafe {
                    let mut len = 0;
                    let mut data = ptr::null_mut();
                    sys::nvpair_value_int8_array(nvp, &mut data, &mut len);
                    debug_assert!(!data.is_null());
                    let len = len as usize;
                    slice::from_raw_parts(data, len).to_vec()
                };
                Value::I8Array(value)
            }

            DATA_TYPE_UINT8_ARRAY => {
                let value = unsafe {
                    let mut len = 0;
                    let mut data = ptr::null_mut();
                    sys::nvpair_value_uint8_array(nvp, &mut data, &mut len);
                    debug_assert!(!data.is_null());
                    let len = len as usize;
                    slice::from_raw_parts(data, len).to_vec()
                };
                Value::U8Array(value)
            }

            DATA_TYPE_DOUBLE => todo!(),
            _ => unreachable!(),
        }
    }
}
