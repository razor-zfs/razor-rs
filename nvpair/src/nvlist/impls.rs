use std::ops;

use super::*;

macro_rules! nvlist_add_assign {
    ($add:ident, $value:ty) => {
        impl ops::AddAssign<(&str, $value)> for NvList {
            fn add_assign(&mut self, (name, value): (&str, $value)) {
                let name = fcstring(name);
                let value = value.into();
                unsafe { libnvpair::$add(self.nvl, name.as_ptr(), value) }
            }
        }

        impl<'a, T> ops::AddAssign<(&str, $value)> for NvListRef<'a, T> {
            fn add_assign(&mut self, (name, value): (&str, $value)) {
                let name = fcstring(name);
                let value = value.into();
                unsafe { libnvpair::$add(self.nvl, name.as_ptr(), value) }
            }
        }
    };
}

nvlist_add_assign!(fnvlist_add_boolean_value, bool);
nvlist_add_assign!(fnvlist_add_int8, i8);
nvlist_add_assign!(fnvlist_add_uint8, u8);
nvlist_add_assign!(fnvlist_add_int16, i16);
nvlist_add_assign!(fnvlist_add_uint16, u16);
nvlist_add_assign!(fnvlist_add_int32, i32);
nvlist_add_assign!(fnvlist_add_uint32, u32);
nvlist_add_assign!(fnvlist_add_int64, i64);
nvlist_add_assign!(fnvlist_add_uint64, u64);

impl ops::AddAssign<&str> for NvList {
    fn add_assign(&mut self, name: &str) {
        let name = fcstring(name);
        unsafe { libnvpair::fnvlist_add_boolean(self.nvl, name.as_ptr()) }
    }
}

impl<'a, T> ops::AddAssign<&str> for NvListRef<'a, T> {
    fn add_assign(&mut self, name: &str) {
        let name = fcstring(name);
        unsafe { libnvpair::fnvlist_add_boolean(self.nvl, name.as_ptr()) }
    }
}

impl ops::AddAssign<(&str, &str)> for NvList {
    fn add_assign(&mut self, (name, value): (&str, &str)) {
        let name = fcstring(name);
        let value = fcstring(value);
        unsafe { libnvpair::fnvlist_add_string(self.nvl, name.as_ptr(), value.as_ptr()) }
    }
}

impl<'a, T> ops::AddAssign<(&str, &str)> for NvListRef<'a, T> {
    fn add_assign(&mut self, (name, value): (&str, &str)) {
        let name = fcstring(name);
        let value = fcstring(value);
        unsafe { libnvpair::fnvlist_add_string(self.nvl, name.as_ptr(), value.as_ptr()) }
    }
}

macro_rules! nvlist_add_assign_array {
    ($add:ident, $value:ty) => {
        impl ops::AddAssign<(&str, &[$value])> for NvList {
            fn add_assign(&mut self, (name, value): (&str, &[$value])) {
                let name = fcstring(name);
                unsafe {
                    libnvpair::$add(
                        self.nvl,
                        name.as_ptr(),
                        value.as_ptr() as *mut $value,
                        value.len() as u32,
                    )
                }
            }
        }

        impl<'a, T> ops::AddAssign<(&str, &[$value])> for NvListRef<'a, T> {
            fn add_assign(&mut self, (name, value): (&str, &[$value])) {
                let name = fcstring(name);
                unsafe {
                    libnvpair::$add(
                        self.nvl,
                        name.as_ptr(),
                        value.as_ptr() as *mut $value,
                        value.len() as u32,
                    )
                }
            }
        }
    };
}

nvlist_add_assign_array!(fnvlist_add_int8_array, i8);
nvlist_add_assign_array!(fnvlist_add_uint8_array, u8);
nvlist_add_assign_array!(fnvlist_add_int16_array, i16);
nvlist_add_assign_array!(fnvlist_add_uint16_array, u16);
nvlist_add_assign_array!(fnvlist_add_int32_array, i32);
nvlist_add_assign_array!(fnvlist_add_uint32_array, u32);
nvlist_add_assign_array!(fnvlist_add_int64_array, i64);
nvlist_add_assign_array!(fnvlist_add_uint64_array, u64);

