use std::usize;

use super::*;
use libnvpair::{NvListIterator, NvPair};
use serde::de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::Deserialize;

pub struct NvListDeserializer<'de> {
    input: &'de mut libnvpair::NvList,
    first: bool,
    size: usize,
    index: usize,
}

impl<'de> NvListDeserializer<'de> {
    pub fn from_nvlist(input: &'de mut libnvpair::NvList) -> Self {
        NvListDeserializer {
            input,
            first: true,
            size: usize::MAX,
            index: 0,
        }
    }
}

pub fn _from_nvlist<'a, T>(s: &'a mut libnvpair::NvList) -> Result<T>
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
    type Error = libnvpair::NvListError;

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
            match match self.input.curr_nvpair.raw_nvpair.as_ref() {
                Some(_) => libnvpair::nvpair_type(&mut self.input.curr_nvpair)?,
                None => todo!(),
            } {
                libnvpair::NvPairType::Uint16Array => {
                    dbg!("Deserializing u16 arr");
                    libnvpair::nvpair_value_uint16_array(&mut self.input.curr_nvpair)?;
                    let curr = self.index;
                    self.index += 1;
                    if let libnvpair::ContextType::U16Arr(u16vec) =
                        &mut self.input.curr_nvpair.pair_value
                    {
                        if self.first {
                            self.first = false;
                            self.size = u16vec.len();
                        }
                        visitor.visit_u16(u16vec[curr])
                    } else {
                        Err(libnvpair::NvListError::UnmatchingVariables)
                    }
                }
                libnvpair::NvPairType::Uint16 => {
                    dbg!("Deserializing u16");
                    /*let mut x = 0;
                    let val: *mut u16 = &mut x;
                    dbg!(self.nvpair);
                    dbg!(unsafe { *self.nvpair });
                    NvListError::from_nvlist_rc(unsafe {
                        sys::nvpair_value_uint16(self.nvpair, val)
                    })?;*/

                    libnvpair::nvpair_value_uint16(&mut self.input.curr_nvpair)?;
                    if let libnvpair::ContextType::U16(u16val) = self.input.curr_nvpair.pair_value {
                        dbg!(u16val);
                        visitor.visit_u16(u16val)
                    } else {
                        Err(libnvpair::NvListError::UnmatchingVariables)
                    }
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing u32");
        /*let mut x = 0;
        let val: *mut u32 = &mut x;
        dbg!(self.nvpair);
        dbg!(unsafe { *self.nvpair });
        NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_uint32(self.nvpair, val) })?;
        */

        libnvpair::nvpair_value_uint32(&mut self.input.curr_nvpair)?;

        if let libnvpair::ContextType::U32(u32val) = self.input.curr_nvpair.pair_value {
            dbg!(u32val);
            visitor.visit_u32(u32val)
        } else {
            Err(libnvpair::NvListError::UnmatchingVariables)
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
        //let mut mystr: *mut u8 = std::ptr::null_mut();
        //let mut mystr_ptr: *mut *mut u8 = &mut mystr;
        //NvListError::from_nvlist_rc(unsafe { sys::nvpair_value_string(self.nvpair, mystr_ptr) })?;
        //dbg!(unsafe { CStr::from_ptr(*mystr_ptr).to_str()? });
        libnvpair::nvpair_value_string(&mut self.input.curr_nvpair)?;
        /*match &mut self.input.curr_nvpair.pair_value {
            libnvpair::ContextType::Str(strval) => visitor.visit_borrowed_str(strval.as_str()),
            _ => Err(libnvpair::NvListError::UnmatchingVariables),
        }*/
        dbg!(&mut self.input.curr_nvpair.pair_value);
        if let libnvpair::ContextType::Str(strval) = &mut self.input.curr_nvpair.pair_value {
            visitor.visit_str(strval)
        } else {
            Err(libnvpair::NvListError::UnmatchingVariables)
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
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
        let value = visitor.visit_seq(CommaSeparated::new(&mut self, self.input.into_iter()))?;

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
        let value = visitor.visit_map(CommaSeparated::new(&mut self, self.input.into_iter()))?;
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
        dbg!(libnvpair::nvpair_name(&mut self.input.curr_nvpair)?.as_str());
        visitor.visit_str(libnvpair::nvpair_name(&mut self.input.curr_nvpair)?.as_str())
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
    iter: NvListIterator,
}

impl<'a, 'de> CommaSeparated<'a, 'de> {
    fn new(de: &'a mut NvListDeserializer<'de>, iter: NvListIterator) -> Self {
        CommaSeparated { de, iter }
    }
}

impl<'de, 'a> SeqAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = libnvpair::NvListError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        dbg!("Deserializing seq in SeqAccess");
        if self.de.first {
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
    type Error = libnvpair::NvListError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        dbg!("Deserializing map key");
        match self.iter.next() {
            Some(_) => seed.deserialize(&mut *self.de).map(Some),
            None => {
                self.de.input.curr_nvpair = NvPair::default();
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
    _de: &'a mut NvListDeserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn _new(_de: &'a mut NvListDeserializer<'de>) -> Self {
        Enum { _de }
    }
}

impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = libnvpair::NvListError;
    type Variant = Self;

    fn variant_seed<V>(self, _seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        todo!();
    }
}

impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = libnvpair::NvListError;

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
    use super::*;
    use libnvpair::NvFlag;
    use serde::Deserialize;

    #[test]
    fn basic_de() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: u16,
            b: u32,
        }
        let expected = Test { a: 3, b: 5 };
        let mut nvlist = libnvpair::nvlist_alloc(NvFlag::UniqueName).unwrap();
        libnvpair::nvlist_add_uint16(&nvlist, "a", 3).unwrap();
        libnvpair::nvlist_add_uint32(&nvlist, "b", 5).unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn with_string_de() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: u16,
            b: u32,
            c: String,
        }

        let expected = Test {
            a: 3,
            b: 5,
            c: "test".to_string(),
        };
        let mut nvlist = libnvpair::nvlist_alloc(NvFlag::UniqueName).unwrap();
        libnvpair::nvlist_add_uint16(&nvlist, "a", 3).unwrap();
        libnvpair::nvlist_add_uint32(&nvlist, "b", 5).unwrap();
        libnvpair::nvlist_add_string(&nvlist, "c", "test").unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn with_vec_de() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: u16,
            b: u32,
            c: String,
            d: Vec<u16>,
        }

        let expected = Test {
            a: 3,
            b: 5,
            c: "test".to_string(),
            d: vec![1, 2, 3, 4, 5],
        };
        let arr: [u16; 5] = [1, 2, 3, 4, 5];
        let mut nvlist = libnvpair::nvlist_alloc(NvFlag::UniqueName).unwrap();
        libnvpair::nvlist_add_uint16(&nvlist, "a", 3).unwrap();
        libnvpair::nvlist_add_uint32(&nvlist, "b", 5).unwrap();
        libnvpair::nvlist_add_string(&nvlist, "c", "test").unwrap();
        libnvpair::nvlist_add_uint16_arr(&nvlist, "d", arr).unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn with_tuple_de() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: u16,
            b: u32,
            c: String,
            d: Vec<u16>,
            e: (u16, u16, u16),
        }

        let expected = Test {
            a: 3,
            b: 5,
            c: "test".to_string(),
            d: vec![1, 2, 3, 4, 5],
            e: (1, 2, 3),
        };
        let arr: [u16; 5] = [1, 2, 3, 4, 5];
        let tup: [u16; 3] = [1, 2, 3];
        let mut nvlist = libnvpair::nvlist_alloc(NvFlag::UniqueName).unwrap();
        libnvpair::nvlist_add_uint16(&nvlist, "a", 3).unwrap();
        libnvpair::nvlist_add_uint32(&nvlist, "b", 5).unwrap();
        libnvpair::nvlist_add_string(&nvlist, "c", "test").unwrap();
        libnvpair::nvlist_add_uint16_arr(&nvlist, "d", arr).unwrap();
        libnvpair::nvlist_add_uint16_arr(&nvlist, "e", tup).unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }
}
