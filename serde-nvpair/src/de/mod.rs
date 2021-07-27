use std::convert::TryInto;
use std::usize;

use self::ctx_type_deserializer::CtxTypeDeserializer;
use super::*;
use libnvpair::{ContextType, CtxIter, NvList, NvListError, NvListIterator, NvPair};
use serde::de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::Deserialize;

mod ctx_type_deserializer;

pub struct NvListDeserializer<'de> {
    input: &'de mut libnvpair::NvList,
    nested_nvlist: Option<NvList>,
    curr_pair: NvPair,
}

impl<'de> NvListDeserializer<'de> {
    pub fn from_nvlist(input: &'de mut libnvpair::NvList) -> Self {
        NvListDeserializer {
            input,
            nested_nvlist: None,
            curr_pair: NvPair::default(),
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

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing i8 start function");
        unsafe {
            match match self.curr_pair.raw_nvpair.as_ref() {
                Some(_) => {
                    dbg!("pointer exists");
                    dbg!(self.curr_pair.r#type());
                    self.curr_pair.r#type()
                }
                None => todo!(),
            } {
                libnvpair::NvPairType::Int8 => {
                    dbg!("Deserializing i8");
                    let val = self.curr_pair.value_int8()?;
                    dbg!(val);
                    visitor.visit_i8(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing i16 start function");
        unsafe {
            match match self.curr_pair.raw_nvpair.as_ref() {
                Some(_) => {
                    dbg!("pointer exists");
                    dbg!(self.curr_pair.r#type());
                    self.curr_pair.r#type()
                }
                None => todo!(),
            } {
                libnvpair::NvPairType::Int16 => {
                    dbg!("Deserializing i8");
                    let val = self.curr_pair.value_int16()?;
                    dbg!(val);
                    visitor.visit_i16(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing i32 start function");
        unsafe {
            match match self.curr_pair.raw_nvpair.as_ref() {
                Some(_) => {
                    dbg!("pointer exists");
                    dbg!(self.curr_pair.r#type());
                    self.curr_pair.r#type()
                }
                None => todo!(),
            } {
                libnvpair::NvPairType::Int32 => {
                    dbg!("Deserializing i32");
                    let val = self.curr_pair.value_int32()?;
                    dbg!(val);
                    visitor.visit_i32(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing i64 start function");
        unsafe {
            match match self.curr_pair.raw_nvpair.as_ref() {
                Some(_) => {
                    dbg!("pointer exists");
                    dbg!(self.curr_pair.r#type());
                    self.curr_pair.r#type()
                }
                None => todo!(),
            } {
                libnvpair::NvPairType::Int64 => {
                    dbg!("Deserializing i64");
                    let val = self.curr_pair.value_int64()?;
                    dbg!(val);
                    visitor.visit_i64(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing u8 start function");
        unsafe {
            match match self.curr_pair.raw_nvpair.as_ref() {
                Some(_) => {
                    dbg!("pointer exists");
                    dbg!(self.curr_pair.r#type());
                    self.curr_pair.r#type()
                }
                None => todo!(),
            } {
                libnvpair::NvPairType::Uint8 => {
                    dbg!("Deserializing u8");
                    let val = self.curr_pair.value_uint8()?;
                    dbg!(val);
                    visitor.visit_u8(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing u16 start function");
        unsafe {
            match match self.curr_pair.raw_nvpair.as_ref() {
                Some(_) => {
                    dbg!("pointer exists");
                    dbg!(self.curr_pair.r#type());
                    self.curr_pair.r#type()
                }
                None => todo!(),
            } {
                libnvpair::NvPairType::Uint16 => {
                    dbg!("Deserializing u16");
                    let val = self.curr_pair.value_uint16()?;
                    dbg!(val);
                    visitor.visit_u16(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing u32 start function");
        unsafe {
            match match self.curr_pair.raw_nvpair.as_ref() {
                Some(_) => {
                    dbg!("pointer exists");
                    dbg!(self.curr_pair.r#type());
                    self.curr_pair.r#type()
                }
                None => todo!(),
            } {
                libnvpair::NvPairType::Uint32 => {
                    dbg!("Deserializing u32");
                    let val = self.curr_pair.value_uint32()?;
                    dbg!(val);
                    visitor.visit_u32(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing u64 start function");
        unsafe {
            match match self.curr_pair.raw_nvpair.as_ref() {
                Some(_) => {
                    dbg!("pointer exists");
                    dbg!(self.curr_pair.r#type());
                    self.curr_pair.r#type()
                }
                None => todo!(),
            } {
                libnvpair::NvPairType::Uint64 => {
                    dbg!("Deserializing u64");
                    let val = self.curr_pair.value_uint64()?;
                    dbg!(val);
                    visitor.visit_u64(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        }
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
        let val = self.curr_pair.value_string()?;
        dbg!(&mut self.curr_pair);
        visitor.visit_str(val.as_ref())
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
        match self.curr_pair.r#type() {
            //libnvpair::NvPairType::ByteArray => todo!(),
            libnvpair::NvPairType::Uint16Array
            | libnvpair::NvPairType::Int16Array
            | libnvpair::NvPairType::Int32Array
            | libnvpair::NvPairType::Uint32Array
            | libnvpair::NvPairType::Int64Array
            | libnvpair::NvPairType::Uint64Array
            | libnvpair::NvPairType::StringArray
            | libnvpair::NvPairType::NvlistArray
            | libnvpair::NvPairType::Int8Array
            | libnvpair::NvPairType::Uint8Array => {
                // TODO: check it it is ok?
                dbg!("in arr");
                let mut iter: CtxIter<ContextType> = self.curr_pair.try_into()?;
                let value = visitor.visit_seq(NvSeqAnalyzer::new(&mut self, iter))?;
                Ok(value)
            }
            _ => {
                dbg!("in None");
                Err(NvListError::UnmatchingVariables)
            }
        }
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
        // TODO: check if this is ok
        match self.nested_nvlist {
            Some(nvlist) => {
                dbg!("deserializing nested");
                let nvlist_clone = nvlist.clone();
                let value =
                    visitor.visit_map(CommaSeparated::new(&mut self, nvlist_clone.into_iter()))?;
                Ok(value)
            }
            None => {
                dbg!("deserializing input");
                let nvlist_clone = self.input.clone();
                let value =
                    visitor.visit_map(CommaSeparated::new(&mut self, nvlist_clone.into_iter()))?;
                Ok(value)
            }
        }
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
        dbg!(self.curr_pair.name()?.as_str());
        visitor.visit_str(self.curr_pair.name()?.as_str())
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

struct NvSeqAnalyzer<'a, 'de: 'a> {
    de: &'a mut NvListDeserializer<'de>,
    nvpair_iter: CtxIter<ContextType>,
}

impl<'a, 'de> NvSeqAnalyzer<'a, 'de> {
    fn new(de: &'a mut NvListDeserializer<'de>, nvpair_iter: CtxIter<ContextType>) -> Self {
        NvSeqAnalyzer { de, nvpair_iter }
    }
}

impl<'de, 'a> SeqAccess<'de> for NvSeqAnalyzer<'a, 'de> {
    type Error = libnvpair::NvListError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        dbg!("Deserializing seq in SeqAccess");
        match self.nvpair_iter.next() {
            Some(x) => seed
                .deserialize(CtxTypeDeserializer::from_ctx_type(x))
                .map(Some),
            None => Ok(None),
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
            Some(nvpair) => {
                dbg!("getting some");
                self.de.curr_pair = nvpair;
                seed.deserialize(&mut *self.de).map(Some)
            }
            None => {
                dbg!("getting none");
                self.de.curr_pair = NvPair::default();
                self.de.nested_nvlist = None;
                Ok(None)
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        dbg!("Deserializing map value");
        if self.de.curr_pair.r#type() == libnvpair::NvPairType::Nvlist {
            self.de.nested_nvlist = Some(self.de.curr_pair.value_nvlist()?)
        }

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
    fn struct_u8() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: u8,
            b: u8,
            c: u8,
        }
        let expected = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_uint8("a", 3).unwrap();
        nvlist.add_uint8("b", 5).unwrap();
        nvlist.add_uint8("c", 7).unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_u16() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: u16,
            b: u16,
            c: u16,
        }
        let expected = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_uint16("a", 3).unwrap();
        nvlist.add_uint16("b", 5).unwrap();
        nvlist.add_uint16("c", 7).unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_u32() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: u32,
            b: u32,
            c: u32,
        }
        let expected = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_uint32("a", 3).unwrap();
        nvlist.add_uint32("b", 5).unwrap();
        nvlist.add_uint32("c", 7).unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_u64() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: u64,
            b: u64,
            c: u64,
        }
        let expected = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_uint64("a", 3).unwrap();
        nvlist.add_uint64("b", 5).unwrap();
        nvlist.add_uint64("c", 7).unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_i8() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: i8,
            b: i8,
            c: i8,
        }
        let expected = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_int8("a", 3).unwrap();
        nvlist.add_int8("b", 5).unwrap();
        nvlist.add_int8("c", 7).unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_i16() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: i16,
            b: i16,
            c: i16,
        }
        let expected = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_int16("a", 3).unwrap();
        nvlist.add_int16("b", 5).unwrap();
        nvlist.add_int16("c", 7).unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_i32() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: i32,
            b: i32,
            c: i32,
        }
        let expected = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_int32("a", 3).unwrap();
        nvlist.add_int32("b", 5).unwrap();
        nvlist.add_int32("c", 7).unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_i64() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: i64,
            b: i64,
            c: i64,
        }
        let expected = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_int64("a", 3).unwrap();
        nvlist.add_int64("b", 5).unwrap();
        nvlist.add_int64("c", 7).unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_strings() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: String,
            b: String,
            c: String,
        }
        let expected = Test {
            a: "test1".to_string(),
            b: "test2".to_string(),
            c: "test3".to_string(),
        };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_string("a", "test1").unwrap();
        nvlist.add_string("b", "test2").unwrap();
        nvlist.add_string("c", "test3").unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_mix() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: u8,
            b: u16,
            c: u32,
            d: u64,
            e: i8,
            f: i16,
            g: i32,
            h: i64,
            i: String,
        }

        let expected = Test {
            a: 3,
            b: 5,
            c: 7,
            d: 11,
            e: 13,
            f: 17,
            g: 19,
            h: 23,
            i: "test".to_string(),
        };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_uint8("a", 3).unwrap();
        nvlist.add_uint16("b", 5).unwrap();
        nvlist.add_uint32("c", 7).unwrap();
        nvlist.add_uint64("d", 11).unwrap();
        nvlist.add_int8("e", 13).unwrap();
        nvlist.add_int16("f", 17).unwrap();
        nvlist.add_int32("g", 19).unwrap();
        nvlist.add_int64("h", 23).unwrap();
        nvlist.add_int64("h", 23).unwrap();
        nvlist.add_string("i", "test").unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn basic_vec_u8() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: Vec<u8>,
            b: Vec<u8>,
            c: Vec<u8>,
        }
        let expected = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let arr1: [u8; 5] = [1, 2, 3, 4, 5];
        let arr2: [u8; 5] = [6, 7, 8, 9, 10];
        let arr3: [u8; 5] = [11, 12, 13, 14, 15];
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_uint8_arr("a", arr1).unwrap();
        nvlist.add_uint8_arr("b", arr2).unwrap();
        nvlist.add_uint8_arr("c", arr3).unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn basic_vec_u16() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: Vec<u16>,
            b: Vec<u16>,
            c: Vec<u16>,
        }
        let expected = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let arr1: [u16; 5] = [1, 2, 3, 4, 5];
        let arr2: [u16; 5] = [6, 7, 8, 9, 10];
        let arr3: [u16; 5] = [11, 12, 13, 14, 15];
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_uint16_arr("a", arr1).unwrap();
        nvlist.add_uint16_arr("b", arr2).unwrap();
        nvlist.add_uint16_arr("c", arr3).unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_nested() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Nested {
            a: u16,
            b: u16,
        }
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: Nested,
            b: u16,
        }
        let expected = Test {
            a: Nested { a: 3, b: 5 },
            b: 6,
        };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        let mut nvlist_nested = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist_nested.add_uint16("a", 3).unwrap();
        nvlist_nested.add_uint16("b", 5).unwrap();
        nvlist.add_nvlist("a", &nvlist_nested).unwrap();
        nvlist.add_uint16("b", 6).unwrap();

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
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_uint16("a", 3).unwrap();
        nvlist.add_uint32("b", 5).unwrap();
        nvlist.add_string("c", "test").unwrap();

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
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_uint16("a", 3).unwrap();
        nvlist.add_uint32("b", 5).unwrap();
        nvlist.add_string("c", "test").unwrap();
        nvlist.add_uint16_arr("d", arr).unwrap();

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
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_uint16("a", 3).unwrap();
        nvlist.add_uint32("b", 5).unwrap();
        nvlist.add_string("c", "test").unwrap();
        nvlist.add_uint16_arr("d", arr).unwrap();
        nvlist.add_uint16_arr("e", tup).unwrap();

        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }
}