macro_rules! nvlist_add {
    ($add:ident, $method:ident, $value:ty) => {
        impl NvList {
            pub fn $add(
                &mut self,
                name: impl AsRef<str>,
                value: $value,
            ) -> Result<(), NvListError> {
                let name = cstring(name)?;
                let value = value.into();
                unsafe { libnvpair::$method(self.nvl, name.as_ptr(), value) }
            }
        }

        impl<'a, T> NvListRef<'a, T> {
            pub fn $add(
                &mut self,
                name: impl AsRef<str>,
                value: $value,
            ) -> Result<(), NvListError> {
                let name = cstring(name)?;
                let value = value.into();
                unsafe { libnvpair::$method(self.nvl, name.as_ptr(), value) }
            }
        }
    };
}

nvlist_add!(add_boolean_value, nvlist_add_boolean_value, bool);
nvlist_add!(add_int8, nvlist_add_int8, i8);
nvlist_add!(add_uint8, nvlist_add_uint8, u8);
nvlist_add!(add_int16, nvlist_add_int16, i16);
nvlist_add!(add_uint16, nvlist_add_uint16, u16);
nvlist_add!(add_int32, nvlist_add_int32, i32);
nvlist_add!(add_uint32, nvlist_add_uint32, u32);
nvlist_add!(add_int64, nvlist_add_int64, i64);
nvlist_add!(add_uint64, nvlist_add_uint64, u64);
nvlist_add!(add_f64, nvlist_add_double, f64);

macro_rules! nvlist_add_array {
    ($add:ident, $method:ident, $value:ty) => {
        impl NvList {
            pub fn $add(
                &mut self,
                name: impl AsRef<str>,
                value: &[$value],
            ) -> Result<(), NvListError> {
                let name = cstring(name)?;
                unsafe {
                    libnvpair::$method(
                        self.nvl,
                        name.as_ptr(),
                        value.as_ptr() as *mut $value,
                        value.len() as u32,
                    )
                }
            }
        }

        impl<'a, T> NvListRef<'a, T> {
            pub fn $add(
                &mut self,
                name: impl AsRef<str>,
                value: &[$value],
            ) -> Result<(), NvListError> {
                let name = cstring(name)?;
                unsafe {
                    libnvpair::$method(
                        self.nvl,
                        name.as_ptr(),
                        value.as_ptr() as *mut $value,
                        value.len() as u32,
                    )
                }
            }
        }
    };
}

nvlist_add_array!(add_int8_array, nvlist_add_int8_array, i8);
nvlist_add_array!(add_uint8_array, nvlist_add_uint8_array, u8);
nvlist_add_array!(add_int16_array, nvlist_add_int16_array, i16);
nvlist_add_array!(add_uint16_array, nvlist_add_uint16_array, u16);
nvlist_add_array!(add_int32_array, nvlist_add_int32_array, i32);
nvlist_add_array!(add_uint32_array, nvlist_add_uint32_array, u32);
nvlist_add_array!(add_int64_array, nvlist_add_int64_array, i64);
nvlist_add_array!(add_uint64_array, nvlist_add_uint64_array, u64);

impl NvList {
    /// Add named boolean (without value, i.e. always true) to this nvlist
    pub fn add_boolean(&mut self, name: impl AsRef<str>) -> Result<(), NvListError> {
        let name = cstring(name)?;
        unsafe { libnvpair::nvlist_add_boolean(self.nvl, name.as_ptr()) }
    }

