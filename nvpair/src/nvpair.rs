use std::borrow::Cow;
use std::ffi;
use std::slice;

use razor_libnvpair as libnvpair;

use super::*;

#[derive(Clone, Copy, PartialEq)]
pub struct NvPair {
    nvp: *mut libnvpair::nvpair_t,
}

impl NvPair {
    pub fn nvp(&self) -> *mut libnvpair::nvpair_t {
        self.nvp
    }

    #[inline]
    pub fn value(&self) -> Value {
        to_value(self)
    }

    #[inline]
    pub fn name(&self) -> Cow<'_, str> {
        unsafe { ffi::CStr::from_ptr(libnvpair::nvpair_name(self.nvp)) }.to_string_lossy()
    }

    #[inline]
    pub fn r#type(&self) -> libnvpair::data_type_t {
        unsafe { libnvpair::nvpair_type(self.nvp) }
    }

    #[inline]
    pub fn boolean(&self) -> libnvpair::boolean_t {
        unsafe { libnvpair::nvpair_value_boolean_value(self.nvp) }
    }

    #[inline]
    pub fn byte(&self) -> char {
        unsafe {
            let value = libnvpair::nvpair_value_byte(self.nvp).into();
            char::from_u32_unchecked(value)
        }
    }

    #[inline]
    pub fn int8(&self) -> i8 {
        unsafe { libnvpair::nvpair_value_int8(self.nvp) }
    }

    #[inline]
    pub fn uint8(&self) -> u8 {
        unsafe { libnvpair::nvpair_value_uint8(self.nvp) }
    }

    #[inline]
    pub fn int16(&self) -> i16 {
        unsafe { libnvpair::nvpair_value_int16(self.nvp) }
    }

    #[inline]
    pub fn uint16(&self) -> u16 {
        unsafe { libnvpair::nvpair_value_uint16(self.nvp) }
    }

    #[inline]
    pub fn int32(&self) -> i32 {
        unsafe { libnvpair::nvpair_value_int32(self.nvp) }
    }

    #[inline]
    pub fn uint32(&self) -> u32 {
        unsafe { libnvpair::nvpair_value_uint32(self.nvp) }
    }

    #[inline]
    pub fn int64(&self) -> i64 {
        unsafe { libnvpair::nvpair_value_int64(self.nvp) }
    }

    #[inline]
    pub fn uint64(&self) -> u64 {
        unsafe { libnvpair::nvpair_value_uint64(self.nvp) }
    }

    #[inline]
    pub fn string(&self) -> Cow<'_, str> {
        unsafe {
            let cstr = libnvpair::nvpair_value_string(self.nvp);
            debug_assert!(!cstr.is_null());
            ffi::CStr::from_ptr(cstr).to_string_lossy()
        }
    }

    #[inline]
    pub fn nvlist(&self) -> NvListRef<'_, Self> {
        let nvl = unsafe { libnvpair::nvpair_value_nvlist(self.nvp) };
        NvListRef::from_raw(nvl, self)
    }

    #[inline]
    pub fn boolean_array(&self) -> &[libnvpair::boolean_t] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_boolean_array(self.nvp);
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    #[inline]
    pub fn int8_array(&self) -> &[i8] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_int8_array(self.nvp);
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    #[inline]
    pub fn uint8_array(&self) -> &[u8] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_uint8_array(self.nvp);
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    #[inline]
    pub fn int16_array(&self) -> &[i16] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_int16_array(self.nvp);
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    #[inline]
    pub fn uint16_array(&self) -> &[u16] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_uint16_array(self.nvp);
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    #[inline]
    pub fn int32_array(&self) -> &[i32] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_int32_array(self.nvp);
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    #[inline]
    pub fn uint32_array(&self) -> &[u32] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_uint32_array(self.nvp);
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    #[inline]
    pub fn int64_array(&self) -> &[i64] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_int64_array(self.nvp);
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    #[inline]
    pub fn uint64_array(&self) -> &[u64] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_uint64_array(self.nvp);
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    #[inline]
    pub fn string_array(&self) -> Vec<Cow<'_, str>> {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_string_array(self.nvp);
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
                .iter()
                .map(|item| ffi::CStr::from_ptr(*item).to_string_lossy())
                .collect::<Vec<_>>()
        }
    }
}

