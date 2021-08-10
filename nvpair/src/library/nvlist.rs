use std::ffi::{CString, NulError};
use std::marker::PhantomData;
use std::ops::Not;
use std::panic;
use std::ptr;
use std::result::Result as StdResult;

use libc::c_char;

use super::*;

#[derive(PartialEq)]
pub struct NvList {
    pub raw: *mut sys::nvlist_t,
}

impl NvList {
    pub fn nvlist_alloc(flag: NvFlag) -> Result<Self> {
        let mut nvlist: *mut sys::nvlist_t = ptr::null_mut();
        let nvlist_ptr: *mut *mut sys::nvlist_t = &mut nvlist;

        match flag {
            NvFlag::UniqueName => NvListError::from_nvlist_rc(unsafe {
                sys::nvlist_alloc(nvlist_ptr, sys::NV_UNIQUE_NAME, 0)
            })?,
            NvFlag::UniqueNameType => NvListError::from_nvlist_rc(unsafe {
                sys::nvlist_alloc(nvlist_ptr, sys::NV_UNIQUE_NAME, 0)
            })?,
        }

        Ok(Self {
            raw: unsafe { *nvlist_ptr },
        })
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

        let length = conversion.len() as u32;
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_boolean_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                conversion.as_mut_ptr(),
                length,
            )
        })?;

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
        let length = v.as_ref().len() as u32;
        let arr = v.as_ref().as_ptr() as *mut u8;
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_uint8_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                arr,
                length,
            )
        })?;

        Ok(())
    }

    pub fn add_uint16_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[u16]> + Sized,
    {
        let length = v.as_ref().len() as u32;
        let arr = v.as_ref().as_ptr() as *mut u16;
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_uint16_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                arr,
                length,
            )
        })?;

        Ok(())
    }

    pub fn add_uint32_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[u32]> + Sized,
    {
        let length = v.as_ref().len() as u32;
        let arr = v.as_ref().as_ptr() as *mut u32;
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_uint32_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                arr,
                length,
            )
        })?;

        Ok(())
    }

    pub fn add_uint64_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[u64]> + Sized,
    {
        let length = v.as_ref().len() as u32;
        let arr = v.as_ref().as_ptr() as *mut u64;
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_uint64_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                arr,
                length,
            )
        })?;

        Ok(())
    }

    pub fn add_int8_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[i8]> + Sized,
    {
        let length = v.as_ref().len() as u32;
        let arr = v.as_ref().as_ptr() as *mut i8;
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_int8_array(self.raw, CString::new(name.as_ref())?.as_ptr(), arr, length)
        })?;

        Ok(())
    }

    pub fn add_int16_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[i16]> + Sized,
    {
        let length = v.as_ref().len() as u32;
        let arr = v.as_ref().as_ptr() as *mut i16;
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_int16_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                arr,
                length,
            )
        })?;

        Ok(())
    }

    pub fn add_int32_arr<T, W>(&mut self, name: T, v: &W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[i32]> + Sized,
    {
        let length = v.as_ref().len() as u32;
        let arr = v.as_ref().as_ptr() as *mut i32;
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_int32_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                arr,
                length,
            )
        })?;

        Ok(())
    }

    pub fn add_int64_arr<T, W>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[i64]> + Sized,
    {
        let length = v.as_ref().len() as u32;
        let arr = v.as_ref().as_ptr() as *mut i64;
        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_int64_array(
                self.raw,
                CString::new(name.as_ref())?.as_ptr(),
                arr,
                length,
            )
        })?;

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

    pub fn add_string_arr<T, W, S>(&mut self, name: T, v: W) -> Result<()>
    where
        T: AsRef<str>,
        W: AsRef<[S]> + Sized,
        S: AsRef<str>,
    {
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

        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_add_string_array(self.raw, CString::new(name.as_ref())?.as_ptr(), x, len)
        })?;

        Ok(())
    }

    pub fn add_nvlist<T>(&mut self, name: T, v: &Self) -> Result<()>
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
        let mut nvpair: *mut sys::nvpair_t = ptr::null_mut();
        let nvpair_ptr: *mut *mut sys::nvpair_t = &mut nvpair;

        NvListError::from_nvlist_rc(unsafe {
            sys::nvlist_lookup_nvpair(self.raw, CString::new(name.as_ref())?.as_ptr(), nvpair_ptr)
        })?;

        let nvpair = NvPair::from(unsafe { *nvpair_ptr });

        Ok(nvpair)
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            nvl: self.raw,
            nvp: None,
            anchor: PhantomData,
        }
    }

    pub fn items(&self) -> Items<'_> {
        Items {
            nvl: self.raw,
            nvp: None,
            anchor: PhantomData,
        }
    }
}

