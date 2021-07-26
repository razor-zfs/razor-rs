use std::convert::TryFrom;
use std::marker::PhantomData;
use std::{ffi::CStr, slice};

use super::*;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct NvPair {
    pub raw_nvpair: *mut sys::nvpair_t,
    //pub pair_name: String,
    //pub pair_value: ContextType,
}

impl NvPair {
    pub fn default() -> NvPair {
        NvPair {
            raw_nvpair: std::ptr::null_mut(),
            //pair_name: "".to_string(),
            //pair_value: ContextType::Empty,
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
}

pub struct Iter<T>
where
    T: SafeNvPair,
{
    vec: Vec<T>,
    index: usize,
    _marker: PhantomData<T>,
}

impl TryFrom<NvPair> for Iter<u8> {
    type Error = NvListError;
    fn try_from(mut nvpair: NvPair) -> Result<Self> {
        match nvpair.r#type() {
            NvPairType::Uint8Array => {
                let vec = nvpair.value_uint8_array()?;
                Ok(Iter::<u8> {
                    vec,
                    index: 0,
                    _marker: PhantomData,
                })
            }
            _ => Err(NvListError::UnmatchingVariables),
        }
    }
}

impl TryFrom<NvPair> for Iter<u16> {
    type Error = NvListError;
    fn try_from(mut nvpair: NvPair) -> Result<Self> {
        match nvpair.r#type() {
            NvPairType::Uint16Array => {
                let vec = nvpair.value_uint16_array()?;
                Ok(Iter::<u16> {
                    vec,
                    index: 0,
                    _marker: PhantomData,
                })
            }
            _ => Err(NvListError::UnmatchingVariables),
        }
    }
}

impl TryFrom<NvPair> for Iter<u32> {
    type Error = NvListError;
    fn try_from(mut nvpair: NvPair) -> Result<Self> {
        match nvpair.r#type() {
            NvPairType::Uint32Array => {
                let vec = nvpair.value_uint32_array()?;
                Ok(Iter::<u32> {
                    vec,
                    index: 0,
                    _marker: PhantomData,
                })
            }
            _ => Err(NvListError::UnmatchingVariables),
        }
    }
}

impl TryFrom<NvPair> for Iter<u64> {
    type Error = NvListError;
    fn try_from(mut nvpair: NvPair) -> Result<Self> {
        match nvpair.r#type() {
            NvPairType::Uint64Array => {
                let vec = nvpair.value_uint64_array()?;
                Ok(Iter::<u64> {
                    vec,
                    index: 0,
                    _marker: PhantomData,
                })
            }
            _ => Err(NvListError::UnmatchingVariables),
        }
    }
}

macro_rules! generate_iterators {
    ($($typ:ty),*) => {
        $(
            impl Iterator for Iter<$typ> {
                type Item = $typ;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.index < self.vec.len() {
                        let index = self.index;
                        self.index += 1;
                        Some(self.vec[index])
                    } else {
                        None
                    }
                }
            }
        )*
    };
}

generate_iterators!(u8, u16, u32, u64, i8, i16, i32, i64);

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
        let mut nvlist = nvlist_alloc(NvFlag::UniqueName).unwrap();
        let arr: [u8; 5] = [1, 2, 3, 4, 5];
        nvlist_add_uint8_arr(&nvlist, "d", arr).unwrap();
        let mut nvpair = NvPair::default();
        nvpair.raw_nvpair = unsafe { sys::nvlist_next_nvpair(nvlist.raw, nvpair.raw_nvpair) };
        let mut iter: Iter<u8> = nvpair.try_into().unwrap();
        assert_eq!(Some(1_u8), iter.next());
        assert_eq!(Some(2_u8), iter.next());
        assert_eq!(Some(3_u8), iter.next());
        assert_eq!(Some(4_u8), iter.next());
        assert_eq!(Some(5_u8), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn basic_iter_u16() {
        let mut nvlist = nvlist_alloc(NvFlag::UniqueName).unwrap();
        let arr: [u16; 5] = [1, 2, 3, 4, 5];
        nvlist_add_uint16_arr(&nvlist, "d", arr).unwrap();
        let mut nvpair = NvPair::default();
        nvpair.raw_nvpair = unsafe { sys::nvlist_next_nvpair(nvlist.raw, nvpair.raw_nvpair) };
        let mut iter: Iter<u16> = nvpair.try_into().unwrap();
        assert_eq!(Some(1_u16), iter.next());
        assert_eq!(Some(2_u16), iter.next());
        assert_eq!(Some(3_u16), iter.next());
        assert_eq!(Some(4_u16), iter.next());
        assert_eq!(Some(5_u16), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn basic_iter_u32() {
        let mut nvlist = nvlist_alloc(NvFlag::UniqueName).unwrap();
        let arr: [u32; 5] = [1, 2, 3, 4, 5];
        nvlist_add_uint32_arr(&nvlist, "d", arr).unwrap();
        let mut nvpair = NvPair::default();
        nvpair.raw_nvpair = unsafe { sys::nvlist_next_nvpair(nvlist.raw, nvpair.raw_nvpair) };
        let mut iter: Iter<u32> = nvpair.try_into().unwrap();
        assert_eq!(Some(1_u32), iter.next());
        assert_eq!(Some(2_u32), iter.next());
        assert_eq!(Some(3_u32), iter.next());
        assert_eq!(Some(4_u32), iter.next());
        assert_eq!(Some(5_u32), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn basic_iter_u64() {
        let mut nvlist = nvlist_alloc(NvFlag::UniqueName).unwrap();
        let arr: [u64; 5] = [1, 2, 3, 4, 5];
        nvlist_add_uint64_arr(&nvlist, "d", arr).unwrap();
        let mut nvpair = NvPair::default();
        nvpair.raw_nvpair = unsafe { sys::nvlist_next_nvpair(nvlist.raw, nvpair.raw_nvpair) };
        let mut iter: Iter<u64> = nvpair.try_into().unwrap();
        assert_eq!(Some(1_u64), iter.next());
        assert_eq!(Some(2_u64), iter.next());
        assert_eq!(Some(3_u64), iter.next());
        assert_eq!(Some(4_u64), iter.next());
        assert_eq!(Some(5_u64), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}