impl From<*mut libnvpair::nvpair_t> for NvPair {
    fn from(nvp: *mut libnvpair::nvpair_t) -> Self {
        Self { nvp }
    }
}

// #[derive(Debug)]
// pub struct CtxIter<ContextType> {
//     vec: ContextType,
//     index: usize,
// }

// impl TryFrom<NvPair> for CtxIter<ContextType> {
//     type Error = NvListError;
//     fn try_from(nvpair: NvPair) -> Result<Self> {
//         match nvpair.r#type() {
//             NvPairType::Uint8Array => {
//                 let vec = nvpair.value_uint8_array()?;
//                 Ok(Self {
//                     vec: ContextType::U8Arr(vec),
//                     index: 0,
//                 })
//             }
//             NvPairType::Uint16Array => {
//                 let vec = nvpair.value_uint16_array()?;
//                 Ok(Self {
//                     vec: ContextType::U16Arr(vec),
//                     index: 0,
//                 })
//             }
//             NvPairType::Uint32Array => {
//                 let vec = nvpair.value_uint32_array()?;
//                 Ok(Self {
//                     vec: ContextType::U32Arr(vec),
//                     index: 0,
//                 })
//             }
//             NvPairType::Uint64Array => {
//                 let vec = nvpair.value_uint64_array()?;
//                 Ok(Self {
//                     vec: ContextType::U64Arr(vec),
//                     index: 0,
//                 })
//             }
//             NvPairType::Int8Array => {
//                 let vec = nvpair.value_int8_array()?;
//                 Ok(Self {
//                     vec: ContextType::I8Arr(vec),
//                     index: 0,
//                 })
//             }
//             NvPairType::Int16Array => {
//                 let vec = nvpair.value_int16_array()?;
//                 Ok(Self {
//                     vec: ContextType::I16Arr(vec),
//                     index: 0,
//                 })
//             }
//             NvPairType::Int32Array => {
//                 let vec = nvpair.value_int32_array()?;
//                 Ok(Self {
//                     vec: ContextType::I32Arr(vec),
//                     index: 0,
//                 })
//             }
//             NvPairType::Int64Array => {
//                 let vec = nvpair.value_int64_array()?;
//                 Ok(Self {
//                     vec: ContextType::I64Arr(vec),
//                     index: 0,
//                 })
//             }
//             NvPairType::BooleanArray => {
//                 let vec = nvpair.value_boolean_array()?;
//                 Ok(Self {
//                     vec: ContextType::BooleanArr(vec),
//                     index: 0,
//                 })
//             }
//             NvPairType::NvlistArray => {
//                 let vec = nvpair.value_nvlist_array()?;
//                 Ok(Self {
//                     vec: ContextType::NvListArr(vec),
//                     index: 0,
//                 })
//             }
//             NvPairType::StringArray => {
//                 let vec = nvpair.value_string_array()?;
//                 Ok(Self {
//                     vec: ContextType::StrArr(vec),
//                     index: 0,
//                 })
//             }
//             _ => Err(NvListError::UnmatchingVariables),
//         }
//     }
// }

// impl Iterator for CtxIter<ContextType> {
//     type Item = ContextType;

