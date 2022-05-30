use std::mem;

use libc::{c_char, c_int, c_uchar, size_t};

use super::*;

const RESERVED_FLAG_0: c_int = 0;

#[inline]
pub unsafe fn nvlist_alloc(flag: u32) -> Result<*mut nvlist_t, NvListError> {
    let mut nvl = mem::MaybeUninit::uninit();
    match sys::nvlist_alloc(nvl.as_mut_ptr(), flag, RESERVED_FLAG_0) {
        0 => Ok(nvl.assume_init()),
        libc::EINVAL => Err(NvListError::InvalidArgument),
        libc::ENOMEM => Err(NvListError::OutOfMemory),
        other => panic!("Impossible return value '{other}' from nvlist_xxx"),
    }
}

#[inline]
pub unsafe fn nvlist_size(nvl: *mut nvlist_t, encoding: i32) -> Result<size_t, NvListError> {
    let mut size = mem::MaybeUninit::uninit();
    match sys::nvlist_size(nvl, size.as_mut_ptr(), encoding) {
        0 => Ok(size.assume_init()),
        libc::EINVAL => Err(NvListError::InvalidArgument),
        libc::ENOMEM => Err(NvListError::OutOfMemory),
        other => panic!("Impossible return value '{other}' from nvlist_xxx"),
    }
}

#[inline]
pub unsafe fn nvlist_dup(nvl: *mut nvlist_t) -> Result<*mut nvlist_t, NvListError> {
    let mut dup = mem::MaybeUninit::uninit();
    match sys::nvlist_dup(nvl, dup.as_mut_ptr(), RESERVED_FLAG_0) {
        0 => Ok(dup.assume_init()),
        libc::EINVAL => Err(NvListError::InvalidArgument),
        libc::ENOMEM => Err(NvListError::OutOfMemory),
        other => panic!("Impossible return value '{other}' from nvlist_xxx"),
    }
}

macro_rules! nvlist_lookup {
    ($lookup:ident, $output:ty) => {
        #[inline]
        pub unsafe fn $lookup(
            nvl: *mut nvlist_t,
            name: *const c_char,
        ) -> Result<$output, NvListLookupError> {
            let mut value = mem::MaybeUninit::uninit();
            match sys::$lookup(nvl, name, value.as_mut_ptr()) {
                0 => Ok(value.assume_init()),
                libc::EINVAL => Err(NvListLookupError::InvalidArgument),
                libc::ENOENT => Err(NvListLookupError::NoSuchNvPair),
                other => panic!("Impossible return value '{other}' from nvlist_lookup_xxx"),
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
        ) -> Result<($output, u32), NvListLookupError> {
            let mut len = 0;
            let mut value = mem::MaybeUninit::uninit();
            match sys::$lookup(nvl, name, value.as_mut_ptr(), &mut len) {
                0 => Ok((value.assume_init(), len)),
                libc::EINVAL => Err(NvListLookupError::InvalidArgument),
                libc::ENOENT => Err(NvListLookupError::NoSuchNvPair),
                other => panic!("Impossible return value '{other}' from nvlist_lookup_xxx_array"),
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
