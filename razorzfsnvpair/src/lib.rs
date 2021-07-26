use std::ffi::CStr;
use std::ffi::CString;
use std::slice;

pub use razorzfsnvpair_sys as sys;

pub use error::NvListError;
pub use library::ContextType;
pub use library::CtxIter;
pub use library::NvFlag;
pub use library::NvList;
pub use library::NvListIterator;
pub use library::NvPair;
pub use library::NvPairType;
pub use library::SafeNvPair;
use sys::nvpair_t;

mod error;
mod library;

pub type Result<T> = std::result::Result<T, NvListError>;

/*pub fn nvlist_alloc(flag: NvFlag) -> Result<NvList> {
    let mut nvlist: *mut sys::nvlist_t = std::ptr::null_mut();
    let nvlist_ptr: *mut *mut sys::nvlist_t = &mut nvlist;

    unsafe {
        match flag {
            NvFlag::UniqueName => {
                NvListError::from_nvlist_rc(sys::nvlist_alloc(nvlist_ptr, sys::NV_UNIQUE_NAME, 0))?
            }
            NvFlag::UniqueNameType => {
                NvListError::from_nvlist_rc(sys::nvlist_alloc(nvlist_ptr, sys::NV_UNIQUE_NAME, 0))?
            }
        }

        Ok(NvList { raw: *nvlist_ptr })
    }
}*/

// pub fn nvlist_add_boolean<T>(nvlist: &NvList, name: T, v: bool) -> Result<()>
// where
//     T: AsRef<str>,
// {
//     let v = if v {
//         sys::boolean_t::B_TRUE
//     } else {
//         sys::boolean_t::B_FALSE
//     };

//     NvListError::from_nvlist_rc(unsafe {
//         sys::nvlist_add_boolean_value(nvlist.raw, CString::new(name.as_ref())?.as_ptr(), v)
//     })?;

//     Ok(())
// }

// pub fn nvlist_add_uint8<T>(nvlist: &NvList, name: T, v: u8) -> Result<()>
// where
//     T: AsRef<str>,
// {
//     NvListError::from_nvlist_rc(unsafe {
//         sys::nvlist_add_uint8(nvlist.raw, CString::new(name.as_ref())?.as_ptr(), v)
//     })?;

//     Ok(())
// }

// pub fn nvlist_add_uint16<T>(nvlist: &NvList, name: T, v: u16) -> Result<()>
// where
//     T: AsRef<str>,
// {
//     NvListError::from_nvlist_rc(unsafe {
//         sys::nvlist_add_uint16(nvlist.raw, CString::new(name.as_ref())?.as_ptr(), v)
//     })?;

//     Ok(())
// }

// pub fn nvlist_add_uint8_arr<T, W>(nvlist: &NvList, name: T, v: W) -> Result<()>
// where
//     T: AsRef<str>,
//     W: AsRef<[u8]> + Sized,
// {
//     unsafe {
//         NvListError::from_nvlist_rc(sys::nvlist_add_uint8_array(
//             nvlist.raw,
//             CString::new(name.as_ref())?.as_ptr(),
//             v.as_ref().to_owned().as_mut_ptr(),
//             v.as_ref().len() as u32,
//         ))?;
//     };

//     Ok(())
// }

// pub fn nvlist_add_uint16_arr<T, W>(nvlist: &NvList, name: T, v: W) -> Result<()>
// where
//     T: AsRef<str>,
//     W: AsRef<[u16]> + Sized,
// {
//     unsafe {
//         NvListError::from_nvlist_rc(sys::nvlist_add_uint16_array(
//             nvlist.raw,
//             CString::new(name.as_ref())?.as_ptr(),
//             v.as_ref().to_owned().as_mut_ptr(),
//             v.as_ref().len() as u32,
//         ))?;
//     };

//     Ok(())
// }

// pub fn nvlist_add_uint32_arr<T, W>(nvlist: &NvList, name: T, v: W) -> Result<()>
// where
//     T: AsRef<str>,
//     W: AsRef<[u32]> + Sized,
// {
//     unsafe {
//         NvListError::from_nvlist_rc(sys::nvlist_add_uint32_array(
//             nvlist.raw,
//             CString::new(name.as_ref())?.as_ptr(),
//             v.as_ref().to_owned().as_mut_ptr(),
//             v.as_ref().len() as u32,
//         ))?;
//     };

