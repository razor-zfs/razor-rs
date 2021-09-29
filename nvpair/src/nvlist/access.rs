use super::*;

use ::std::result::Result as StdResult;

pub trait NvListAccess {
    fn nvl(&self) -> *mut libnvpair::nvlist_t;

    fn add_boolean_value(&mut self, name: impl AsRef<str>, v: bool) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let v = match v {
            false => libnvpair::boolean_t::B_TRUE,
            true => libnvpair::boolean_t::B_FALSE,
        };
        let rc = unsafe { libnvpair::nvlist_add_boolean_value(self.nvl(), name.as_ptr(), v) };
        value_or_err((), rc)
    }

    fn add_boolean(&mut self, name: impl AsRef<str>) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe { libnvpair::nvlist_add_boolean(self.nvl(), name.as_ptr()) };
        value_or_err((), rc)
    }

    fn add_boolean_array(&mut self, name: impl AsRef<str>, v: &[bool]) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let mut v = v
            .iter()
            .map(|item| match item {
                false => libnvpair::boolean_t::B_FALSE,
                true => libnvpair::boolean_t::B_TRUE,
            })
            .collect::<Vec<_>>();
        let length = v.len() as u32;
        let rc = unsafe {
            libnvpair::nvlist_add_boolean_array(self.nvl(), name.as_ptr(), v.as_mut_ptr(), length)
        };
        value_or_err((), rc)
    }

    fn add_int8(&mut self, name: impl AsRef<str>, v: i8) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe { libnvpair::nvlist_add_int8(self.nvl(), name.as_ptr(), v) };
        value_or_err((), rc)
    }

    fn add_uint8(&mut self, name: impl AsRef<str>, v: u8) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe { libnvpair::nvlist_add_uint8(self.nvl(), name.as_ptr(), v) };
        value_or_err((), rc)
    }

    fn add_uint16(&mut self, name: impl AsRef<str>, v: u16) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe { libnvpair::nvlist_add_uint16(self.nvl(), name.as_ptr(), v) };
        value_or_err((), rc)
    }

    fn add_uint32(&mut self, name: impl AsRef<str>, v: u32) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe { libnvpair::nvlist_add_uint32(self.nvl(), name.as_ptr(), v) };
        value_or_err((), rc)
    }

    fn add_uint64(&mut self, name: impl AsRef<str>, v: u64) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe { libnvpair::nvlist_add_uint64(self.nvl(), name.as_ptr(), v) };
        value_or_err((), rc)
    }

    fn add_int16(&mut self, name: impl AsRef<str>, v: i16) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe { libnvpair::nvlist_add_int16(self.nvl(), name.as_ptr(), v) };
        value_or_err((), rc)
    }

    fn add_int32(&mut self, name: impl AsRef<str>, v: i32) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe { libnvpair::nvlist_add_int32(self.nvl(), name.as_ptr(), v) };
        value_or_err((), rc)
    }

    fn add_int64(&mut self, name: impl AsRef<str>, v: i64) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe { libnvpair::nvlist_add_int64(self.nvl(), name.as_ptr(), v) };
        value_or_err((), rc)
    }

    fn add_uint8_array(&mut self, name: impl AsRef<str>, v: &[u8]) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe {
            libnvpair::nvlist_add_uint8_array(
                self.nvl(),
                name.as_ptr(),
                v.as_ptr() as *mut u8,
                v.len() as u32,
            )
        };
        value_or_err((), rc)
    }

    fn add_uint16_array(&mut self, name: impl AsRef<str>, v: &[u16]) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe {
            libnvpair::nvlist_add_uint16_array(
                self.nvl(),
                name.as_ptr(),
                v.as_ptr() as *mut u16,
                v.len() as u32,
            )
        };
        value_or_err((), rc)
    }

    fn add_uint32_array(&mut self, name: impl AsRef<str>, v: &[u32]) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe {
            libnvpair::nvlist_add_uint32_array(
                self.nvl(),
                name.as_ptr(),
                v.as_ptr() as *mut u32,
                v.len() as u32,
            )
        };
        value_or_err((), rc)
    }

    fn add_uint64_array(&mut self, name: impl AsRef<str>, v: &[u64]) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe {
            libnvpair::nvlist_add_uint64_array(
                self.nvl(),
                name.as_ptr(),
                v.as_ptr() as *mut u64,
                v.len() as u32,
            )
        };
        value_or_err((), rc)
    }

    fn add_int8_array(&mut self, name: impl AsRef<str>, v: &[i8]) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe {
            libnvpair::nvlist_add_int8_array(
                self.nvl(),
                name.as_ptr(),
                v.as_ptr() as *mut i8,
                v.len() as u32,
            )
        };
        value_or_err((), rc)
    }

    fn add_int16_array(&mut self, name: impl AsRef<str>, v: &[i16]) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe {
            libnvpair::nvlist_add_int16_array(
                self.nvl(),
                name.as_ptr(),
                v.as_ptr() as *mut i16,
                v.len() as u32,
            )
        };
        value_or_err((), rc)
    }

    fn add_int32_array(&mut self, name: impl AsRef<str>, v: &[i32]) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe {
            libnvpair::nvlist_add_int32_array(
                self.nvl(),
                name.as_ptr(),
                v.as_ptr() as *mut i32,
                v.len() as u32,
            )
        };
        value_or_err((), rc)
    }

    fn add_int64_array(&mut self, name: impl AsRef<str>, v: &[i64]) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe {
            libnvpair::nvlist_add_int64_array(
                self.nvl(),
                name.as_ptr(),
                v.as_ptr() as *mut i64,
                v.len() as u32,
            )
        };
        value_or_err((), rc)
    }

    fn add_float64(&mut self, name: impl AsRef<str>, v: f64) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe { libnvpair::nvlist_add_double(self.nvl(), name.as_ptr(), v) };
        value_or_err((), rc)
    }

    fn add_string(&mut self, name: impl AsRef<str>, v: impl AsRef<str>) -> Result<()> {
        let name = CString::new(name.as_ref())?;
        let v = CString::new(v.as_ref())?;
        let rc = unsafe { libnvpair::nvlist_add_string(self.nvl(), name.as_ptr(), v.as_ptr()) };
        value_or_err((), rc)
    }

    fn add_string_array<T>(&mut self, name: impl AsRef<str>, v: &[T]) -> Result<()>
    where
        T: AsRef<str>,
    {
        let name = CString::new(name.as_ref())?;

        let cstrings = v
            .iter()
            .map(|x| x.as_ref())
            .map(CString::new)
            .collect::<StdResult<Vec<_>, _>>()?;

        let v = cstrings
            .iter()
            .map(|item| item.as_ptr() as *mut libc::c_char)
            .collect::<Vec<_>>();

        let rc = unsafe {
            libnvpair::nvlist_add_string_array(
                self.nvl(),
                name.as_ptr(),
                v.as_ptr(),
                v.len() as u32,
            )
        };
        value_or_err((), rc)
    }

    fn add_nvlist<T>(&mut self, name: impl AsRef<str>, v: &T) -> Result<()>
    where
        T: NvListAccess,
    {
        let name = CString::new(name.as_ref())?;
        let rc = unsafe { libnvpair::nvlist_add_nvlist(self.nvl(), name.as_ptr(), v.nvl()) };
        value_or_err((), rc)
    }

    fn lookup_nvpair(&self, name: impl AsRef<str>) -> Result<NvPair> {
        let name = CString::new(name.as_ref())?;
        let nvpair = unsafe { libnvpair::nvlist_lookup_nvpair(self.nvl(), name.as_ptr()) };
        Ok(nvpair.into())
    }

    fn iter(&self) -> Iter<'_> {
        Iter {
            nvl: self.nvl(),
            nvp: None,
            anchor: PhantomData,
        }
    }

    fn items(&self) -> Items<'_> {
        Items {
            nvl: self.nvl(),
            nvp: None,
            anchor: PhantomData,
        }
    }
}
