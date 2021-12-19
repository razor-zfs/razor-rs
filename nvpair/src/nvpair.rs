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
    pub fn byte(&self) -> u8 {
        unsafe { libnvpair::nvpair_value_byte(self.nvp) as u8 }
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
    pub fn double(&self) -> f64 {
        unsafe { libnvpair::nvpair_value_double(self.nvp) }
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
    pub fn byte_array(&self) -> &[u8] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_byte_array(self.nvp);
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
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