    /// Add named string to this nvlist
    pub fn add_string(
        &mut self,
        name: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> Result<(), NvListError> {
        add_string(self.nvl, name, value)
    }

    /// Add named nvlist to this nvlist
    pub fn add_nvlist(
        &mut self,
        name: impl AsRef<str>,
        v: impl AsRef<*mut libnvpair::nvlist_t>,
    ) -> Result<(), NvListError> {
        let name = cstring(name)?;
        let nvl = v.as_ref();
        unsafe { libnvpair::nvlist_add_nvlist(self.nvl, name.as_ptr(), *nvl) }
    }

    /// Add named boolean array/slice to this nvlist
    pub fn add_boolean_array(
        &mut self,
        name: impl AsRef<str>,
        v: &[bool],
    ) -> Result<(), NvListError> {
        add_boolean_array_impl(self.nvl, name, v)
    }

    /// Add named string array/slice to this nvlist
    pub fn add_string_array<S>(&mut self, name: impl AsRef<str>, v: &[S]) -> Result<(), NvListError>
    where
        S: AsRef<str>,
    {
        add_string_array_impl(self.nvl, name, v)
    }

    /// Lookup nvpair by name
    pub fn lookup_nvpair(&self, name: impl AsRef<str>) -> Result<Option<NvPair>, NvListError> {
        let name = cstring(name).map_err(|_| NvListError::InvalidArgument)?;
        match unsafe { libnvpair::nvlist_lookup_nvpair(self.nvl, name.as_ptr()) } {
            Ok(nvp) => Ok(Some(NvPair::from(nvp))),
            Err(NvListError::NotFound) => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Iterator over NvPair objects in this NvList
    pub fn iter(&self) -> Iter<'_, Self> {
        Iter {
            nvlist: self.borrow(),
            nvpair: None,
        }
    }

    /// Iterator over (name, value) items in this NvList
    pub fn items(&self) -> Items<'_, Self> {
        Items {
            nvlist: self.borrow(),
            nvpair: None,
        }
    }
}

impl<'a, T> NvListRef<'a, T> {
    /// Add named string to the nvlist
    pub fn add_string(
        &mut self,
        name: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> Result<(), NvListError> {
        add_string(self.nvl, name, value)
    }

    /// Add named nvlist to this nvlist
    pub fn add_nvlist(
        &mut self,
        name: impl AsRef<str>,
        v: impl AsRef<*mut libnvpair::nvlist_t>,
    ) -> Result<(), NvListError> {
        let name = cstring(name)?;
        let nvl = v.as_ref();
        unsafe { libnvpair::nvlist_add_nvlist(self.nvl, name.as_ptr(), *nvl) }
    }

    /// Add named boolean array/slice to this nvlist
    pub fn add_boolean_array(
        &mut self,
        name: impl AsRef<str>,
        v: &[bool],
    ) -> Result<(), NvListError> {
        add_boolean_array_impl(self.nvl, name, v)
    }

    /// Add named string array/slice to this nvlist
    pub fn add_string_array<S>(&mut self, name: impl AsRef<str>, v: &[S]) -> Result<(), NvListError>
    where
        S: AsRef<str>,
    {
        add_string_array_impl(self.nvl, name, v)
    }

    /// Lookup nvpair by name
    pub fn lookup_nvpair(&self, name: impl AsRef<str>) -> Result<Option<NvPair>, NvListError> {
        let name = cstring(name).map_err(|_| NvListError::InvalidArgument)?;
        match unsafe { libnvpair::nvlist_lookup_nvpair(self.nvl, name.as_ptr()) } {
            Ok(nvp) => Ok(Some(NvPair::from(nvp))),
            Err(NvListError::NotFound) => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Iterator over NvPair objects in this NvList
    pub fn iter(&self) -> Iter<'_, Self> {
        Iter {
            nvlist: self.borrow(),
            nvpair: None,
        }
    }

    /// Iterator over (name, value) items in this NvList
    pub fn items(&self) -> Items<'_, Self> {
        Items {
            nvlist: self.borrow(),
            nvpair: None,
        }
    }
}

#[inline]
fn add_string(
    nvl: *mut libnvpair::nvlist_t,
    name: impl AsRef<str>,
    value: impl AsRef<str>,
) -> Result<(), NvListError> {
    let name = fcstring(name);
    let value = fcstring(value);
    unsafe { libnvpair::nvlist_add_string(nvl, name.as_ptr(), value.as_ptr()) }
}

#[inline]
fn add_boolean_array_impl(
    nvl: *mut libnvpair::nvlist_t,
    name: impl AsRef<str>,
    v: &[bool],
) -> Result<(), NvListError> {
    let name = cstring(name)?;
    let mut v = v.iter().map(Into::into).collect::<Vec<_>>();
    let nelem = v.len() as u32;
    unsafe { libnvpair::nvlist_add_boolean_array(nvl, name.as_ptr(), v.as_mut_ptr(), nelem) }
}

#[inline]
fn add_string_array_impl<T>(
    nvl: *mut libnvpair::nvlist_t,
    name: impl AsRef<str>,
    v: &[T],
) -> Result<(), NvListError>
where
    T: AsRef<str>,
{
    let name = cstring(name)?;
    let cstrings = v.iter().map(cstring).collect::<Result<Vec<_>, _>>()?;
    // cstrings needs to live until the end of this function
    let v = cstrings
        .iter()
        .map(|item| item.as_ptr() as *mut libc::c_char)
        .collect::<Vec<_>>();
    let nelem = v.len() as u32;
    unsafe { libnvpair::nvlist_add_string_array(nvl, name.as_ptr(), v.as_ptr(), nelem) }
}

#[inline]
fn cstring(text: impl AsRef<str>) -> Result<CString, NvListError> {
    CString::new(text.as_ref()).map_err(|_| NvListError::InvalidArgument)
}

#[inline]
fn fcstring(text: impl AsRef<str>) -> CString {
    CString::new(text.as_ref()).expect("Failed to convert {text} to CString")
}