//     fn next(&mut self) -> Option<Self::Item> {
//         match &self.vec {
//             ContextType::U8Arr(vec) => {
//                 if self.index < vec.len() {
//                     let index = self.index;
//                     self.index += 1;
//                     Some(ContextType::U8(vec[index]))
//                 } else {
//                     None
//                 }
//             }
//             ContextType::I8Arr(vec) => {
//                 if self.index < vec.len() {
//                     let index = self.index;
//                     self.index += 1;
//                     Some(ContextType::I8(vec[index]))
//                 } else {
//                     None
//                 }
//             }
//             ContextType::U16Arr(vec) => {
//                 if self.index < vec.len() {
//                     let index = self.index;
//                     self.index += 1;
//                     Some(ContextType::U16(vec[index]))
//                 } else {
//                     None
//                 }
//             }
//             ContextType::I16Arr(vec) => {
//                 if self.index < vec.len() {
//                     let index = self.index;
//                     self.index += 1;
//                     Some(ContextType::I16(vec[index]))
//                 } else {
//                     None
//                 }
//             }
//             ContextType::U32Arr(vec) => {
//                 if self.index < vec.len() {
//                     let index = self.index;
//                     self.index += 1;
//                     Some(ContextType::U32(vec[index]))
//                 } else {
//                     None
//                 }
//             }
//             ContextType::I32Arr(vec) => {
//                 if self.index < vec.len() {
//                     let index = self.index;
//                     self.index += 1;
//                     Some(ContextType::I32(vec[index]))
//                 } else {
//                     None
//                 }
//             }
//             ContextType::U64Arr(vec) => {
//                 if self.index < vec.len() {
//                     let index = self.index;
//                     self.index += 1;
//                     Some(ContextType::U64(vec[index]))
//                 } else {
//                     None
//                 }
//             }
//             ContextType::I64Arr(vec) => {
//                 if self.index < vec.len() {
//                     let index = self.index;
//                     self.index += 1;
//                     Some(ContextType::I64(vec[index]))
//                 } else {
//                     None
//                 }
//             }
//             ContextType::BooleanArr(vec) => {
//                 if self.index < vec.len() {
//                     let index = self.index;
//                     self.index += 1;
//                     Some(ContextType::Boolean(vec[index]))
//                 } else {
//                     None
//                 }
//             }
//             ContextType::StrArr(vec) => {
//                 if self.index < vec.len() {
//                     let index = self.index;
//                     self.index += 1;
//                     // TODO: check if it is ok to clone here
//                     Some(ContextType::Str(vec[index].clone()))
//                 } else {
//                     None
//                 }
//             }
//             ContextType::DoubleArr(vec) => {
//                 if self.index < vec.len() {
//                     let index = self.index;
//                     self.index += 1;
//                     Some(ContextType::Double(vec[index]))
//                 } else {
//                     None
//                 }
//             }
//             ContextType::NvListArr(_vec) => todo!(),
//             _ => None,
//         }
//     }
// }

// #[derive(Clone, Debug, PartialEq)]
// pub enum ContextType {
//     U8(u8),
//     I8(i8),
//     U16(u16),
//     I16(i16),
//     U32(u32),
//     I32(i32),
//     U64(u64),
//     I64(i64),
//     Boolean(bool),
//     Str(String),
//     Double(f64),
//     NvList(NvList),
//     U8Arr(Vec<u8>),
//     U16Arr(Vec<u16>),
//     U32Arr(Vec<u32>),
//     U64Arr(Vec<u64>),
//     I8Arr(Vec<i8>),
//     I16Arr(Vec<i16>),
//     I32Arr(Vec<i32>),
//     I64Arr(Vec<i64>),
//     BooleanArr(Vec<bool>),
//     StrArr(Vec<String>),
//     DoubleArr(Vec<f64>),
//     NvListArr(Vec<NvList>),
//     Empty,
// }

// #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
// pub enum NvPairType {
//     Dontcare,
//     Unknown,
//     Boolean,
//     Byte,
//     Int16,
//     Uint16,
//     Int32,
//     Uint32,
//     Int64,
//     Uint64,
//     String,
//     ByteArray,
//     Int16Array,
//     Uint16Array,
//     Int32Array,
//     Uint32Array,
//     Int64Array,
//     Uint64Array,
//     StringArray,
//     Hrtime,
//     Nvlist,
//     NvlistArray,
//     BooleanValue,
//     Int8,
//     Uint8,
//     BooleanArray,
//     Int8Array,
//     Uint8Array,
//     Double,
// }

