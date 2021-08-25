use std::borrow::Cow;
use std::convert::TryFrom;
use std::{ffi::CStr, slice};

use crate::error::value_or_err;

use super::*;

#[derive(Clone, Copy, PartialEq)]
pub struct NvPair {
    pub(crate) raw: *mut sys::nvpair_t,
}

impl NvPair {
    pub fn name(&self) -> Cow<'_, str> {
        let cstr = unsafe { CStr::from_ptr(sys::nvpair_name(self.raw)) };
        String::from_utf8_lossy(cstr.to_bytes())
    }

    pub fn r#type(&self) -> NvPairType {
        let nvp_type = unsafe { sys::nvpair_type(self.raw) };
        nvp_type.into()
    }

    pub fn value_boolean(&self) -> Result<bool> {
        let mut x = sys::boolean_t::B_FALSE;
        let val: *mut sys::boolean_t = &mut x;

        NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_boolean_value(self.raw, val) })?;

        match unsafe { val.as_ref() } {
            Some(boolval) => {
                if let sys::boolean_t::B_FALSE = *boolval {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_boolean_array(&self) -> Result<Vec<bool>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut boolean_arr: *mut sys::boolean_t = std::ptr::null_mut();
        let boolean_arr_ptr: *mut *mut sys::boolean_t = &mut boolean_arr;

        NvListError::from_nvlist_rc(unsafe {
            sys::nvpair_value_boolean_array(self.raw, boolean_arr_ptr, size_ptr)
        })?;

        match unsafe { boolean_arr_ptr.as_ref() } {
            Some(arr) => {
                let vec = unsafe { slice::from_raw_parts(*arr, size as usize).to_vec() };
                let mut bool_vec = Vec::with_capacity(vec.len());
                for c_bool in vec {
                    if c_bool == sys::boolean_t::B_TRUE {
                        bool_vec.push(true);
                    } else {
                        bool_vec.push(false);
                    }
                }

                Ok(bool_vec)
            }
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_uint8(&self) -> Result<u8> {
        let mut val = 0;
        let rc = unsafe { sys::nvpair_value_uint8(self.raw, &mut val) };
        value_or_err(val, rc)
    }

    pub fn value_uint16(&self) -> Result<u16> {
        let mut x = 0;
        let val: *mut u16 = &mut x;

        NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_uint16(self.raw, val) })?;

        match unsafe { val.as_ref() } {
            Some(u16val) => Ok(*u16val),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_uint32(&self) -> Result<u32> {
        let mut x = 0;
        let val: *mut u32 = &mut x;

        NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_uint32(self.raw, val) })?;

        match unsafe { val.as_ref() } {
            Some(u32val) => Ok(*u32val),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_uint64(&self) -> Result<u64> {
        let mut x = 0;
        let val: *mut u64 = &mut x;

        NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_uint64(self.raw, val) })?;

        match unsafe { val.as_ref() } {
            Some(u64val) => Ok(*u64val),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_float64(&self) -> Result<f64> {
        let mut x: f64 = 0.0;
        let val: *mut f64 = &mut x;

        NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_double(self.raw, val) })?;

        match unsafe { val.as_ref() } {
            Some(f64val) => Ok(*f64val),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_int8(&self) -> Result<i8> {
        let mut x = 0;
        let val: *mut i8 = &mut x;

        NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_int8(self.raw, val) })?;

        match unsafe { val.as_ref() } {
            Some(i8val) => Ok(*i8val),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_int16(&self) -> Result<i16> {
        let mut x = 0;
        let val: *mut i16 = &mut x;

        NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_int16(self.raw, val) })?;

        match unsafe { val.as_ref() } {
            Some(i16val) => Ok(*i16val),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_int32(&self) -> Result<i32> {
        let mut x = 0;
        let val: *mut i32 = &mut x;

        NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_int32(self.raw, val) })?;

        match unsafe { val.as_ref() } {
            Some(i32val) => Ok(*i32val),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_int64(&self) -> Result<i64> {
        let mut x = 0;
        let val: *mut i64 = &mut x;

        NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_int64(self.raw, val) })?;
        match unsafe { val.as_ref() } {
            Some(i64val) => Ok(*i64val),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_uint8_array(&self) -> Result<Vec<u8>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut u8arr: *mut u8 = std::ptr::null_mut();
        let u8arr_ptr: *mut *mut u8 = &mut u8arr;
        NvListError::from_nvlist_rc(unsafe {
            sys::nvpair_value_uint8_array(self.raw, u8arr_ptr, size_ptr)
        })?;

        match unsafe { u8arr_ptr.as_ref() } {
            Some(arr) => Ok(unsafe { slice::from_raw_parts(*arr, size as usize).to_vec() }),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_uint16_array(&self) -> Result<Vec<u16>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut u16arr: *mut u16 = std::ptr::null_mut();
        let u16arr_ptr: *mut *mut u16 = &mut u16arr;

        NvListError::from_nvlist_rc(unsafe {
            sys::nvpair_value_uint16_array(self.raw, u16arr_ptr, size_ptr)
        })?;

        match unsafe { u16arr_ptr.as_ref() } {
            Some(arr) => Ok(unsafe { slice::from_raw_parts(*arr, size as usize).to_vec() }),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_uint32_array(&self) -> Result<Vec<u32>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut u32arr: *mut u32 = std::ptr::null_mut();
        let u32arr_ptr: *mut *mut u32 = &mut u32arr;

        NvListError::from_nvlist_rc(unsafe {
            sys::nvpair_value_uint32_array(self.raw, u32arr_ptr, size_ptr)
        })?;

        match unsafe { u32arr_ptr.as_ref() } {
            Some(arr) => Ok(unsafe { slice::from_raw_parts(*arr, size as usize).to_vec() }),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_uint64_array(&self) -> Result<Vec<u64>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut u64arr: *mut u64 = std::ptr::null_mut();
        let u64arr_ptr: *mut *mut u64 = &mut u64arr;

        NvListError::from_nvlist_rc(unsafe {
            sys::nvpair_value_uint64_array(self.raw, u64arr_ptr, size_ptr)
        })?;

        match unsafe { u64arr_ptr.as_ref() } {
            Some(arr) => Ok(unsafe { slice::from_raw_parts(*arr, size as usize).to_vec() }),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_int8_array(&self) -> Result<Vec<i8>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut i8arr: *mut i8 = std::ptr::null_mut();
        let i8arr_ptr: *mut *mut i8 = &mut i8arr;

        NvListError::from_nvlist_rc(unsafe {
            sys::nvpair_value_int8_array(self.raw, i8arr_ptr, size_ptr)
        })?;

        match unsafe { i8arr_ptr.as_ref() } {
            Some(arr) => Ok(unsafe { slice::from_raw_parts(*arr, size as usize).to_vec() }),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_int16_array(&self) -> Result<Vec<i16>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut i16arr: *mut i16 = std::ptr::null_mut();
        let i16arr_ptr: *mut *mut i16 = &mut i16arr;

        NvListError::from_nvlist_rc(unsafe {
            sys::nvpair_value_int16_array(self.raw, i16arr_ptr, size_ptr)
        })?;

        match unsafe { i16arr_ptr.as_ref() } {
            Some(arr) => Ok(unsafe { slice::from_raw_parts(*arr, size as usize).to_vec() }),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_int32_array(&self) -> Result<Vec<i32>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut i32arr: *mut i32 = std::ptr::null_mut();
        let i32arr_ptr: *mut *mut i32 = &mut i32arr;

        NvListError::from_nvlist_rc(unsafe {
            sys::nvpair_value_int32_array(self.raw, i32arr_ptr, size_ptr)
        })?;

        match unsafe { i32arr_ptr.as_ref() } {
            Some(arr) => Ok(unsafe { slice::from_raw_parts(*arr, size as usize).to_vec() }),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_int64_array(&self) -> Result<Vec<i64>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut i64arr: *mut i64 = std::ptr::null_mut();
        let i64arr_ptr: *mut *mut i64 = &mut i64arr;

        NvListError::from_nvlist_rc(unsafe {
            sys::nvpair_value_int64_array(self.raw, i64arr_ptr, size_ptr)
        })?;

        match unsafe { i64arr_ptr.as_ref() } {
            Some(arr) => Ok(unsafe { slice::from_raw_parts(*arr, size as usize).to_vec() }),
            None => Err(NvListError::ConversionError),
        }
    }

    pub fn value_string(&self) -> Result<String> {
        let mut str: *mut libc::c_char = std::ptr::null_mut();
        let str_ptr: *mut *mut libc::c_char = &mut str;

        NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_string(self.raw, str_ptr) })?;
        let name = unsafe { CStr::from_ptr(*str_ptr).to_str()?.to_string() };
        Ok(name)
    }

    pub fn value_string_array(&self) -> Result<Vec<String>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut str: *mut libc::c_char = std::ptr::null_mut();
        let mut str_ptr: *mut *mut libc::c_char = &mut str;
        let str_arr_ptr: *mut *mut *mut libc::c_char = &mut str_ptr;

        NvListError::from_nvlist_rc(unsafe {
            sys::nvpair_value_string_array(self.raw, str_arr_ptr, size_ptr)
        })?;

        let mut str_vec = unsafe { Vec::with_capacity(*size_ptr as usize) };
        let cstr_vec = unsafe { slice::from_raw_parts(*str_arr_ptr, size as usize).to_vec() };

        for cstr in cstr_vec {
            str_vec.push(unsafe { CStr::from_ptr(cstr).to_str()?.to_string() });
        }

        Ok(str_vec)
    }

    pub fn value_nvlist(&self) -> Result<NvList> {
        let mut raw_nvlist: *mut sys::nvlist_t = std::ptr::null_mut();
        let raw_nvlist_ptr: *mut *mut sys::nvlist_t = &mut raw_nvlist;

        NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_nvlist(self.raw, raw_nvlist_ptr) })?;
        let nvlist = NvList::from(unsafe { *raw_nvlist_ptr });

        Ok(nvlist)
    }

    pub fn value_nvlist_array(&self) -> Result<Vec<NvList>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut nvlist: *mut sys::nvlist_t = std::ptr::null_mut();
        let mut nvlist_ptr: *mut *mut sys::nvlist_t = &mut nvlist;
        let nvlist_arr_ptr: *mut *mut *mut sys::nvlist_t = &mut nvlist_ptr;

        NvListError::from_nvlist_rc(unsafe {
            sys::nvpair_value_nvlist_array(self.raw, nvlist_arr_ptr, size_ptr)
        })?;

        match unsafe { nvlist_arr_ptr.as_ref() } {
            Some(arr) => {
                let vec_ptr = unsafe { slice::from_raw_parts(*arr, size as usize).to_vec() };
                let mut nvlist_vec = Vec::with_capacity(vec_ptr.len());
                for nvlist in vec_ptr {
                    nvlist_vec.push(NvList::from(nvlist))
                }

                Ok(nvlist_vec)
            }
            None => Err(NvListError::ConversionError),
        }
    }
}

impl From<*mut sys::nvpair_t> for NvPair {
    fn from(nvp: *mut sys::nvpair_t) -> Self {
        Self { raw: nvp }
    }
}

#[derive(Debug)]
pub struct CtxIter<ContextType> {
    vec: ContextType,
    index: usize,
}

impl TryFrom<NvPair> for CtxIter<ContextType> {
    type Error = NvListError;
    fn try_from(nvpair: NvPair) -> Result<Self> {
        match nvpair.r#type() {
            NvPairType::Uint8Array => {
                let vec = nvpair.value_uint8_array()?;
                Ok(Self {
                    vec: ContextType::U8Arr(vec),
                    index: 0,
                })
            }
            NvPairType::Uint16Array => {
                let vec = nvpair.value_uint16_array()?;
                Ok(Self {
                    vec: ContextType::U16Arr(vec),
                    index: 0,
                })
            }
            NvPairType::Uint32Array => {
                let vec = nvpair.value_uint32_array()?;
                Ok(Self {
                    vec: ContextType::U32Arr(vec),
                    index: 0,
                })
            }
            NvPairType::Uint64Array => {
                let vec = nvpair.value_uint64_array()?;
                Ok(Self {
                    vec: ContextType::U64Arr(vec),
                    index: 0,
                })
            }
            NvPairType::Int8Array => {
                let vec = nvpair.value_int8_array()?;
                Ok(Self {
                    vec: ContextType::I8Arr(vec),
                    index: 0,
                })
            }
            NvPairType::Int16Array => {
                let vec = nvpair.value_int16_array()?;
                Ok(Self {
                    vec: ContextType::I16Arr(vec),
                    index: 0,
                })
            }
            NvPairType::Int32Array => {
                let vec = nvpair.value_int32_array()?;
                Ok(Self {
                    vec: ContextType::I32Arr(vec),
                    index: 0,
                })
            }
            NvPairType::Int64Array => {
                let vec = nvpair.value_int64_array()?;
                Ok(Self {
                    vec: ContextType::I64Arr(vec),
                    index: 0,
                })
            }
            NvPairType::BooleanArray => {
                let vec = nvpair.value_boolean_array()?;
                Ok(Self {
                    vec: ContextType::BooleanArr(vec),
                    index: 0,
                })
            }
            NvPairType::NvlistArray => {
                let vec = nvpair.value_nvlist_array()?;
                Ok(Self {
                    vec: ContextType::NvListArr(vec),
                    index: 0,
                })
            }
            NvPairType::StringArray => {
                let vec = nvpair.value_string_array()?;
                Ok(Self {
                    vec: ContextType::StrArr(vec),
                    index: 0,
                })
            }
            _ => Err(NvListError::UnmatchingVariables),
        }
    }
}

impl Iterator for CtxIter<ContextType> {
    type Item = ContextType;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.vec {
            ContextType::U8Arr(vec) => {
                if self.index < vec.len() {
                    let index = self.index;
                    self.index += 1;
                    Some(ContextType::U8(vec[index]))
                } else {
                    None
                }
            }
            ContextType::I8Arr(vec) => {
                if self.index < vec.len() {
                    let index = self.index;
                    self.index += 1;
                    Some(ContextType::I8(vec[index]))
                } else {
                    None
                }
            }
            ContextType::U16Arr(vec) => {
                if self.index < vec.len() {
                    let index = self.index;
                    self.index += 1;
                    Some(ContextType::U16(vec[index]))
                } else {
                    None
                }
            }
            ContextType::I16Arr(vec) => {
                if self.index < vec.len() {
                    let index = self.index;
                    self.index += 1;
                    Some(ContextType::I16(vec[index]))
                } else {
                    None
                }
            }
            ContextType::U32Arr(vec) => {
                if self.index < vec.len() {
                    let index = self.index;
                    self.index += 1;
                    Some(ContextType::U32(vec[index]))
                } else {
                    None
                }
            }
            ContextType::I32Arr(vec) => {
                if self.index < vec.len() {
                    let index = self.index;
                    self.index += 1;
                    Some(ContextType::I32(vec[index]))
                } else {
                    None
                }
            }
            ContextType::U64Arr(vec) => {
                if self.index < vec.len() {
                    let index = self.index;
                    self.index += 1;
                    Some(ContextType::U64(vec[index]))
                } else {
                    None
                }
            }
            ContextType::I64Arr(vec) => {
                if self.index < vec.len() {
                    let index = self.index;
                    self.index += 1;
                    Some(ContextType::I64(vec[index]))
                } else {
                    None
                }
            }
            ContextType::BooleanArr(vec) => {
                if self.index < vec.len() {
                    let index = self.index;
                    self.index += 1;
                    Some(ContextType::Boolean(vec[index]))
                } else {
                    None
                }
            }
            ContextType::StrArr(vec) => {
                if self.index < vec.len() {
                    let index = self.index;
                    self.index += 1;
                    // TODO: check if it is ok to clone here
                    Some(ContextType::Str(vec[index].clone()))
                } else {
                    None
                }
            }
            ContextType::DoubleArr(vec) => {
                if self.index < vec.len() {
                    let index = self.index;
                    self.index += 1;
                    Some(ContextType::Double(vec[index]))
                } else {
                    None
                }
            }
            ContextType::NvListArr(_vec) => todo!(),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContextType {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    Boolean(bool),
    Str(String),
    Double(f64),
    NvList(NvList),
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
    NvListArr(Vec<NvList>),
    Empty,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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

pub trait SafeNvPair {}

impl SafeNvPair for u8 {}
impl SafeNvPair for u16 {}
impl SafeNvPair for u32 {}
impl SafeNvPair for u64 {}
impl SafeNvPair for i8 {}
impl SafeNvPair for i16 {}
impl SafeNvPair for i32 {}
impl SafeNvPair for i64 {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn basic_iter_u8() {
        let mut nvlist = NvList::new(NvFlag::UniqueName).unwrap();
        let arr: [u8; 5] = [1, 2, 3, 4, 5];
        nvlist.add_uint8_arr("d", arr).unwrap();
        let nvpair =
            NvPair::from(unsafe { sys::nvlist_next_nvpair(nvlist.raw, std::ptr::null_mut()) });
        let mut iter: CtxIter<ContextType> = nvpair.try_into().unwrap();
        assert_eq!(Some(ContextType::U8(1)), iter.next());
        assert_eq!(Some(ContextType::U8(2)), iter.next());
        assert_eq!(Some(ContextType::U8(3)), iter.next());
        assert_eq!(Some(ContextType::U8(4)), iter.next());
        assert_eq!(Some(ContextType::U8(5)), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn basic_iter_u16() {
        let mut nvlist = NvList::new(NvFlag::UniqueName).unwrap();
        let arr: [u16; 5] = [1, 2, 3, 4, 5];
        nvlist.add_uint16_arr("d", arr).unwrap();
        let nvpair =
            NvPair::from(unsafe { sys::nvlist_next_nvpair(nvlist.raw, std::ptr::null_mut()) });
        let mut iter: CtxIter<ContextType> = nvpair.try_into().unwrap();
        assert_eq!(Some(ContextType::U16(1)), iter.next());
        assert_eq!(Some(ContextType::U16(2)), iter.next());
        assert_eq!(Some(ContextType::U16(3)), iter.next());
        assert_eq!(Some(ContextType::U16(4)), iter.next());
        assert_eq!(Some(ContextType::U16(5)), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn basic_iter_u32() {
        let mut nvlist = NvList::new(NvFlag::UniqueName).unwrap();
        let arr: [u32; 5] = [1, 2, 3, 4, 5];
        nvlist.add_uint32_arr("d", arr).unwrap();
        let nvpair =
            NvPair::from(unsafe { sys::nvlist_next_nvpair(nvlist.raw, std::ptr::null_mut()) });
        let mut iter: CtxIter<ContextType> = nvpair.try_into().unwrap();
        assert_eq!(Some(ContextType::U32(1)), iter.next());
        assert_eq!(Some(ContextType::U32(2)), iter.next());
        assert_eq!(Some(ContextType::U32(3)), iter.next());
        assert_eq!(Some(ContextType::U32(4)), iter.next());
        assert_eq!(Some(ContextType::U32(5)), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn basic_iter_u64() {
        let mut nvlist = NvList::new(NvFlag::UniqueName).unwrap();
        let arr: [u64; 5] = [1, 2, 3, 4, 5];
        nvlist.add_uint64_arr("d", arr).unwrap();
        let nvpair =
            NvPair::from(unsafe { sys::nvlist_next_nvpair(nvlist.raw, std::ptr::null_mut()) });
        let mut iter: CtxIter<ContextType> = nvpair.try_into().unwrap();
        assert_eq!(Some(ContextType::U64(1)), iter.next());
        assert_eq!(Some(ContextType::U64(2)), iter.next());
        assert_eq!(Some(ContextType::U64(3)), iter.next());
        assert_eq!(Some(ContextType::U64(4)), iter.next());
        assert_eq!(Some(ContextType::U64(5)), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}
