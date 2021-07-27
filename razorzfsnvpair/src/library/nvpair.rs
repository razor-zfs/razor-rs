use std::convert::TryFrom;
use std::{ffi::CStr, slice};

use super::*;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct NvPair {
    pub raw_nvpair: *mut sys::nvpair_t,
}

impl NvPair {
    pub fn default() -> NvPair {
        NvPair {
            raw_nvpair: std::ptr::null_mut(),
        }
    }

    pub fn name(&self) -> Result<String> {
        unsafe {
            Ok(CStr::from_ptr(sys::nvpair_name(self.raw_nvpair))
                .to_str()?
                .to_string())
        }
    }

    pub fn r#type(&self) -> NvPairType {
        unsafe { sys::nvpair_type(self.raw_nvpair).into() }
    }

    pub fn value_uint8(&self) -> Result<u8> {
        let mut x = 0;
        let val: *mut u8 = &mut x;

        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_uint8(self.raw_nvpair, val))?;

            match val.as_ref() {
                Some(u8val) => Ok(*u8val),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_uint16(&mut self) -> Result<u16> {
        let mut x = 0;
        let val: *mut u16 = &mut x;

        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_uint16(self.raw_nvpair, val))?;

            match val.as_ref() {
                Some(u16val) => Ok(*u16val),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_uint32(&self) -> Result<u32> {
        let mut x = 0;
        let val: *mut u32 = &mut x;

        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_uint32(self.raw_nvpair, val))?;

            match val.as_ref() {
                Some(u32val) => Ok(*u32val),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_uint64(&self) -> Result<u64> {
        let mut x = 0;
        let val: *mut u64 = &mut x;

        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_uint64(self.raw_nvpair, val))?;

            match val.as_ref() {
                Some(u64val) => Ok(*u64val),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_int8(&self) -> Result<i8> {
        let mut x = 0;
        let val: *mut i8 = &mut x;

        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_int8(self.raw_nvpair, val))?;

            match val.as_ref() {
                Some(i8val) => Ok(*i8val),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_int16(&mut self) -> Result<i16> {
        let mut x = 0;
        let val: *mut i16 = &mut x;

        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_int16(self.raw_nvpair, val))?;

            match val.as_ref() {
                Some(i16val) => Ok(*i16val),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_int32(&self) -> Result<i32> {
        let mut x = 0;
        let val: *mut i32 = &mut x;

        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_int32(self.raw_nvpair, val))?;

            match val.as_ref() {
                Some(i32val) => Ok(*i32val),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_int64(&self) -> Result<i64> {
        let mut x = 0;
        let val: *mut i64 = &mut x;

        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_int64(self.raw_nvpair, val))?;

            match val.as_ref() {
                Some(i64val) => Ok(*i64val),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_uint8_array(&mut self) -> Result<Vec<u8>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut u8arr: *mut u8 = std::ptr::null_mut();
        let u8arr_ptr: *mut *mut u8 = &mut u8arr;
        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_uint8_array(
                self.raw_nvpair,
                u8arr_ptr,
                size_ptr,
            ))?;

            match u8arr_ptr.as_ref() {
                Some(arr) => Ok(slice::from_raw_parts(*arr, size as usize).to_vec()),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_uint16_array(&mut self) -> Result<Vec<u16>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut u16arr: *mut u16 = std::ptr::null_mut();
        let u16arr_ptr: *mut *mut u16 = &mut u16arr;
        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_uint16_array(
                self.raw_nvpair,
                u16arr_ptr,
                size_ptr,
            ))?;

            match u16arr_ptr.as_ref() {
                Some(arr) => Ok(slice::from_raw_parts(*arr, size as usize).to_vec()),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_uint32_array(&mut self) -> Result<Vec<u32>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut u32arr: *mut u32 = std::ptr::null_mut();
        let u32arr_ptr: *mut *mut u32 = &mut u32arr;
        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_uint32_array(
                self.raw_nvpair,
                u32arr_ptr,
                size_ptr,
            ))?;

            match u32arr_ptr.as_ref() {
                Some(arr) => Ok(slice::from_raw_parts(*arr, size as usize).to_vec()),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_uint64_array(&mut self) -> Result<Vec<u64>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut u64arr: *mut u64 = std::ptr::null_mut();
        let u64arr_ptr: *mut *mut u64 = &mut u64arr;
        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_uint64_array(
                self.raw_nvpair,
                u64arr_ptr,
                size_ptr,
            ))?;

            match u64arr_ptr.as_ref() {
                Some(arr) => Ok(slice::from_raw_parts(*arr, size as usize).to_vec()),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_int8_array(&mut self) -> Result<Vec<i8>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut i8arr: *mut i8 = std::ptr::null_mut();
        let i8arr_ptr: *mut *mut i8 = &mut i8arr;
        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_int8_array(
                self.raw_nvpair,
                i8arr_ptr,
                size_ptr,
            ))?;

            match i8arr_ptr.as_ref() {
                Some(arr) => Ok(slice::from_raw_parts(*arr, size as usize).to_vec()),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_int16_array(&mut self) -> Result<Vec<i16>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut i16arr: *mut i16 = std::ptr::null_mut();
        let i16arr_ptr: *mut *mut i16 = &mut i16arr;
        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_int16_array(
                self.raw_nvpair,
                i16arr_ptr,
                size_ptr,
            ))?;

            match i16arr_ptr.as_ref() {
                Some(arr) => Ok(slice::from_raw_parts(*arr, size as usize).to_vec()),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_int32_array(&mut self) -> Result<Vec<i32>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut i32arr: *mut i32 = std::ptr::null_mut();
        let i32arr_ptr: *mut *mut i32 = &mut i32arr;
        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_int32_array(
                self.raw_nvpair,
                i32arr_ptr,
                size_ptr,
            ))?;

            match i32arr_ptr.as_ref() {
                Some(arr) => Ok(slice::from_raw_parts(*arr, size as usize).to_vec()),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_int64_array(&mut self) -> Result<Vec<i64>> {
        let mut size = 0;
        let size_ptr: *mut sys::uint_t = &mut size;
        let mut i64arr: *mut i64 = std::ptr::null_mut();
        let i64arr_ptr: *mut *mut i64 = &mut i64arr;
        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_int64_array(
                self.raw_nvpair,
                i64arr_ptr,
                size_ptr,
            ))?;

            match i64arr_ptr.as_ref() {
                Some(arr) => Ok(slice::from_raw_parts(*arr, size as usize).to_vec()),
                None => Err(NvListError::ConversionError),
            }
        }
    }

    pub fn value_string(&self) -> Result<String> {
        let mut str: *mut u8 = std::ptr::null_mut();
        let str_ptr: *mut *mut u8 = &mut str;

        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_string(self.raw_nvpair, str_ptr))?;
            let name = CStr::from_ptr(*str_ptr).to_str()?.to_string();
            Ok(name)
        }
    }

    pub fn value_nvlist(&self) -> Result<NvList> {
        let mut raw_nvlist: *mut sys::nvlist_t = std::ptr::null_mut();
        let raw_nvlist_ptr: *mut *mut sys::nvlist_t = &mut raw_nvlist;

        unsafe {
            NvListError::from_nvlist_rc(sys::nvpair_value_nvlist(self.raw_nvpair, raw_nvlist_ptr))?;
            let nvlist = NvList {
                raw: *raw_nvlist_ptr,
            };

            Ok(nvlist)
        }
    }
}

pub struct CtxIter<ContextType> {
    vec: ContextType,
    index: usize,
}

impl TryFrom<NvPair> for CtxIter<ContextType> {
    type Error = NvListError;
    fn try_from(mut nvpair: NvPair) -> Result<Self> {
        match nvpair.r#type() {
            NvPairType::Uint16Array => {
                let vec = nvpair.value_uint16_array()?;
                Ok(CtxIter {
                    vec: ContextType::U16Arr(vec),
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
            ContextType::NvListArr(vec) => {
                if self.index < vec.len() {
                    let index = self.index;
                    self.index += 1;
                    // TODO: check if it is ok to copy here
                    Some(ContextType::NvList(vec[index]))
                } else {
                    None
                }
            }
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
    //generate_iterators!(u8, u16, u32, u64, i8, i16, i32, i64);

    #[test]
    fn basic_iter_u8() {
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        let arr: [u8; 5] = [1, 2, 3, 4, 5];
        nvlist.add_uint8_arr("d", arr).unwrap();
        let mut nvpair = NvPair::default();
        nvpair.raw_nvpair = unsafe { sys::nvlist_next_nvpair(nvlist.raw, nvpair.raw_nvpair) };
        /*let mut iter: CtxIter<ContextType> = nvpair.try_into().unwrap();
        assert_eq!(Some(ContextType::U16(1)), iter.next());
        assert_eq!(Some(ContextType::U16(2)), iter.next());
        assert_eq!(Some(ContextType::U16(3)), iter.next());
        assert_eq!(Some(ContextType::U16(4)), iter.next());
        assert_eq!(Some(ContextType::U16(5)), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());*/
    }

    #[test]
    fn basic_iter_u16() {
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        let arr: [u16; 5] = [1, 2, 3, 4, 5];
        nvlist.add_uint16_arr("d", arr).unwrap();
        let mut nvpair = NvPair::default();
        nvpair.raw_nvpair = unsafe { sys::nvlist_next_nvpair(nvlist.raw, nvpair.raw_nvpair) };
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
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        let arr: [u32; 5] = [1, 2, 3, 4, 5];
        nvlist.add_uint32_arr("d", arr).unwrap();
        let mut nvpair = NvPair::default();
        nvpair.raw_nvpair = unsafe { sys::nvlist_next_nvpair(nvlist.raw, nvpair.raw_nvpair) };
        /*let mut iter: Iter<u32> = nvpair.try_into().unwrap();
        assert_eq!(Some(1_u32), iter.next());
        assert_eq!(Some(2_u32), iter.next());
        assert_eq!(Some(3_u32), iter.next());
        assert_eq!(Some(4_u32), iter.next());
        assert_eq!(Some(5_u32), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());*/
    }

    #[test]
    fn basic_iter_u64() {
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        let arr: [u64; 5] = [1, 2, 3, 4, 5];
        nvlist.add_uint64_arr("d", arr).unwrap();
        let mut nvpair = NvPair::default();
        nvpair.raw_nvpair = unsafe { sys::nvlist_next_nvpair(nvlist.raw, nvpair.raw_nvpair) };
        /*let mut iter: Iter<u64> = nvpair.try_into().unwrap();
        assert_eq!(Some(1_u64), iter.next());
        assert_eq!(Some(2_u64), iter.next());
        assert_eq!(Some(3_u64), iter.next());
        assert_eq!(Some(4_u64), iter.next());
        assert_eq!(Some(5_u64), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());*/
    }
}
