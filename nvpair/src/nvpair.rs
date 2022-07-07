use std::borrow::Cow;
use std::ffi;
use std::ops;
use std::ptr;
use std::slice;

use razor_libnvpair as libnvpair;

use super::*;

/// Safe idiomatic nvpair_t wrapper.
///
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NvPair {
    nvp: *mut libnvpair::nvpair_t,
}

impl NvPair {
    #[inline]
    pub fn value(&self) -> Value {
        to_value(self)
    }

    /// Returns the name of the nvpair.
    ///
    #[inline]
    pub fn name(&self) -> Cow<'_, str> {
        let name = unsafe {
            let name = libnvpair::nvpair_name(self.nvp);
            ffi::CStr::from_ptr(name)
        };
        name.to_string_lossy()
    }

    /// Returns the type of the nvpair.
    ///
    #[inline]
    pub fn r#type(&self) -> libnvpair::data_type_t {
        unsafe { libnvpair::nvpair_type(self.nvp) }
    }

    /// Returns the boolean value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not boolean.
    ///
    #[inline]
    pub fn boolean(&self) -> libnvpair::boolean_t {
        unsafe { libnvpair::fnvpair_value_boolean_value(self.nvp) }
    }

    /// Returns the `u8` value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not byte (u8).
    ///
    #[inline]
    pub fn byte(&self) -> u8 {
        unsafe { libnvpair::fnvpair_value_byte(self.nvp) as u8 }
    }

    /// Returns the `i8` value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not int8 (i8).
    ///
    #[inline]
    pub fn int8(&self) -> i8 {
        unsafe { libnvpair::fnvpair_value_int8(self.nvp) }
    }

    /// Returns the `u8` value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not uint8 (u8).
    ///
    #[inline]
    pub fn uint8(&self) -> u8 {
        unsafe { libnvpair::fnvpair_value_uint8(self.nvp) }
    }

    /// Returns the `i16` value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not int16 (i16).
    ///
    #[inline]
    pub fn int16(&self) -> i16 {
        unsafe { libnvpair::fnvpair_value_int16(self.nvp) }
    }

    /// Returns the `u16` value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not uint16 (u16).
    ///
    #[inline]
    pub fn uint16(&self) -> u16 {
        unsafe { libnvpair::fnvpair_value_uint16(self.nvp) }
    }

    /// Returns the `i32` value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not int32 (i32).
    ///
    #[inline]
    pub fn int32(&self) -> i32 {
        unsafe { libnvpair::fnvpair_value_int32(self.nvp) }
    }

    /// Returns the `u32` value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not uint32 (u32).
    ///
    #[inline]
    pub fn uint32(&self) -> u32 {
        unsafe { libnvpair::fnvpair_value_uint32(self.nvp) }
    }

    /// Returns the `i64` value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not int64 (i64).
    ///
    #[inline]
    pub fn int64(&self) -> i64 {
        unsafe { libnvpair::fnvpair_value_int64(self.nvp) }
    }

    /// Returns the `u64` value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not uint64 (u64).
    ///
    #[inline]
    pub fn uint64(&self) -> u64 {
        unsafe { libnvpair::fnvpair_value_uint64(self.nvp) }
    }

    /// Returns the `f64` value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not double (f64).
    ///
    #[inline]
    pub fn double(&self) -> f64 {
        unsafe { libnvpair::nvpair_value_double(self.nvp).expect("NvPair type is not f64") }
    }

    /// Returns the `String` value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not string (String).
    ///
    #[inline]
    pub fn string(&self) -> Cow<'_, str> {
        let cstr = unsafe {
            let cstr = libnvpair::fnvpair_value_string(self.nvp);
            debug_assert!(!cstr.is_null());
            ffi::CStr::from_ptr(cstr)
        };
        cstr.to_string_lossy()
    }

    /// Returns the `NvListRef` value of the nvpair.
    /// The returning `NvListRef` object tracks the parent `NvPair` object lifetime
    /// and does not outlive it.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not nvlist.
    ///
    #[inline]
    pub fn nvlist(&self) -> NvListRef<'_, Self> {
        let nvl = unsafe { libnvpair::fnvpair_value_nvlist(self.nvp) };
        NvListRef::from_raw(nvl, self)
    }

    /// Returns the byte slice `[u8]` value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not byte array.
    ///
    #[inline]
    pub fn byte_array(&self) -> &[u8] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_byte_array(self.nvp)
                .expect("NvPair type is not byte array");
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    /// Returns the `[boolean_t]` slice value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not boolean array.
    ///
    #[inline]
    pub fn boolean_array(&self) -> &[libnvpair::boolean_t] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_boolean_array(self.nvp)
                .expect("NvPair type is not boolean array");
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    /// Returns the `[i8]` slice value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not int8 array.
    ///
    #[inline]
    pub fn int8_array(&self) -> &[i8] {
        unsafe {
            let (data, len) =
                libnvpair::nvpair_value_int8_array(self.nvp).expect("NvPair type is not i8 array");
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    /// Returns the `[u8]` slice value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not uint8 array.
    ///
    #[inline]
    pub fn uint8_array(&self) -> &[u8] {
        unsafe {
            let (data, len) =
                libnvpair::nvpair_value_uint8_array(self.nvp).expect("NvPair type is not u8 array");
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    /// Returns the `[i16]` slice value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not int16 array.
    ///
    #[inline]
    pub fn int16_array(&self) -> &[i16] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_int16_array(self.nvp)
                .expect("NvPair type is not i16 array");
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    /// Returns the `[u16]` slice value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not uint16 array.
    ///
    #[inline]
    pub fn uint16_array(&self) -> &[u16] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_uint16_array(self.nvp)
                .expect("NvPair type is not u16 array");
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    /// Returns the `[i32]` slice value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not int32 array.
    ///
    #[inline]
    pub fn int32_array(&self) -> &[i32] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_int32_array(self.nvp)
                .expect("NvPair type is not i32 array");
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    /// Returns the `[u32]` slice value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not uint32 array.
    ///
    #[inline]
    pub fn uint32_array(&self) -> &[u32] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_uint32_array(self.nvp)
                .expect("NvPair type is not u32 array");
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    /// Returns the `[i64]` slice value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not int64 array.
    ///
    #[inline]
    pub fn int64_array(&self) -> &[i64] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_int64_array(self.nvp)
                .expect("NvPair type is not i64 array");
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    /// Returns the `[u64]` slice value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not uint64 array.
    ///
    #[inline]
    pub fn uint64_array(&self) -> &[u64] {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_uint64_array(self.nvp)
                .expect("NvPair type is not u64 array");
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
        }
    }

    /// Returns the `Vec<Cow<'_, str>>` value of the nvpair.
    ///
    /// # Panics
    ///
    /// Panics if the type of this nvpair is not string array.
    ///
    #[inline]
    pub fn string_array(&self) -> Vec<Cow<'_, str>> {
        unsafe {
            let (data, len) = libnvpair::nvpair_value_string_array(self.nvp)
                .expect("NvPair type is not string array");
            debug_assert!(!data.is_null());
            let len = len as usize;
            slice::from_raw_parts(data, len)
                .iter()
                .map(|item| ffi::CStr::from_ptr(*item).to_string_lossy())
                .collect::<Vec<_>>()
        }
    }
}

impl NvPair {
    pub(super) fn as_ptr(nvp: Option<Self>) -> *mut libnvpair::nvpair_t {
        match nvp {
            Some(nvp) => nvp.nvp,
            None => ptr::null_mut(),
        }
    }
}

impl ops::Deref for NvPair {
    type Target = *mut libnvpair::nvpair_t;

    fn deref(&self) -> &Self::Target {
        &self.nvp
    }
}

impl From<*mut libnvpair::nvpair_t> for NvPair {
    fn from(nvp: *mut libnvpair::nvpair_t) -> Self {
        Self { nvp }
    }
}

impl AsRef<*mut libnvpair::nvpair_t> for NvPair {
    fn as_ref(&self) -> &*mut libnvpair::nvpair_t {
        &self.nvp
    }
}
