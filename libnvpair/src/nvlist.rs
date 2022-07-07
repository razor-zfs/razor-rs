use std::mem;

use libc::{c_char, c_int, c_uchar, c_uint, size_t};

use super::*;

const RESERVED_FLAG_0: c_int = 0;

#[inline]
pub unsafe fn nvlist_alloc(flag: u32) -> Result<*mut nvlist_t, NvListError> {
    let mut nvl = mem::MaybeUninit::uninit();
    match sys::nvlist_alloc(nvl.as_mut_ptr(), flag, RESERVED_FLAG_0) {
        0 => Ok(nvl.assume_init()),
        libc::EINVAL => Err(NvListError::InvalidArgument),
        libc::ENOMEM => Err(NvListError::OutOfMemory),
        other => panic!("Impossible return value '{other}' from 'nvlist_alloc()'"),
    }
}

#[inline]
pub unsafe fn nvlist_size(nvl: *mut nvlist_t, encoding: i32) -> Result<size_t, NvListError> {
    let mut size = mem::MaybeUninit::uninit();
    match sys::nvlist_size(nvl, size.as_mut_ptr(), encoding) {
        0 => Ok(size.assume_init()),
        libc::EINVAL => Err(NvListError::InvalidArgument),
        libc::ENOMEM => Err(NvListError::OutOfMemory),
        other => panic!("Impossible return value '{other}' from 'nvlist_size()'"),
    }
}

#[inline]
pub unsafe fn nvlist_dup(nvl: *mut nvlist_t) -> Result<*mut nvlist_t, NvListError> {
    let mut dup = mem::MaybeUninit::uninit();
    match sys::nvlist_dup(nvl, dup.as_mut_ptr(), RESERVED_FLAG_0) {
        0 => Ok(dup.assume_init()),
        libc::EINVAL => Err(NvListError::InvalidArgument),
        libc::ENOMEM => Err(NvListError::OutOfMemory),
        other => panic!("Impossible return value '{other}' from 'nvlist_dup()'"),
    }
}

macro_rules! nvlist_lookup {
    ($lookup:ident, $output:ty) => {
        #[inline]
        pub unsafe fn $lookup(
            nvl: *mut nvlist_t,
            name: *const c_char,
        ) -> Result<$output, NvListError> {
            let mut value = mem::MaybeUninit::uninit();
            match sys::$lookup(nvl, name, value.as_mut_ptr()) {
                0 => Ok(value.assume_init()),
                libc::ENOENT => Err(NvListError::NotFound),
                libc::EINVAL => Err(NvListError::InvalidArgument),
                other => panic!(
                    "Impossible return value '{other}' from '{}()'",
                    stringify!($lookup)
                ),
            }
        }
    };
}

nvlist_lookup!(nvlist_lookup_nvpair, *mut nvpair_t);
nvlist_lookup!(nvlist_lookup_nvlist, *mut nvlist_t);
nvlist_lookup!(nvlist_lookup_boolean_value, boolean_t);
nvlist_lookup!(nvlist_lookup_byte, c_uchar);
nvlist_lookup!(nvlist_lookup_int8, i8);
nvlist_lookup!(nvlist_lookup_uint8, u8);
nvlist_lookup!(nvlist_lookup_int16, i16);
nvlist_lookup!(nvlist_lookup_uint16, u16);
nvlist_lookup!(nvlist_lookup_int32, i32);
nvlist_lookup!(nvlist_lookup_uint32, u32);
nvlist_lookup!(nvlist_lookup_int64, i64);
nvlist_lookup!(nvlist_lookup_uint64, u64);
nvlist_lookup!(nvlist_lookup_string, *const c_char);

macro_rules! nvlist_lookup_array {
    ($lookup:ident, $output:ty) => {
        #[inline]
        pub unsafe fn $lookup(
            nvl: *mut nvlist_t,
            name: *const c_char,
        ) -> Result<($output, c_uint), NvListError> {
            let mut len = 0;
            let mut value = mem::MaybeUninit::uninit();
            match sys::$lookup(nvl, name, value.as_mut_ptr(), &mut len) {
                0 => Ok((value.assume_init(), len)),
                libc::ENOENT => Err(NvListError::NotFound),
                libc::EINVAL => Err(NvListError::InvalidArgument),
                other => panic!(
                    "Impossible return value '{other}' from '{}()'",
                    stringify!($add)
                ),
            }
        }
    };
}