// impl From<sys::data_type_t> for NvPairType {
//     fn from(mtype: sys::data_type_t) -> Self {
//         match mtype {
//             sys::data_type_t::DATA_TYPE_DONTCARE => Self::Dontcare,
//             sys::data_type_t::DATA_TYPE_UNKNOWN => Self::Unknown,
//             sys::data_type_t::DATA_TYPE_BOOLEAN => Self::Boolean,
//             sys::data_type_t::DATA_TYPE_BYTE => Self::Byte,
//             sys::data_type_t::DATA_TYPE_INT16 => Self::Int16,
//             sys::data_type_t::DATA_TYPE_UINT16 => Self::Uint16,
//             sys::data_type_t::DATA_TYPE_INT32 => Self::Int32,
//             sys::data_type_t::DATA_TYPE_UINT32 => Self::Uint32,
//             sys::data_type_t::DATA_TYPE_INT64 => Self::Int64,
//             sys::data_type_t::DATA_TYPE_UINT64 => Self::Uint64,
//             sys::data_type_t::DATA_TYPE_STRING => Self::String,
//             sys::data_type_t::DATA_TYPE_BYTE_ARRAY => Self::ByteArray,
//             sys::data_type_t::DATA_TYPE_INT16_ARRAY => Self::Int16Array,
//             sys::data_type_t::DATA_TYPE_UINT16_ARRAY => Self::Uint16Array,
//             sys::data_type_t::DATA_TYPE_INT32_ARRAY => Self::Int32Array,
//             sys::data_type_t::DATA_TYPE_UINT32_ARRAY => Self::Uint32Array,
//             sys::data_type_t::DATA_TYPE_INT64_ARRAY => Self::Int64Array,
//             sys::data_type_t::DATA_TYPE_UINT64_ARRAY => Self::Uint64Array,
//             sys::data_type_t::DATA_TYPE_STRING_ARRAY => Self::StringArray,
//             sys::data_type_t::DATA_TYPE_HRTIME => Self::Hrtime,
//             sys::data_type_t::DATA_TYPE_NVLIST => Self::Nvlist,
//             sys::data_type_t::DATA_TYPE_NVLIST_ARRAY => Self::NvlistArray,
//             sys::data_type_t::DATA_TYPE_BOOLEAN_VALUE => Self::BooleanValue,
//             sys::data_type_t::DATA_TYPE_INT8 => Self::Int8,
//             sys::data_type_t::DATA_TYPE_UINT8 => Self::Uint8,
//             sys::data_type_t::DATA_TYPE_BOOLEAN_ARRAY => Self::BooleanArray,
//             sys::data_type_t::DATA_TYPE_INT8_ARRAY => Self::Int8Array,
//             sys::data_type_t::DATA_TYPE_UINT8_ARRAY => Self::Uint8Array,
//             sys::data_type_t::DATA_TYPE_DOUBLE => Self::Double,
//             _ => Self::Unknown,
//         }
//     }
// }

// pub trait SafeNvPair {}

// impl SafeNvPair for u8 {}
// impl SafeNvPair for u16 {}
// impl SafeNvPair for u32 {}
// impl SafeNvPair for u64 {}
// impl SafeNvPair for i8 {}
// impl SafeNvPair for i16 {}
// impl SafeNvPair for i32 {}
// impl SafeNvPair for i64 {}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::convert::TryInto;

