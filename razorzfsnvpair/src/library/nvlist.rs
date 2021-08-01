use std::ffi::{CString, NulError};
use std::panic;
use std::result::Result as StdResult;

use libc::c_char;

use super::*;

#[derive(Debug, PartialEq, Copy)]
pub struct NvList {
    pub raw: Option<*mut sys::nvlist_t>,
}

impl NvList {
    pub fn new() -> Self {
        Self { raw: None }
    }

    pub fn nvlist_alloc(flag: NvFlag) -> Result<NvList> {
        let mut nvlist: *mut sys::nvlist_t = std::ptr::null_mut();
        let nvlist_ptr: *mut *mut sys::nvlist_t = &mut nvlist;

        unsafe {
            match flag {
                NvFlag::UniqueName => NvListError::from_nvlist_rc(sys::nvlist_alloc(
                    nvlist_ptr,
                    sys::NV_UNIQUE_NAME,
                    0,
                ))?,
                NvFlag::UniqueNameType => NvListError::from_nvlist_rc(sys::nvlist_alloc(
                    nvlist_ptr,
                    sys::NV_UNIQUE_NAME,
                    0,
                ))?,
            }

            Ok(NvList {
                raw: Some(*nvlist_ptr),
            })
        }
    }

    pub fn add_boolean<T>(&mut self, name: T, v: bool) -> Result<()>
    where
        T: AsRef<str>,
    {
        match self.raw {
            Some(raw) => {
                let v = if v {
                    sys::boolean_t::B_TRUE
                } else {
                    sys::boolean_t::B_FALSE
                };

                NvListError::from_nvlist_rc(unsafe {
                    sys::nvlist_add_boolean_value(raw, CString::new(name.as_ref())?.as_ptr(), v)
                })?;

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_boolean_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[bool]> + Sized,
    {
        match self.raw {
            Some(raw) => {
                let mut conversion = Vec::with_capacity(v.as_ref().len());

                for item in v.as_ref() {
                    if *item {
                        conversion.push(sys::boolean_t::B_TRUE)
                    } else {
                        conversion.push(sys::boolean_t::B_FALSE)
                    }
                }

                unsafe {
                    let length = conversion.len() as u32;
                    NvListError::from_nvlist_rc(sys::nvlist_add_boolean_array(
                        raw,
                        CString::new(name.as_ref())?.as_ptr(),
                        conversion.as_mut_ptr(),
                        length,
                    ))?;
                };

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_uint8<T>(&mut self, name: T, v: u8) -> Result<()>
    where
        T: AsRef<str>,
    {
        match self.raw {
            Some(raw) => {
                NvListError::from_nvlist_rc(unsafe {
                    sys::nvlist_add_uint8(raw, CString::new(name.as_ref())?.as_ptr(), v)
                })?;

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_uint16<T>(&mut self, name: T, v: u16) -> Result<()>
    where
        T: AsRef<str>,
    {
        match self.raw {
            Some(raw) => {
                NvListError::from_nvlist_rc(unsafe {
                    sys::nvlist_add_uint16(raw, CString::new(name.as_ref())?.as_ptr(), v)
                })?;

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_uint32<T>(&mut self, name: T, v: u32) -> Result<()>
    where
        T: AsRef<str>,
    {
        match self.raw {
            Some(raw) => {
                NvListError::from_nvlist_rc(unsafe {
                    sys::nvlist_add_uint32(raw, CString::new(name.as_ref())?.as_ptr(), v)
                })?;

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_uint64<T>(&mut self, name: T, v: u64) -> Result<()>
    where
        T: AsRef<str>,
    {
        match self.raw {
            Some(raw) => {
                NvListError::from_nvlist_rc(unsafe {
                    sys::nvlist_add_uint64(raw, CString::new(name.as_ref())?.as_ptr(), v)
                })?;

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_int8<T>(&mut self, name: T, v: i8) -> Result<()>
    where
        T: AsRef<str>,
    {
        match self.raw {
            Some(raw) => {
                NvListError::from_nvlist_rc(unsafe {
                    sys::nvlist_add_int8(raw, CString::new(name.as_ref())?.as_ptr(), v)
                })?;

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_int16<T>(&mut self, name: T, v: i16) -> Result<()>
    where
        T: AsRef<str>,
    {
        match self.raw {
            Some(raw) => {
                NvListError::from_nvlist_rc(unsafe {
                    sys::nvlist_add_int16(raw, CString::new(name.as_ref())?.as_ptr(), v)
                })?;

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_int32<T>(&mut self, name: T, v: i32) -> Result<()>
    where
        T: AsRef<str>,
    {
        match self.raw {
            Some(raw) => {
                NvListError::from_nvlist_rc(unsafe {
                    sys::nvlist_add_int32(raw, CString::new(name.as_ref())?.as_ptr(), v)
                })?;

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_int64<T>(&mut self, name: T, v: i64) -> Result<()>
    where
        T: AsRef<str>,
    {
        match self.raw {
            Some(raw) => {
                NvListError::from_nvlist_rc(unsafe {
                    sys::nvlist_add_int64(raw, CString::new(name.as_ref())?.as_ptr(), v)
                })?;

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_uint8_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[u8]> + Sized,
    {
        match self.raw {
            Some(raw) => {
                unsafe {
                    let length = v.as_ref().len() as u32;
                    let arr = v.as_ref().as_ptr() as *mut u8;
                    NvListError::from_nvlist_rc(sys::nvlist_add_uint8_array(
                        raw,
                        CString::new(name.as_ref())?.as_ptr(),
                        arr,
                        length,
                    ))?;
                };

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_uint16_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[u16]> + Sized,
    {
        match self.raw {
            Some(raw) => {
                unsafe {
                    let length = v.as_ref().len() as u32;
                    let arr = v.as_ref().as_ptr() as *mut u16;
                    NvListError::from_nvlist_rc(sys::nvlist_add_uint16_array(
                        raw,
                        CString::new(name.as_ref())?.as_ptr(),
                        arr,
                        length,
                    ))?;
                };

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_uint32_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[u32]> + Sized,
    {
        match self.raw {
            Some(raw) => {
                unsafe {
                    let length = v.as_ref().len() as u32;
                    let arr = v.as_ref().as_ptr() as *mut u32;
                    NvListError::from_nvlist_rc(sys::nvlist_add_uint32_array(
                        raw,
                        CString::new(name.as_ref())?.as_ptr(),
                        arr,
                        length,
                    ))?;
                };

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_uint64_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[u64]> + Sized,
    {
        match self.raw {
            Some(raw) => {
                unsafe {
                    let length = v.as_ref().len() as u32;
                    let arr = v.as_ref().as_ptr() as *mut u64;
                    NvListError::from_nvlist_rc(sys::nvlist_add_uint64_array(
                        raw,
                        CString::new(name.as_ref())?.as_ptr(),
                        arr,
                        length,
                    ))?;
                };

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_int8_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[i8]> + Sized,
    {
        match self.raw {
            Some(raw) => {
                unsafe {
                    let length = v.as_ref().len() as u32;
                    let arr = v.as_ref().as_ptr() as *mut i8;
                    NvListError::from_nvlist_rc(sys::nvlist_add_int8_array(
                        raw,
                        CString::new(name.as_ref())?.as_ptr(),
                        arr,
                        length,
                    ))?;
                };

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_int16_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[i16]> + Sized,
    {
        match self.raw {
            Some(raw) => {
                unsafe {
                    let length = v.as_ref().len() as u32;
                    let arr = v.as_ref().as_ptr() as *mut i16;
                    NvListError::from_nvlist_rc(sys::nvlist_add_int16_array(
                        raw,
                        CString::new(name.as_ref())?.as_ptr(),
                        arr,
                        length,
                    ))?;
                };

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_int32_arr<T, W>(&mut self, name: T, v: &W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[i32]> + Sized,
    {
        match self.raw {
            Some(raw) => {
                unsafe {
                    let length = v.as_ref().len() as u32;
                    let arr = v.as_ref().as_ptr() as *mut i32;
                    NvListError::from_nvlist_rc(sys::nvlist_add_int32_array(
                        raw,
                        CString::new(name.as_ref())?.as_ptr(),
                        arr,
                        length,
                    ))?;
                };

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_int64_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[i64]> + Sized,
    {
        match self.raw {
            Some(raw) => {
                unsafe {
                    let length = v.as_ref().len() as u32;
                    let arr = v.as_ref().as_ptr() as *mut i64;
                    NvListError::from_nvlist_rc(sys::nvlist_add_int64_array(
                        raw,
                        CString::new(name.as_ref())?.as_ptr(),
                        arr,
                        length,
                    ))?;
                };

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_float64<T>(&mut self, name: T, v: f64) -> Result<()>
    where
        T: AsRef<str>,
    {
        match self.raw {
            Some(raw) => {
                NvListError::from_nvlist_rc(unsafe {
                    sys::nvlist_add_double(raw, CString::new(name.as_ref())?.as_ptr(), v)
                })?;

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_string<T>(&mut self, name: T, v: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        match self.raw {
            Some(raw) => {
                NvListError::from_nvlist_rc(unsafe {
                    sys::nvlist_add_string(
                        raw,
                        CString::new(name.as_ref())?.as_ptr(),
                        CString::new(v.as_ref())?.as_ptr(),
                    )
                })?;

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_string_arr<T, W, S>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[S]> + Sized,
        S: AsRef<str>,
    {
        match self.raw {
            Some(raw) => {
                let cstrings = v
                    .as_ref()
                    .iter()
                    .map(|x| x.as_ref())
                    .map(CString::new)
                    .collect::<StdResult<Vec<_>, NulError>>()?;

                let converted = cstrings
                    .iter()
                    .map(|item| item.as_ptr() as *mut c_char)
                    .collect::<Vec<_>>();

                let x = converted.as_ptr();
                let len = converted.len() as u32;

                unsafe {
                    NvListError::from_nvlist_rc(sys::nvlist_add_string_array(
                        raw,
                        CString::new(name.as_ref())?.as_ptr(),
                        x,
                        len,
                    ))?;
                };

                Ok(())
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn add_nvlist<T>(&mut self, name: T, v: &NvList) -> Result<()>
    where
        T: AsRef<str>,
    {
        match self.raw {
            Some(raw) => {
                if let Some(raw_value) = v.raw {
                    NvListError::from_nvlist_rc(unsafe {
                        sys::nvlist_add_nvlist(
                            raw,
                            CString::new(name.as_ref())?.as_ptr(),
                            raw_value,
                        )
                    })?;

                    Ok(())
                } else {
                    Err(NvListError::NvListNullPointer)
                }
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }

    pub fn lookup_nvpair<T>(&self, name: T) -> Result<NvPair>
    where
        T: AsRef<str>,
    {
        match self.raw {
            Some(raw) => {
                let mut nvpair: *mut sys::nvpair_t = std::ptr::null_mut();
                let nvpair_ptr: *mut *mut sys::nvpair_t = &mut nvpair;

                unsafe {
                    NvListError::from_nvlist_rc(sys::nvlist_lookup_nvpair(
                        raw,
                        CString::new(name.as_ref())?.as_ptr(),
                        nvpair_ptr,
                    ))?;

                    let nvpair = NvPair::from(*nvpair_ptr);

                    Ok(nvpair)
                }
            }
            None => Err(NvListError::NvListNullPointer),
        }
    }
}

impl From<*mut sys::nvlist_t> for NvList {
    fn from(nvl: *mut sys::nvlist_t) -> Self {
        Self { raw: Some(nvl) }
    }
}

impl Clone for NvList {
    fn clone(&self) -> NvList {
        match self.raw {
            Some(raw) => {
                let mut cloned_nvlist: *mut sys::nvlist_t = std::ptr::null_mut();
                let cloned_nvlist_ptr: *mut *mut sys::nvlist_t = &mut cloned_nvlist;
                let rc = unsafe { sys::nvlist_dup(raw, cloned_nvlist_ptr, 0) };

                if rc == libc::EINVAL {
                    panic!("Nvlist clone invalid argument");
                } else if rc == libc::ENOMEM {
                    panic!("Nvlist clone insufficient memory");
                }

                unsafe { NvList::from(*cloned_nvlist_ptr) }
            }
            None => panic!("NvList clone null pointer"),
        }
    }
}

impl IntoIterator for NvList {
    type Item = NvPair;
    type IntoIter = NvListIterator;

    fn into_iter(self) -> Self::IntoIter {
        NvListIterator {
            nvlist: self,
            nvp: None,
            completed: false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct NvListIterator {
    nvlist: NvList,
    nvp: Option<*mut sys::nvpair_t>,
    completed: bool,
}

impl Iterator for NvListIterator {
    type Item = NvPair;

    fn next(&mut self) -> Option<Self::Item> {
        let current = if let Some(nvp) = self.nvp {
            nvp
        } else {
            std::ptr::null_mut()
        };

        match self.nvlist.raw {
            Some(raw) => {
                let nvp = unsafe { sys::nvlist_next_nvpair(raw, current) };
                if nvp.is_null() {
                    self.nvp = None;
                } else {
                    self.nvp = Some(nvp);
                }

                self.nvp.map(NvPair::from)
            }
            None => None,
        }
    }
}

pub enum NvFlag {
    UniqueName,
    UniqueNameType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nvlist_iter() {
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        let arr: [u8; 5] = [1, 2, 3, 4, 5];
        nvlist.add_uint16("a", 3).unwrap();
        nvlist.add_uint32("b", 5).unwrap();
        nvlist.add_uint8_arr("d", arr).unwrap();
        let mut iter = nvlist.into_iter();
        let pair1 = iter.next().unwrap();
        let pair2 = iter.next().unwrap();
        let pair3 = iter.next().unwrap();
        assert_eq!("a".to_string(), pair1.name().unwrap());
        assert_eq!(NvPairType::Uint16, pair1.r#type().unwrap());
        assert_eq!("b".to_string(), pair2.name().unwrap());
        assert_eq!(NvPairType::Uint32, pair2.r#type().unwrap());
        assert_eq!("d".to_string(), pair3.name().unwrap());
        assert_eq!(NvPairType::Uint8Array, pair3.r#type().unwrap());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}
