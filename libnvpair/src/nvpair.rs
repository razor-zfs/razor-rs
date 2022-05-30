use std::mem;

use libc::{c_char, c_uchar};

use super::*;

macro_rules! nvpair_value {
    ($accessor:ident, $output:ty) => {
        #[inline]
        pub unsafe fn $accessor(nvp: *mut nvpair_t) -> Result<$output, NvPairTypeMismatch> {
            let mut value = mem::MaybeUninit::uninit();
            match sys::$accessor(nvp, value.as_mut_ptr()) {
                0 => Ok(value.assume_init()),
                libc::EINVAL => Err(NvPairTypeMismatch),
                other => panic!("Impossible return value '{other}' from nvpair_value accessor"),
            }
        }
    };
}

nvpair_value!(nvpair_value_boolean_value, boolean_t);
nvpair_value!(nvpair_value_byte, c_uchar);
nvpair_value!(nvpair_value_int8, i8);
nvpair_value!(nvpair_value_uint8, u8);
nvpair_value!(nvpair_value_int16, i16);
nvpair_value!(nvpair_value_uint16, u16);
nvpair_value!(nvpair_value_int32, i32);
nvpair_value!(nvpair_value_uint32, u32);
nvpair_value!(nvpair_value_int64, i64);
nvpair_value!(nvpair_value_uint64, u64);
nvpair_value!(nvpair_value_double, f64);
nvpair_value!(nvpair_value_string, *mut c_char);
nvpair_value!(nvpair_value_nvlist, *mut nvlist_t);

macro_rules! nvpair_value_array {
    ($accessor:ident, $output:ty) => {
        #[inline]
        pub unsafe fn $accessor(nvp: *mut nvpair_t) -> Result<($output, u32), NvPairTypeMismatch> {
            let mut len = 0;
            let mut value = mem::MaybeUninit::uninit();
            match sys::$accessor(nvp, value.as_mut_ptr(), &mut len) {
                0 => Ok((value.assume_init(), len)),
                libc::EINVAL => Err(NvPairTypeMismatch),
                other => panic!("Impossible return value '{other}' from nvpair_value accessor"),
            }
        }
    };
}

nvpair_value_array!(nvpair_value_byte_array, *mut c_uchar);
nvpair_value_array!(nvpair_value_boolean_array, *mut boolean_t);
nvpair_value_array!(nvpair_value_int8_array, *mut i8);
nvpair_value_array!(nvpair_value_uint8_array, *mut u8);
nvpair_value_array!(nvpair_value_int16_array, *mut i16);
nvpair_value_array!(nvpair_value_uint16_array, *mut u16);
nvpair_value_array!(nvpair_value_int32_array, *mut i32);
nvpair_value_array!(nvpair_value_uint32_array, *mut u32);
nvpair_value_array!(nvpair_value_int64_array, *mut i64);
nvpair_value_array!(nvpair_value_uint64_array, *mut u64);
nvpair_value_array!(nvpair_value_string_array, *mut *mut c_char);
nvpair_value_array!(nvpair_value_nvlist_array, *mut *mut nvlist_t);