impl From<*mut sys::nvlist_t> for NvList {
    fn from(nvl: *mut sys::nvlist_t) -> Self {
        Self { raw: nvl }
    }
}

impl Clone for NvList {
    fn clone(&self) -> Self {
        let mut cloned_nvlist: *mut sys::nvlist_t = ptr::null_mut();
        let cloned_nvlist_ptr: *mut *mut sys::nvlist_t = &mut cloned_nvlist;
        let rc = unsafe { sys::nvlist_dup(self.raw, cloned_nvlist_ptr, 0) };

        if rc == libc::EINVAL {
            panic!("Nvlist clone invalid argument");
        } else if rc == libc::ENOMEM {
            panic!("Nvlist clone insufficient memory");
        }

        unsafe { Self::from(*cloned_nvlist_ptr) }
    }
}

impl Drop for NvList {
    fn drop(&mut self) {
        unsafe { sys::nvlist_free(self.raw) };
    }
}

impl IntoIterator for NvList {
    type Item = NvPair;
    type IntoIter = NvListIterator;

    fn into_iter(self) -> Self::IntoIter {
        NvListIterator {
            nvlist: self,
            nvp: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct NvListIterator {
    nvlist: NvList,
    nvp: Option<*mut sys::nvpair_t>,
}

impl Iterator for NvListIterator {
    type Item = NvPair;

    fn next(&mut self) -> Option<Self::Item> {
        let nvp = self.nvp.unwrap_or_else(ptr::null_mut);
        let nvp = unsafe { sys::nvlist_next_nvpair(self.nvlist.raw, nvp) };
        self.nvp = nvp.is_null().not().then(|| nvp);
        self.nvp.map(NvPair::from)
    }
}

#[derive(Debug)]
pub struct Iter<'a> {
    nvl: *mut sys::nvlist_t,
    nvp: Option<*mut sys::nvpair_t>,
    anchor: PhantomData<&'a NvList>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = NvPair;

    fn next(&mut self) -> Option<Self::Item> {
        let nvp = self.nvp.unwrap_or_else(ptr::null_mut);
        let nvp = unsafe { sys::nvlist_next_nvpair(self.nvl, nvp) };
        self.nvp = nvp.is_null().not().then(|| nvp);
        self.nvp.map(NvPair::from)
    }
}

#[derive(Debug)]
pub struct Items<'a> {
    nvl: *mut sys::nvlist_t,
    nvp: Option<*mut sys::nvpair_t>,
    anchor: PhantomData<&'a NvList>,
}

impl<'a> Iterator for Items<'a> {
    type Item = (String, Value);

    fn next(&mut self) -> Option<Self::Item> {
        let nvp = self.nvp.unwrap_or_else(ptr::null_mut);
        let nvp = unsafe { sys::nvlist_next_nvpair(self.nvl, nvp) };
        self.nvp = nvp.is_null().not().then(|| nvp);
        self.nvp
            .map(NvPair::from)
            .map(|nvpair| (nvpair.name().to_string(), nvpair.value()))
    }
}

#[derive(Debug)]
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

        let mut iter = dbg!(nvlist).into_iter();
        let pair1 = dbg!(iter.next().unwrap());
        let pair2 = dbg!(iter.next().unwrap());
        let pair3 = dbg!(iter.next().unwrap());
        assert_eq!(pair1.name(), "a");
        assert_eq!(NvPairType::Uint16, pair1.r#type());
        assert_eq!(pair2.name(), "b");
        assert_eq!(NvPairType::Uint32, pair2.r#type());
        assert_eq!(pair3.name(), "d");
        assert_eq!(NvPairType::Uint8Array, pair3.r#type());
        assert_eq!(None, iter.next());
    }
}