//     Ok(())
// }

// pub fn nvlist_add_uint64_arr<T, W>(nvlist: &NvList, name: T, v: W) -> Result<()>
// where
//     T: AsRef<str>,
//     W: AsRef<[u64]> + Sized,
// {
//     unsafe {
//         NvListError::from_nvlist_rc(sys::nvlist_add_uint64_array(
//             nvlist.raw,
//             CString::new(name.as_ref())?.as_ptr(),
//             v.as_ref().to_owned().as_mut_ptr(),
//             v.as_ref().len() as u32,
//         ))?;
//     };

//     Ok(())
// }

// pub fn nvlist_add_uint32<T>(nvlist: &NvList, name: T, v: u32) -> Result<()>
// where
//     T: AsRef<str>,
// {
//     NvListError::from_nvlist_rc(unsafe {
//         sys::nvlist_add_uint32(nvlist.raw, CString::new(name.as_ref())?.as_ptr(), v)
//     })?;

//     Ok(())
// }

// pub fn nvlist_add_uint64<T>(nvlist: &NvList, name: T, v: u64) -> Result<()>
// where
//     T: AsRef<str>,
// {
//     NvListError::from_nvlist_rc(unsafe {
//         sys::nvlist_add_uint64(nvlist.raw, CString::new(name.as_ref())?.as_ptr(), v)
//     })?;

//     Ok(())
// }

// pub fn nvlist_add_int8<T>(nvlist: &NvList, name: T, v: i8) -> Result<()>
// where
//     T: AsRef<str>,
// {
//     NvListError::from_nvlist_rc(unsafe {
//         sys::nvlist_add_int8(nvlist.raw, CString::new(name.as_ref())?.as_ptr(), v)
//     })?;

//     Ok(())
// }

// pub fn nvlist_add_int16<T>(nvlist: &NvList, name: T, v: i16) -> Result<()>
// where
//     T: AsRef<str>,
// {
//     NvListError::from_nvlist_rc(unsafe {
//         sys::nvlist_add_int16(nvlist.raw, CString::new(name.as_ref())?.as_ptr(), v)
//     })?;

//     Ok(())
// }

// pub fn nvlist_add_int32<T>(nvlist: &NvList, name: T, v: i32) -> Result<()>
// where
//     T: AsRef<str>,
// {
//     NvListError::from_nvlist_rc(unsafe {
//         sys::nvlist_add_int32(nvlist.raw, CString::new(name.as_ref())?.as_ptr(), v)
//     })?;

//     Ok(())
// }

// pub fn nvlist_add_int64<T>(nvlist: &NvList, name: T, v: i64) -> Result<()>
// where
//     T: AsRef<str>,
// {
//     NvListError::from_nvlist_rc(unsafe {
//         sys::nvlist_add_int64(nvlist.raw, CString::new(name.as_ref())?.as_ptr(), v)
//     })?;

//     Ok(())
// }

// pub fn nvlist_add_float64<T>(nvlist: &NvList, name: T, v: f64) -> Result<()>
// where
//     T: AsRef<str>,
// {
//     NvListError::from_nvlist_rc(unsafe {
//         sys::nvlist_add_double(nvlist.raw, CString::new(name.as_ref())?.as_ptr(), v)
//     })?;

//     Ok(())
// }

// pub fn nvlist_add_string<T>(nvlist: &NvList, name: T, v: T) -> Result<()>
// where
//     T: AsRef<str>,
// {
//     NvListError::from_nvlist_rc(unsafe {
//         sys::nvlist_add_string(
//             nvlist.raw,
//             CString::new(name.as_ref())?.as_ptr(),
//             CString::new(v.as_ref())?.as_ptr(),
//         )
//     })?;

//     Ok(())
// }

