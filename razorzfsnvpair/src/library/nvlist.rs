use std::ffi::CString;

use libc::c_char;

use super::*;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct NvList {
    pub raw: *mut sys::nvlist_t,
}

impl NvList {
    pub fn default() -> Self {
        NvList {
            raw: std::ptr::null_mut(),
        }
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

            Ok(NvList { raw: *nvlist_ptr })
        }
    }

    pub fn add_boolean<T>(&mut self, name: T, v: bool) -> Result<()>
    where
        T: AsRef<str>,
    {
        let v = if v {
            sys::boolean_t::B_TRUE
        } else {
            sys::boolean_t::B_FALSE
        };

        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_boolean_value(self.raw, CString::new(name.as_ref())?.as_ptr(), v)
        })?;

        Ok(())
    }

    pub fn add_boolean_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[bool]> + Sized,
    {
        let mut conversion = Vec::with_capacity(v.as_ref().len());

        for item in v.as_ref() {
            if *item {
                conversion.push(sys::boolean_t::B_TRUE)
            } else {
                conversion.push(sys::boolean_t::B_FALSE)
            }
        }

        unsafe {
            NvListError::from_nvlist_rc(sys::nvlist_add_boolean_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                conversion.as_mut_ptr(),
                conversion.len() as u32,
            ))?;
        };

        Ok(())
    }

    pub fn add_uint8<T>(&mut self, name: T, v: u8) -> Result<()>
    where
        T: AsRef<str>,
    {
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_uint8(self.raw, CString::new(name.as_ref())?.as_ptr(), v)
        })?;

        Ok(())
    }

    pub fn add_uint16<T>(&mut self, name: T, v: u16) -> Result<()>
    where
        T: AsRef<str>,
    {
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_uint16(self.raw, CString::new(name.as_ref())?.as_ptr(), v)
        })?;

        Ok(())
    }

    pub fn add_uint32<T>(&mut self, name: T, v: u32) -> Result<()>
    where
        T: AsRef<str>,
    {
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_uint32(self.raw, CString::new(name.as_ref())?.as_ptr(), v)
        })?;

        Ok(())
    }

    pub fn add_uint64<T>(&mut self, name: T, v: u64) -> Result<()>
    where
        T: AsRef<str>,
    {
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_uint64(self.raw, CString::new(name.as_ref())?.as_ptr(), v)
        })?;

        Ok(())
    }

    pub fn add_int8<T>(&mut self, name: T, v: i8) -> Result<()>
    where
        T: AsRef<str>,
    {
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_int8(self.raw, CString::new(name.as_ref())?.as_ptr(), v)
        })?;

        Ok(())
    }

    pub fn add_int16<T>(&mut self, name: T, v: i16) -> Result<()>
    where
        T: AsRef<str>,
    {
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_int16(self.raw, CString::new(name.as_ref())?.as_ptr(), v)
        })?;

        Ok(())
    }

    pub fn add_int32<T>(&mut self, name: T, v: i32) -> Result<()>
    where
        T: AsRef<str>,
    {
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_int32(self.raw, CString::new(name.as_ref())?.as_ptr(), v)
        })?;

        Ok(())
    }

    pub fn add_int64<T>(&mut self, name: T, v: i64) -> Result<()>
    where
        T: AsRef<str>,
    {
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_int64(self.raw, CString::new(name.as_ref())?.as_ptr(), v)
        })?;

        Ok(())
    }

    pub fn add_uint8_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[u8]> + Sized,
    {
        unsafe {
            NvListError::from_nvlist_rc(sys::nvlist_add_uint8_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                v.as_ref().to_owned().as_mut_ptr(),
                v.as_ref().len() as u32,
            ))?;
        };

        Ok(())
    }

    pub fn add_uint16_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[u16]> + Sized,
    {
        unsafe {
            NvListError::from_nvlist_rc(sys::nvlist_add_uint16_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                v.as_ref().to_owned().as_mut_ptr(),
                v.as_ref().len() as u32,
            ))?;
        };

        Ok(())
    }

    pub fn add_uint32_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[u32]> + Sized,
    {
        unsafe {
            NvListError::from_nvlist_rc(sys::nvlist_add_uint32_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                v.as_ref().to_owned().as_mut_ptr(),
                v.as_ref().len() as u32,
            ))?;
        };

        Ok(())
    }

    pub fn add_uint64_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[u64]> + Sized,
    {
        unsafe {
            NvListError::from_nvlist_rc(sys::nvlist_add_uint64_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                v.as_ref().to_owned().as_mut_ptr(),
                v.as_ref().len() as u32,
            ))?;
        };

        Ok(())
    }

    pub fn add_int8_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[i8]> + Sized,
    {
        unsafe {
            NvListError::from_nvlist_rc(sys::nvlist_add_int8_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                v.as_ref().to_owned().as_mut_ptr(),
                v.as_ref().len() as u32,
            ))?;
        };

        Ok(())
    }

    pub fn add_int16_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[i16]> + Sized,
    {
        unsafe {
            NvListError::from_nvlist_rc(sys::nvlist_add_int16_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                v.as_ref().to_owned().as_mut_ptr(),
                v.as_ref().len() as u32,
            ))?;
        };

        Ok(())
    }

    pub fn add_int32_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[i32]> + Sized,
    {
        unsafe {
            NvListError::from_nvlist_rc(sys::nvlist_add_int32_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                v.as_ref().to_owned().as_mut_ptr(),
                v.as_ref().len() as u32,
            ))?;
        };

        Ok(())
    }

    pub fn add_int64_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[i64]> + Sized,
    {
        unsafe {
            NvListError::from_nvlist_rc(sys::nvlist_add_int64_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                v.as_ref().to_owned().as_mut_ptr(),
                v.as_ref().len() as u32,
            ))?;
        };

        Ok(())
    }

    pub fn add_float64<T>(&mut self, name: T, v: f64) -> Result<()>
    where
        T: AsRef<str>,
    {
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_double(self.raw, CString::new(name.as_ref())?.as_ptr(), v)
        })?;

        Ok(())
    }

    pub fn add_string<T>(&mut self, name: T, v: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_string(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                CString::new(v.as_ref())?.as_ptr(),
            )
        })?;

        Ok(())
    }

    // TODO: check if its ok
    pub fn add_string_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[String]> + Sized,
    {
        let mut vec = Vec::with_capacity(v.as_ref().len());

        for str in v.as_ref() {
            vec.push(str.clone())
        }

        let converted: Vec<*mut c_char> = vec
            .into_iter()
            .map(|s| CString::new(s).unwrap().into_raw())
            .collect();

        let x = converted.as_ptr();
        let len = converted.len();

        unsafe {
            NvListError::from_nvlist_rc(sys::nvlist_add_string_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                x,
                len as u32,
            ))?;
        };

        Ok(())
    }

    pub fn add_nvlist<T>(&mut self, name: T, v: &NvList) -> Result<()>
    where
        T: AsRef<str>,
    {
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_nvlist(self.raw, CString::new(name.as_ref())?.as_ptr(), v.raw)
        })?;

        Ok(())
    }

    pub fn lookup_nvpair<T>(&self, name: T) -> Result<NvPair>
    where
        T: AsRef<str>,
    {
        let mut nvpair: *mut nvpair_t = std::ptr::null_mut();
        let nvpair_ptr: *mut *mut nvpair_t = &mut nvpair;

        unsafe {
            NvListError::from_nvlist_rc(sys::nvlist_lookup_nvpair(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                nvpair_ptr,
            ))?;

            let nvpair = NvPair {
                raw_nvpair: *nvpair_ptr,
            };

            Ok(nvpair)
        }
    }
}

impl IntoIterator for NvList {
    type Item = NvPair;
    type IntoIter = NvListIterator;

    fn into_iter(self) -> Self::IntoIter {
        NvListIterator {
            nvlist: self,
            curr_nvpair: NvPair::default(),
            completed: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NvListIterator {
    pub nvlist: NvList,
    pub curr_nvpair: NvPair,
    pub completed: bool,
}

impl Iterator for NvListIterator {
    type Item = NvPair;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if !self.completed {
                self.curr_nvpair.raw_nvpair =
                    sys::nvlist_next_nvpair(self.nvlist.raw, self.curr_nvpair.raw_nvpair);

                match self.curr_nvpair.raw_nvpair.as_ref() {
                    Some(_) => Some(self.curr_nvpair.clone()),
                    None => {
                        self.completed = true;
                        None
                    }
                }
            } else {
                None
            }
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
        assert_eq!(NvPairType::Uint16, pair1.r#type());
        assert_eq!("b".to_string(), pair2.name().unwrap());
        assert_eq!(NvPairType::Uint32, pair2.r#type());
        assert_eq!("d".to_string(), pair3.name().unwrap());
        assert_eq!(NvPairType::Uint8Array, pair3.r#type());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}
