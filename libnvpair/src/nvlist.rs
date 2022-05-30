use std::mem;

use libc::{c_char, c_int, c_uchar, size_t};

use super::*;

const RESERVED_FLAG_0: c_int = 0;

#[inline]
pub unsafe fn nvlist_alloc(flag: u32) -> *mut nvlist_t {
    let mut nvl = mem::MaybeUninit::uninit();
    sys::nvlist_alloc(nvl.as_mut_ptr(), flag, RESERVED_FLAG_0);
    nvl.assume_init()
}

#[inline]
pub unsafe fn nvlist_size(nvl: *mut nvlist_t, encoding: i32) -> size_t {
    let mut size = mem::MaybeUninit::uninit();
    sys::nvlist_size(nvl, size.as_mut_ptr(), encoding);
    size.assume_init()
}

#[inline]
pub unsafe fn nvlist_dup(nvl: *mut nvlist_t) -> anyhow::Result<*mut nvlist_t> {
    let mut dup = mem::MaybeUninit::uninit();
    match sys::nvlist_dup(nvl, dup.as_mut_ptr(), RESERVED_FLAG_0) {
        0 => Ok(dup.assume_init()),
        libc::EINVAL => anyhow::bail!("Nvlist clone invalid argument"),
        libc::ENOMEM => anyhow::bail!("Nvlist clone insufficient memory"),
        rc => anyhow::bail!("unknown error code {}", rc),
    }
}

#[inline]
pub unsafe fn nvlist_lookup_nvpair(nvl: *mut nvlist_t, name: *const c_char) -> *mut nvpair_t {
    let mut nvp = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_nvpair(nvl, name, nvp.as_mut_ptr());
    nvp.assume_init()
}

#[inline]
pub unsafe fn nvlist_lookup_boolean_value(nvl: *mut nvlist_t, name: *const c_char) -> boolean_t {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_boolean_value(nvl, name, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvlist_lookup_byte(nvl: *mut nvlist_t, name: *const c_char) -> c_uchar {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_byte(nvl, name, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvlist_lookup_int8(nvl: *mut nvlist_t, name: *const c_char) -> i8 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_int8(nvl, name, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvlist_lookup_uint8(nvl: *mut nvlist_t, name: *const c_char) -> u8 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_uint8(nvl, name, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvlist_lookup_int16(nvl: *mut nvlist_t, name: *const c_char) -> i16 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_int16(nvl, name, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvlist_lookup_uint16(nvl: *mut nvlist_t, name: *const c_char) -> u16 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_uint16(nvl, name, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvlist_lookup_int32(nvl: *mut nvlist_t, name: *const c_char) -> i32 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_int32(nvl, name, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvlist_lookup_uint32(nvl: *mut nvlist_t, name: *const c_char) -> u32 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_uint32(nvl, name, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvlist_lookup_int64(nvl: *mut nvlist_t, name: *const c_char) -> i64 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_int64(nvl, name, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvlist_lookup_uint64(nvl: *mut nvlist_t, name: *const c_char) -> u64 {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_uint64(nvl, name, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvlist_lookup_string(nvl: *mut nvlist_t, name: *const c_char) -> *const c_char {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_string(nvl, name, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvlist_lookup_nvlist(nvl: *mut nvlist_t, name: *const c_char) -> *mut nvlist_t {
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_nvlist(nvl, name, value.as_mut_ptr());
    value.assume_init()
}

#[inline]
pub unsafe fn nvlist_lookup_boolean_array(
    nvl: *mut nvlist_t,
    name: *const c_char,
) -> (*mut boolean_t, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_boolean_array(nvl, name, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvlist_lookup_byte_array(
    nvl: *mut nvlist_t,
    name: *const c_char,
) -> (*mut c_uchar, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_byte_array(nvl, name, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvlist_lookup_int8_array(nvl: *mut nvlist_t, name: *const c_char) -> (*mut i8, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_int8_array(nvl, name, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvlist_lookup_uint8_array(nvl: *mut nvlist_t, name: *const c_char) -> (*mut u8, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_uint8_array(nvl, name, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvlist_lookup_int16_array(
    nvl: *mut nvlist_t,
    name: *const c_char,
) -> (*mut i16, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_int16_array(nvl, name, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvlist_lookup_uint16_array(
    nvl: *mut nvlist_t,
    name: *const c_char,
) -> (*mut u16, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_uint16_array(nvl, name, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvlist_lookup_int32_array(
    nvl: *mut nvlist_t,
    name: *const c_char,
) -> (*mut i32, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_int32_array(nvl, name, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvlist_lookup_uint32_array(
    nvl: *mut nvlist_t,
    name: *const c_char,
) -> (*mut u32, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_uint32_array(nvl, name, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvlist_lookup_int64_array(
    nvl: *mut nvlist_t,
    name: *const c_char,
) -> (*mut i64, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_int64_array(nvl, name, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvlist_lookup_uint64_array(
    nvl: *mut nvlist_t,
    name: *const c_char,
) -> (*mut u64, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_uint64_array(nvl, name, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvlist_lookup_string_array(
    nvl: *mut nvlist_t,
    name: *const c_char,
) -> (*mut *mut c_char, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_string_array(nvl, name, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}

#[inline]
pub unsafe fn nvlist_lookup_nvlist_array(
    nvl: *mut nvlist_t,
    name: *const c_char,
) -> (*mut *mut nvlist_t, u32) {
    let mut len = 0;
    let mut value = mem::MaybeUninit::uninit();
    sys::nvlist_lookup_nvlist_array(nvl, name, value.as_mut_ptr(), &mut len);
    (value.assume_init(), len)
}
