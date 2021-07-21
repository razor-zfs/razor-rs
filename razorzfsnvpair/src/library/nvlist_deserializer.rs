use core::slice;
use std::ffi::{CStr, CString};
use std::ops::{AddAssign, MulAssign, Neg};
use std::usize;

use super::*;
use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};
use serde::Deserialize;

pub struct NvListDeserializer<'de> {
    input: &'de *mut sys::nvlist_t,
    nvpair: *mut sys::nvpair_t,
    first: bool,
    size: usize,
    index: usize,
}

impl<'de> NvListDeserializer<'de> {
    pub fn from_nvlist(input: &'de *mut sys::nvlist_t) -> Self {
        let nvpair: *mut sys::nvpair_t = std::ptr::null_mut();
        NvListDeserializer {
            input,
            nvpair,
            first: true,
            size: usize::MAX,
            index: 0,
        }
    }
}

pub fn from_nvlist<'a, T>(s: &'a *mut sys::nvlist_t) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = NvListDeserializer::from_nvlist(s);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
    // if deserializer.input.is_empty() {
    //     Ok(t)
    // } else {
    //     Err(NvListError::UnmatchingVariables)
    // }
}

impl<'de> NvListDeserializer<'de> {}

impl<'de, 'a> de::Deserializer<'de> for &'a mut NvListDeserializer<'de> {
    type Error = NvListError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unsafe {
            match match self.nvpair.as_ref() {
                Some(_) => sys::nvpair_type(self.nvpair),
                None => todo!(),
            } {
                sys::data_type_t::DATA_TYPE_UINT16_ARRAY => {
                    dbg!("Deserializing u16 arr");
                    let mut x = 0;
                    let val: *mut sys::uint_t = &mut x;
                    dbg!(self.nvpair);
                    dbg!(unsafe { *self.nvpair });
                    let mut u16arr: *mut u16 = std::ptr::null_mut();
                    let mut u16arr_ptr: *mut *mut u16 = &mut u16arr;
                    NvListError::from_nvlist_rc(sys::nvpair_value_uint16_array(
                        self.nvpair,
                        u16arr_ptr,
                        val,
                    ))?;
                    dbg!("len of array: ", x);
                    self.size = x as usize;

                    match u16arr_ptr.as_ref() {
                        Some(arr) => {
                            dbg!(slice::from_raw_parts(*arr, x as usize));
                            let a = slice::from_raw_parts(*arr, x as usize).to_vec();
                            let x = self.index;
                            self.index += 1;
                            visitor.visit_u16(a[x])
                        }
                        None => Err(NvListError::ConversionError),
                    }
                }
                sys::data_type_t::DATA_TYPE_UINT16 => {
                    dbg!("Deserializing u16");
                    let mut x = 0;
                    let val: *mut u16 = &mut x;
                    dbg!(self.nvpair);
                    dbg!(unsafe { *self.nvpair });
                    NvListError::from_nvlist_rc(unsafe {
                        sys::nvpair_value_uint16(self.nvpair, val)
                    })?;

                    match unsafe { val.as_ref() } {
                        Some(x) => {
                            dbg!(*x);
                            visitor.visit_u16(*x)
                        }
                        None => Err(NvListError::ConversionError),
                    }
                }
                _ => Err(NvListError::InvalidArgument),
            }
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing u32");
        let mut x = 0;
        let val: *mut u32 = &mut x;
        dbg!(self.nvpair);
        dbg!(unsafe { *self.nvpair });
        NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_uint32(self.nvpair, val) })?;

        match unsafe { val.as_ref() } {
            Some(x) => {
                dbg!(*x);
                visitor.visit_u32(*x)
            }
            None => Err(NvListError::ConversionError),
        }
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing str");
        let mut mystr: *mut u8 = std::ptr::null_mut();
        let mut mystr_ptr: *mut *mut u8 = &mut mystr;
        NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_string(self.nvpair, mystr_ptr) })?;
        dbg!(unsafe { CStr::from_ptr(*mystr_ptr).to_str()? });
        visitor.visit_borrowed_str(unsafe { CStr::from_ptr(*mystr_ptr).to_str()? })
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing seq");
        let value = visitor.visit_seq(CommaSeparated::new(&mut self))?;

        Ok(value)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing typle");
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing map");
        let value = visitor.visit_map(CommaSeparated::new(&mut self))?;
        Ok(value)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing struct");
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing object identifier");
        dbg!(self.nvpair);
        dbg!(unsafe { CStr::from_ptr(sys::nvpair_name(self.nvpair)).to_str()? });
        visitor
            .visit_borrowed_str(unsafe { CStr::from_ptr(sys::nvpair_name(self.nvpair)).to_str()? })
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }
}