//     #[test]
//     fn basic_iter_u8() {
//         let mut nvlist = NvList::new(NvFlag::UniqueName).unwrap();
//         let arr: [u8; 5] = [1, 2, 3, 4, 5];
//         nvlist.add_uint8_arr("d", arr).unwrap();
//         let nvpair =
//             NvPair::from(unsafe { sys::nvlist_next_nvpair(nvlist.raw, std::ptr::null_mut()) });
//         let mut iter: CtxIter<ContextType> = nvpair.try_into().unwrap();
//         assert_eq!(Some(ContextType::U8(1)), iter.next());
//         assert_eq!(Some(ContextType::U8(2)), iter.next());
//         assert_eq!(Some(ContextType::U8(3)), iter.next());
//         assert_eq!(Some(ContextType::U8(4)), iter.next());
//         assert_eq!(Some(ContextType::U8(5)), iter.next());
//         assert_eq!(None, iter.next());
//         assert_eq!(None, iter.next());
//         assert_eq!(None, iter.next());
//     }

//     #[test]
//     fn basic_iter_u16() {
//         let mut nvlist = NvList::new(NvFlag::UniqueName).unwrap();
//         let arr: [u16; 5] = [1, 2, 3, 4, 5];
//         nvlist.add_uint16_arr("d", arr).unwrap();
//         let nvpair =
//             NvPair::from(unsafe { sys::nvlist_next_nvpair(nvlist.raw, std::ptr::null_mut()) });
//         let mut iter: CtxIter<ContextType> = nvpair.try_into().unwrap();
//         assert_eq!(Some(ContextType::U16(1)), iter.next());
//         assert_eq!(Some(ContextType::U16(2)), iter.next());
//         assert_eq!(Some(ContextType::U16(3)), iter.next());
//         assert_eq!(Some(ContextType::U16(4)), iter.next());
//         assert_eq!(Some(ContextType::U16(5)), iter.next());
//         assert_eq!(None, iter.next());
//         assert_eq!(None, iter.next());
//         assert_eq!(None, iter.next());
//     }

//     #[test]
//     fn basic_iter_u32() {
//         let mut nvlist = NvList::new(NvFlag::UniqueName).unwrap();
//         let arr: [u32; 5] = [1, 2, 3, 4, 5];
//         nvlist.add_uint32_arr("d", arr).unwrap();
//         let nvpair =
//             NvPair::from(unsafe { sys::nvlist_next_nvpair(nvlist.raw, std::ptr::null_mut()) });
//         let mut iter: CtxIter<ContextType> = nvpair.try_into().unwrap();
//         assert_eq!(Some(ContextType::U32(1)), iter.next());
//         assert_eq!(Some(ContextType::U32(2)), iter.next());
//         assert_eq!(Some(ContextType::U32(3)), iter.next());
//         assert_eq!(Some(ContextType::U32(4)), iter.next());
//         assert_eq!(Some(ContextType::U32(5)), iter.next());
//         assert_eq!(None, iter.next());
//         assert_eq!(None, iter.next());
//         assert_eq!(None, iter.next());
//     }

//     #[test]
//     fn basic_iter_u64() {
//         let mut nvlist = NvList::new(NvFlag::UniqueName).unwrap();
//         let arr: [u64; 5] = [1, 2, 3, 4, 5];
//         nvlist.add_uint64_arr("d", arr).unwrap();
//         let nvpair =
//             NvPair::from(unsafe { sys::nvlist_next_nvpair(nvlist.raw, std::ptr::null_mut()) });
//         let mut iter: CtxIter<ContextType> = nvpair.try_into().unwrap();
//         assert_eq!(Some(ContextType::U64(1)), iter.next());
//         assert_eq!(Some(ContextType::U64(2)), iter.next());
//         assert_eq!(Some(ContextType::U64(3)), iter.next());
//         assert_eq!(Some(ContextType::U64(4)), iter.next());
//         assert_eq!(Some(ContextType::U64(5)), iter.next());
//         assert_eq!(None, iter.next());
//         assert_eq!(None, iter.next());
//         assert_eq!(None, iter.next());
//     }
// }