/*pub fn nvpair_value_uint16_array(nvpair: &mut NvPair) -> Result<()> {
    let mut size = 0;
    let size_ptr: *mut sys::uint_t = &mut size;
    let mut u16arr: *mut u16 = std::ptr::null_mut();
    let u16arr_ptr: *mut *mut u16 = &mut u16arr;
    unsafe {
        NvListError::from_nvlist_rc(sys::nvpair_value_uint16_array(
            nvpair.raw_nvpair,
            u16arr_ptr,
            size_ptr,
        ))?;

        match u16arr_ptr.as_ref() {
            Some(arr) => {
                let u16vec = slice::from_raw_parts(*arr, size as usize).to_vec();
                //nvpair.pair_value = ContextType::U16Arr(u16vec);
                //nvpair.pair_name = CStr::from_ptr(sys::nvpair_name(nvpair.raw_nvpair))
                //    .to_str()?
                //    .to_string();
                Ok(())
            }
            None => Err(NvListError::ConversionError),
        }
    }
}

pub fn nvpair_value_uint16(nvpair: &mut NvPair) -> Result<u16> {
    let mut x = 0;
    let val: *mut u16 = &mut x;

    unsafe {
        NvListError::from_nvlist_rc(sys::nvpair_value_uint16(nvpair.raw_nvpair, val))?;

        match val.as_ref() {
            Some(u16val) => {
                // nvpair.pair_name = CStr::from_ptr(sys::nvpair_name(nvpair.raw_nvpair))
                //     .to_str()?
                //     .to_string();
                // nvpair.pair_value = ContextType::U16(*u16val);
                Ok(*u16val)
            }
            None => Err(NvListError::ConversionError),
        }
    }
}

pub fn nvpair_value_uint32(nvpair: &mut NvPair) -> Result<u32> {
    let mut x = 0;
    let val: *mut u32 = &mut x;

    unsafe {
        NvListError::from_nvlist_rc(sys::nvpair_value_uint32(nvpair.raw_nvpair, val))?;
        dbg!("after func");

        match val.as_ref() {
            Some(u32val) => {
                // nvpair.pair_name = CStr::from_ptr(sys::nvpair_name(nvpair.raw_nvpair))
                //     .to_str()?
                //     .to_string();
                // nvpair.pair_value = ContextType::U32(*u32val);
                Ok(*u32val)
            }
            None => Err(NvListError::ConversionError),
        }
    }
}

pub fn nvpair_value_string(nvpair: &mut NvPair) -> Result<String> {
    let mut str: *mut u8 = std::ptr::null_mut();
    let str_ptr: *mut *mut u8 = &mut str;

    unsafe {
        NvListError::from_nvlist_rc(sys::nvpair_value_string(nvpair.raw_nvpair, str_ptr))?;
        // nvpair.pair_name = CStr::from_ptr(sys::nvpair_name(nvpair.raw_nvpair))
        //     .to_str()?
        //     .to_string();
        let name = CStr::from_ptr(*str_ptr).to_str()?.to_string();
        Ok(name)
    }
}*/

// pub fn nvlist_lookup_nvpair<T>(nvlist: &NvList, name: T) -> Result<NvPair>
// where
//     T: AsRef<str>,
// {
//     let mut nvpair: *mut nvpair_t = std::ptr::null_mut();
//     let nvpair_ptr: *mut *mut nvpair_t = &mut nvpair;

//     unsafe {
//         NvListError::from_nvlist_rc(sys::nvlist_lookup_nvpair(
//             nvlist.raw,
//             CString::new(name.as_ref())?.as_ptr(),
//             nvpair_ptr,
//         ))?;

//         let nvpair = NvPair {
//             raw_nvpair: *nvpair_ptr,
//             //pair_name: "".to_string(),
//             //pair_value: ContextType::Empty,
//         };

//         Ok(nvpair)
//     }
// }
/*
pub fn nvpair_type(nvpair: &mut NvPair) -> Result<NvPairType> {
    Ok(unsafe { NvPairType::from(sys::nvpair_type(nvpair.raw_nvpair)) })
}

pub fn nvpair_name(nvpair: &mut NvPair) -> Result<String> {
    unsafe {
        Ok(CStr::from_ptr(sys::nvpair_name(nvpair.raw_nvpair))
            .to_str()?
            .to_string())
    }
}*/