nvlist_lookup_array!(nvlist_lookup_boolean_array, *mut boolean_t);
nvlist_lookup_array!(nvlist_lookup_byte_array, *mut c_uchar);
nvlist_lookup_array!(nvlist_lookup_int8_array, *mut i8);
nvlist_lookup_array!(nvlist_lookup_uint8_array, *mut u8);
nvlist_lookup_array!(nvlist_lookup_int16_array, *mut i16);
nvlist_lookup_array!(nvlist_lookup_uint16_array, *mut u16);
nvlist_lookup_array!(nvlist_lookup_int32_array, *mut i32);
nvlist_lookup_array!(nvlist_lookup_uint32_array, *mut u32);
nvlist_lookup_array!(nvlist_lookup_int64_array, *mut i64);
nvlist_lookup_array!(nvlist_lookup_uint64_array, *mut u64);
nvlist_lookup_array!(nvlist_lookup_string_array, *mut *mut c_char);
nvlist_lookup_array!(nvlist_lookup_nvlist_array, *mut *mut nvlist_t);

macro_rules! nvlist_add {
    ($add:ident, $value:ty) => {
        #[inline]
        pub unsafe fn $add(
            nvl: *mut nvlist_t,
            name: *const c_char,
            value: $value,
        ) -> Result<(), NvListError> {
            match sys::$add(nvl, name, value) {
                0 => Ok(()),
                libc::EINVAL => Err(NvListError::InvalidArgument),
                libc::ENOMEM => Err(NvListError::OutOfMemory),
                other => panic!(
                    "Impossible return value '{other}' from '{}()'",
                    stringify!($add)
                ),
            }
        }
    };
}

nvlist_add!(nvlist_add_boolean_value, boolean_t);
nvlist_add!(nvlist_add_byte, c_uchar);
nvlist_add!(nvlist_add_double, f64);
nvlist_add!(nvlist_add_int8, i8);
nvlist_add!(nvlist_add_uint8, u8);
nvlist_add!(nvlist_add_int16, i16);
nvlist_add!(nvlist_add_uint16, u16);
nvlist_add!(nvlist_add_int32, i32);
nvlist_add!(nvlist_add_uint32, u32);
nvlist_add!(nvlist_add_int64, i64);
nvlist_add!(nvlist_add_uint64, u64);
nvlist_add!(nvlist_add_string, *const c_char);
nvlist_add!(nvlist_add_nvlist, *mut nvlist_t);

pub unsafe fn nvlist_add_boolean(
    nvl: *mut nvlist_t,
    name: *const c_char,
) -> Result<(), NvListError> {
    match sys::nvlist_add_boolean(nvl, name) {
        0 => Ok(()),
        libc::EINVAL => Err(NvListError::InvalidArgument),
        libc::ENOMEM => Err(NvListError::OutOfMemory),
        other => panic!("Impossible return value '{other}' from 'nvlist_add_boolean()'"),
    }
}

pub unsafe fn nvlist_add_nvpair(
    nvl: *mut nvlist_t,
    pair: *mut nvpair_t,
) -> Result<(), NvListError> {
    match sys::nvlist_add_nvpair(nvl, pair) {
        0 => Ok(()),
        libc::EINVAL => Err(NvListError::InvalidArgument),
        libc::ENOMEM => Err(NvListError::OutOfMemory),
        other => panic!("Impossible return value '{other}' from 'nvlist_add_nvpair()'"),
    }
}

macro_rules! nvlist_add_array {
    ($add:ident, $value:ty) => {
        #[inline]
        pub unsafe fn $add(
            nvl: *mut nvlist_t,
            name: *const c_char,
            value: $value,
            nelem: c_uint,
        ) -> Result<(), NvListError> {
            match sys::$add(nvl, name, value, nelem) {
                0 => Ok(()),
                libc::EINVAL => Err(NvListError::InvalidArgument),
                libc::ENOMEM => Err(NvListError::OutOfMemory),
                other => panic!(
                    "Impossible return value '{other}' from '{}()'",
                    stringify!($add)
                ),
            }
        }
    };
}

nvlist_add_array!(nvlist_add_boolean_array, *mut boolean_t);
nvlist_add_array!(nvlist_add_byte_array, *mut c_uchar);
nvlist_add_array!(nvlist_add_int8_array, *mut i8);
nvlist_add_array!(nvlist_add_uint8_array, *mut u8);
nvlist_add_array!(nvlist_add_int16_array, *mut i16);
nvlist_add_array!(nvlist_add_uint16_array, *mut u16);
nvlist_add_array!(nvlist_add_int32_array, *mut i32);
nvlist_add_array!(nvlist_add_uint32_array, *mut u32);
nvlist_add_array!(nvlist_add_int64_array, *mut i64);
nvlist_add_array!(nvlist_add_uint64_array, *mut u64);
nvlist_add_array!(nvlist_add_string_array, *const *mut c_char);
nvlist_add_array!(nvlist_add_nvlist_array, *mut *mut nvlist_t);