struct CommaSeparated<'a, 'de: 'a> {
    de: &'a mut NvListDeserializer<'de>,
}

impl<'a, 'de> CommaSeparated<'a, 'de> {
    fn new(de: &'a mut NvListDeserializer<'de>) -> Self {
        CommaSeparated { de }
    }
}

impl<'de, 'a> SeqAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = NvListError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        dbg!("Deserializing seq in SeqAccess");
        if self.de.first {
            self.de.first = false;
            dbg!("Deserializing seq in SeqAccess first");
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            dbg!("Deserializing seq in not first");
            if self.de.index < self.de.size {
                seed.deserialize(&mut *self.de).map(Some)
            } else {
                self.de.index = 0;
                self.de.size = 0;
                self.de.first = true;
                Ok(None)
            }
        }
    }
}

impl<'de, 'a> MapAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = NvListError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        dbg!("Deserializing map key");
        self.de.nvpair = unsafe { sys::nvlist_next_nvpair(*self.de.input, self.de.nvpair) };
        let ptr = unsafe { self.de.nvpair.as_ref() };

        match ptr {
            Some(_) => seed.deserialize(&mut *self.de).map(Some),
            None => {
                self.de.nvpair = std::ptr::null_mut();
                Ok(None)
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        dbg!("Deserializing map value");
        seed.deserialize(&mut *self.de)
    }
}

struct Enum<'a, 'de: 'a> {
    de: &'a mut NvListDeserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn _new(de: &'a mut NvListDeserializer<'de>) -> Self {
        Enum { de }
    }
}

impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = NvListError;
    type Variant = Self;

    fn variant_seed<V>(self, _seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        todo!();
    }
}

impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = NvListError;

    fn unit_variant(self) -> Result<()> {
        todo!();
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        todo!();
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;
    use serde::Deserialize;

    #[test]
    fn basic_de() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: u16,
            b: u32,
        }

        let expected = Test { a: 3, b: 5 };
        let mut nvlist: *mut sys::nvlist_t = std::ptr::null_mut();
        let mut nvlist_ptr: *mut *mut sys::nvlist_t = &mut nvlist;
        let ret = unsafe { sys::nvlist_alloc(nvlist_ptr, sys::NV_UNIQUE_NAME, 0) };
        let mut nvlist = unsafe { *nvlist_ptr };
        unsafe { sys::nvlist_add_uint16(nvlist, CString::new("a").unwrap().as_ptr(), 3) };
        unsafe { sys::nvlist_add_uint32(nvlist, CString::new("b").unwrap().as_ptr(), 5) };

        assert_eq!(expected, from_nvlist(&nvlist).unwrap());
    }

    #[test]
    fn with_str_de() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test<'a> {
            a: u16,
            b: u32,
            c: &'a str,
        }

        let expected = Test {
            a: 3,
            b: 5,
            c: "test",
        };
        let mut nvlist: *mut sys::nvlist_t = std::ptr::null_mut();
        let mut nvlist_ptr: *mut *mut sys::nvlist_t = &mut nvlist;
        let ret = unsafe { sys::nvlist_alloc(nvlist_ptr, sys::NV_UNIQUE_NAME, 0) };
        let mut nvlist = unsafe { *nvlist_ptr };
        unsafe { sys::nvlist_add_uint16(nvlist, CString::new("a").unwrap().as_ptr(), 3) };
        unsafe { sys::nvlist_add_uint32(nvlist, CString::new("b").unwrap().as_ptr(), 5) };
        unsafe {
            sys::nvlist_add_string(
                nvlist,
                CString::new("c").unwrap().as_ptr(),
                CString::new("test").unwrap().as_ptr().to_owned(),
            )
        };

        assert_eq!(expected, from_nvlist(&nvlist).unwrap());
    }

    #[test]
    fn with_vec_de() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test<'a> {
            a: u16,
            b: u32,
            c: &'a str,
            d: Vec<u16>,
        }

        let expected = Test {
            a: 3,
            b: 5,
            c: "test",
            d: vec![1, 2, 3, 4, 5],
        };
        let mut nvlist: *mut sys::nvlist_t = std::ptr::null_mut();
        let mut nvlist_ptr: *mut *mut sys::nvlist_t = &mut nvlist;
        let ret = unsafe { sys::nvlist_alloc(nvlist_ptr, sys::NV_UNIQUE_NAME, 0) };
        let mut nvlist = unsafe { *nvlist_ptr };
        unsafe { sys::nvlist_add_uint16(nvlist, CString::new("a").unwrap().as_ptr(), 3) };
        unsafe { sys::nvlist_add_uint32(nvlist, CString::new("b").unwrap().as_ptr(), 5) };
        unsafe {
            sys::nvlist_add_string(
                nvlist,
                CString::new("c").unwrap().as_ptr(),
                CString::new("test").unwrap().as_ptr().to_owned(),
            )
        };
        let mut arr: [u16; 5] = [1, 2, 3, 4, 5];
        unsafe {
            sys::nvlist_add_uint16_array(
                nvlist,
                CString::new("d").unwrap().as_ptr(),
                arr.as_mut_ptr(),
                arr.len() as u32,
            )
        };

        assert_eq!(expected, from_nvlist(&nvlist).unwrap());
    }

    #[test]
    fn with_tuple_de() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test<'a> {
            a: u16,
            b: u32,
            c: &'a str,
            d: Vec<u16>,
            e: (u16, u16, u16),
        }

        let expected = Test {
            a: 3,
            b: 5,
            c: "test",
            d: vec![1, 2, 3, 4, 5],
            e: (1, 2, 3),
        };
        let mut nvlist: *mut sys::nvlist_t = std::ptr::null_mut();
        let mut nvlist_ptr: *mut *mut sys::nvlist_t = &mut nvlist;
        let ret = unsafe { sys::nvlist_alloc(nvlist_ptr, sys::NV_UNIQUE_NAME, 0) };
        let mut nvlist = unsafe { *nvlist_ptr };
        unsafe { sys::nvlist_add_uint16(nvlist, CString::new("a").unwrap().as_ptr(), 3) };
        unsafe { sys::nvlist_add_uint32(nvlist, CString::new("b").unwrap().as_ptr(), 5) };
        unsafe {
            sys::nvlist_add_string(
                nvlist,
                CString::new("c").unwrap().as_ptr(),
                CString::new("test").unwrap().as_ptr().to_owned(),
            )
        };
        let mut arr: [u16; 5] = [1, 2, 3, 4, 5];
        unsafe {
            sys::nvlist_add_uint16_array(
                nvlist,
                CString::new("d").unwrap().as_ptr(),
                arr.as_mut_ptr(),
                arr.len() as u32,
            )
        };
        let mut arr: [u16; 3] = [1, 2, 3];
        unsafe {
            sys::nvlist_add_uint16_array(
                nvlist,
                CString::new("e").unwrap().as_ptr(),
                arr.as_mut_ptr(),
                arr.len() as u32,
            )
        };

        assert_eq!(expected, from_nvlist(&nvlist).unwrap());
    }
}
