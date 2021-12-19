use std::mem;

use libc::{c_char, c_uchar};

use super::*;

#[inline]
pub unsafe fn nvpair_value_boolean_value(nvp: *mut nvpair_t) -> boolean_t {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_boolean_value(nvp, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvpair_value_int8(nvp: *mut nvpair_t) -> i8 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_int8(nvp, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvpair_value_uint8(nvp: *mut nvpair_t) -> u8 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_uint8(nvp, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvpair_value_int16(nvp: *mut nvpair_t) -> i16 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_int16(nvp, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvpair_value_uint16(nvp: *mut nvpair_t) -> u16 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_uint16(nvp, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvpair_value_int32(nvp: *mut nvpair_t) -> i32 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_int32(nvp, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvpair_value_uint32(nvp: *mut nvpair_t) -> u32 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_uint32(nvp, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvpair_value_int64(nvp: *mut nvpair_t) -> i64 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_int64(nvp, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvpair_value_uint64(nvp: *mut nvpair_t) -> u64 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_uint64(nvp, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvpair_value_double(nvp: *mut nvpair_t) -> f64 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_double(nvp, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvpair_value_string(nvp: *mut nvpair_t) -> *mut c_char {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_string(nvp, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvpair_value_nvlist(nvp: *mut nvpair_t) -> *mut nvlist_t {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_nvlist(nvp, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvpair_value_byte(nvp: *mut nvpair_t) -> c_uchar {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_byte(nvp, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvpair_value_byte_array(nvp: *mut nvpair_t) -> (*mut c_uchar, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_byte_array(nvp, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvpair_value_boolean_array(nvp: *mut nvpair_t) -> (*mut boolean_t, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_boolean_array(nvp, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvpair_value_int8_array(nvp: *mut nvpair_t) -> (*mut i8, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_int8_array(nvp, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvpair_value_uint8_array(nvp: *mut nvpair_t) -> (*mut u8, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_uint8_array(nvp, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvpair_value_int16_array(nvp: *mut nvpair_t) -> (*mut i16, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_int16_array(nvp, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvpair_value_uint16_array(nvp: *mut nvpair_t) -> (*mut u16, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_uint16_array(nvp, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvpair_value_int32_array(nvp: *mut nvpair_t) -> (*mut i32, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_int32_array(nvp, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvpair_value_uint32_array(nvp: *mut nvpair_t) -> (*mut u32, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_uint32_array(nvp, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvpair_value_int64_array(nvp: *mut nvpair_t) -> (*mut i64, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_int64_array(nvp, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvpair_value_uint64_array(nvp: *mut nvpair_t) -> (*mut u64, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_uint64_array(nvp, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvpair_value_string_array(nvp: *mut nvpair_t) -> (*mut *mut c_char, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_string_array(nvp, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvpair_value_nvlist_array(nvp: *mut nvpair_t) -> (*mut *mut nvlist_t, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvpair_value_nvlist_array(nvp, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}
