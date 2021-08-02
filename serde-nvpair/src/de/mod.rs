use std::convert::TryInto;
use std::usize;

use self::ctx_type_deserializer::CtxTypeDeserializer;
use super::*;
use libnvpair::{ContextType, CtxIter, NvList, NvListError, NvListIterator, NvPair};
use serde::de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::Deserialize;

mod ctx_type_deserializer;

#[derive(Clone, Debug, PartialEq)]
pub struct HelperDeserializer {
    nvlist: NvList,
    fields: &'static [&'static str],
}

impl HelperDeserializer {
    pub fn default() -> Self {
        HelperDeserializer {
            nvlist: NvList::new(),
            fields: &[],
        }
    }
}

pub struct NvListDeserializer<'de> {
    input: &'de mut libnvpair::NvList,
    curr_pair: Option<NvPair>,
    curr: HelperDeserializer,
    helpers: Vec<HelperDeserializer>,
    first: bool,
}

impl<'de> NvListDeserializer<'de> {
    pub fn from_nvlist(input: &'de mut libnvpair::NvList) -> Self {
        NvListDeserializer {
            input,
            curr_pair: None,
            helpers: Vec::new(),
            first: true,
            curr: HelperDeserializer::default(),
        }
    }
}

pub fn from_nvlist<'a, T>(s: &'a mut libnvpair::NvList) -> Result<T>
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

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing boolean start function");
        if let Some(nvpair) = &self.curr_pair {
            match nvpair.r#type()? {
                libnvpair::NvPairType::BooleanValue => {
                    dbg!("Deserializing boolean");
                    let val = nvpair.value_boolean()?;
                    dbg!(val);
                    visitor.visit_bool(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        } else {
            Err(NvListError::NvPairDontExist)
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing i8 start function");
        if let Some(nvpair) = &self.curr_pair {
            match nvpair.r#type()? {
                libnvpair::NvPairType::Int8 => {
                    dbg!("Deserializing i8");
                    let val = nvpair.value_int8()?;
                    dbg!(val);
                    visitor.visit_i8(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        } else {
            Err(NvListError::NvPairDontExist)
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing i16 start function");
        if let Some(nvpair) = &self.curr_pair {
            match nvpair.r#type()? {
                libnvpair::NvPairType::Int16 => {
                    dbg!("Deserializing i8");
                    let val = nvpair.value_int16()?;
                    dbg!(val);
                    visitor.visit_i16(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        } else {
            Err(NvListError::NvPairDontExist)
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing i32 start function");
        if let Some(nvpair) = &self.curr_pair {
            match nvpair.r#type()? {
                libnvpair::NvPairType::Int32 => {
                    dbg!("Deserializing i32");
                    let val = nvpair.value_int32()?;
                    dbg!(val);
                    visitor.visit_i32(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        } else {
            Err(NvListError::NvPairDontExist)
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing i64 start function");
        if let Some(nvpair) = &self.curr_pair {
            match nvpair.r#type()? {
                libnvpair::NvPairType::Int64 => {
                    dbg!("Deserializing i64");
                    let val = nvpair.value_int64()?;
                    dbg!(val);
                    visitor.visit_i64(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        } else {
            Err(NvListError::NvPairDontExist)
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing u8 start function");
        if let Some(nvpair) = &self.curr_pair {
            match nvpair.r#type()? {
                libnvpair::NvPairType::Uint8 => {
                    dbg!("Deserializing u8");
                    let val = nvpair.value_uint8()?;
                    dbg!(val);
                    visitor.visit_u8(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        } else {
            Err(NvListError::NvPairDontExist)
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing u16 start function");
        if let Some(nvpair) = &self.curr_pair {
            match nvpair.r#type()? {
                libnvpair::NvPairType::Uint16 => {
                    dbg!("Deserializing u16");
                    let val = nvpair.value_uint16()?;
                    dbg!(val);
                    visitor.visit_u16(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        } else {
            Err(NvListError::NvPairDontExist)
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing u32 start function");
        if let Some(nvpair) = &self.curr_pair {
            match nvpair.r#type()? {
                libnvpair::NvPairType::Uint32 => {
                    dbg!("Deserializing u32");
                    let val = nvpair.value_uint32()?;
                    dbg!(val);
                    visitor.visit_u32(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        } else {
            Err(NvListError::NvPairDontExist)
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing u64 start function");
        if let Some(nvpair) = &self.curr_pair {
            match nvpair.r#type()? {
                libnvpair::NvPairType::Uint64 => {
                    dbg!("Deserializing u64");
                    let val = nvpair.value_uint64()?;
                    dbg!(val);
                    visitor.visit_u64(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        } else {
            Err(NvListError::NvPairDontExist)
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("deserializing f64 start function");
        if let Some(nvpair) = &self.curr_pair {
            match nvpair.r#type()? {
                libnvpair::NvPairType::Double => {
                    dbg!("Deserializing u64");
                    let val = nvpair.value_float64()?;
                    dbg!(val);
                    visitor.visit_f64(val)
                }
                _ => Err(libnvpair::NvListError::InvalidArgument),
            }
        } else {
            Err(NvListError::NvPairDontExist)
        }
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
        if let Some(nvpair) = &self.curr_pair {
            let val = nvpair.value_string()?;
            dbg!(&mut self.curr_pair);
            visitor.visit_str(val.as_ref())
        } else {
            Err(NvListError::NvPairDontExist)
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
        if let Some(nvpair) = self.curr_pair {
            match nvpair.r#type()? {
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
                | libnvpair::NvPairType::Uint8Array
                | libnvpair::NvPairType::BooleanArray => {
                    // TODO: check it it is ok?
                    dbg!("in arr");
                    let iter: CtxIter<ContextType> = nvpair.try_into()?;
                    let value = visitor.visit_seq(NvSeqAnalyzer::new(&mut self, iter))?;
                    Ok(value)
                }
                _ => {
                    dbg!("in None");
                    Err(NvListError::UnmatchingVariables)
                }
            }
        } else {
            Err(NvListError::NvPairDontExist)
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
        let nvlist_clone = self.curr.nvlist.clone();
        let value = visitor.visit_map(CommaSeparated::new(&mut self, nvlist_clone.into_iter()))?;
        Ok(value)
        // TODO: check if this is ok
        /*match self.nested_nvlist {
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
        }*/
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing struct");

        if self.first {
            self.first = false;
            self.helpers.push(HelperDeserializer {
                nvlist: self.input.to_owned(),
                fields,
            })
        }

        if let Some(last) = self.helpers.pop() {
            self.curr = last;
            self.curr.fields = fields;
        } else {
            return Err(NvListError::RestrictedOperation);
        }

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
        if let Some(nvpair) = &self.curr_pair {
            dbg!(nvpair.r#type()?);
            dbg!(nvpair.name());
            visitor.visit_str(nvpair.name().as_ref())
        } else {
            Err(NvListError::NvPairDontExist)
        }
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
    finished: bool,
}

impl<'a, 'de> CommaSeparated<'a, 'de> {
    fn new(de: &'a mut NvListDeserializer<'de>, iter: NvListIterator) -> Self {
        CommaSeparated {
            de,
            iter,
            finished: false,
        }
    }
}

struct NvSeqAnalyzer<'a, 'de: 'a> {
    _de: &'a mut NvListDeserializer<'de>,
    nvpair_iter: CtxIter<ContextType>,
}

impl<'a, 'de> NvSeqAnalyzer<'a, 'de> {
    fn new(_de: &'a mut NvListDeserializer<'de>, nvpair_iter: CtxIter<ContextType>) -> Self {
        NvSeqAnalyzer { _de, nvpair_iter }
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
            Some(x) => {
                dbg!("in some");
                seed.deserialize(CtxTypeDeserializer::from(x)).map(Some)
            }
            None => {
                dbg!("in none");
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

        loop {
            match self.iter.next() {
                Some(nvpair) => {
                    dbg!("getting some");
                    dbg!("struct fields: ", &self.de.curr.fields);
                    let name = nvpair.name();
                    let name = name.as_ref();
                    dbg!("current struct field: ", &name);
                    if self.de.curr.fields.contains(&name) {
                        self.de.curr_pair = Some(nvpair);
                        break;
                    } else {
                        dbg!("name not found: ", name)
                    }
                }
                None => {
                    dbg!("getting none");
                    if let Some(last) = self.de.helpers.pop() {
                        self.de.curr = last;
                    }

                    self.de.curr_pair = None;
                    self.finished = true;
                    break;
                }
            };
        }
        if !self.finished {
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        dbg!("Deserializing map value");
        if let Some(nvpair) = &self.de.curr_pair {
            if nvpair.r#type()? == libnvpair::NvPairType::Nvlist {
                self.de.helpers.push(self.de.curr.to_owned());
                self.de.helpers.push(HelperDeserializer {
                    nvlist: nvpair.value_nvlist()?,
                    fields: &[],
                })
            }
        } else {
            return Err(NvListError::NvPairDontExist);
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
    fn struct_f32() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: f32,
            b: f32,
            c: f32,
        }
        let expected = Test {
            a: 3.5,
            b: 5.9,
            c: 4.8,
        };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_float64("a", 3.5).unwrap();
        nvlist.add_float64("b", 5.9).unwrap();
        nvlist.add_float64("c", 4.8).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_f64() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: f64,
            b: f64,
            c: f64,
        }
        let expected = Test {
            a: 3.5,
            b: 5.9,
            c: 4.8,
        };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_float64("a", 3.5).unwrap();
        nvlist.add_float64("b", 5.9).unwrap();
        nvlist.add_float64("c", 4.8).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

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

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
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

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
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

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
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

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
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

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
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

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
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

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
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

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
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

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_bools() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: bool,
            b: bool,
            c: bool,
        }
        let expected = Test {
            a: true,
            b: false,
            c: true,
        };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_boolean("a", true).unwrap();
        nvlist.add_boolean("b", false).unwrap();
        nvlist.add_boolean("c", true).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_mix_basic() {
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
            j: bool,
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
            j: false,
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
        nvlist.add_boolean("j", false).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_u8() {
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

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_u16() {
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

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_u32() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: Vec<u32>,
            b: Vec<u32>,
            c: Vec<u32>,
        }
        let expected = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let arr1: [u32; 5] = [1, 2, 3, 4, 5];
        let arr2: [u32; 5] = [6, 7, 8, 9, 10];
        let arr3: [u32; 5] = [11, 12, 13, 14, 15];
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_uint32_arr("a", arr1).unwrap();
        nvlist.add_uint32_arr("b", arr2).unwrap();
        nvlist.add_uint32_arr("c", arr3).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_u64() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: Vec<u64>,
            b: Vec<u64>,
            c: Vec<u64>,
        }
        let expected = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let arr1: [u64; 5] = [1, 2, 3, 4, 5];
        let arr2: [u64; 5] = [6, 7, 8, 9, 10];
        let arr3: [u64; 5] = [11, 12, 13, 14, 15];
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_uint64_arr("a", arr1).unwrap();
        nvlist.add_uint64_arr("b", arr2).unwrap();
        nvlist.add_uint64_arr("c", arr3).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_i8() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: Vec<i8>,
            b: Vec<i8>,
            c: Vec<i8>,
        }
        let expected = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let arr1: [i8; 5] = [1, 2, 3, 4, 5];
        let arr2: [i8; 5] = [6, 7, 8, 9, 10];
        let arr3: [i8; 5] = [11, 12, 13, 14, 15];
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_int8_arr("a", arr1).unwrap();
        nvlist.add_int8_arr("b", arr2).unwrap();
        nvlist.add_int8_arr("c", arr3).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_i16() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: Vec<i16>,
            b: Vec<i16>,
            c: Vec<i16>,
        }
        let expected = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let arr1: [i16; 5] = [1, 2, 3, 4, 5];
        let arr2: [i16; 5] = [6, 7, 8, 9, 10];
        let arr3: [i16; 5] = [11, 12, 13, 14, 15];
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_int16_arr("a", arr1).unwrap();
        nvlist.add_int16_arr("b", arr2).unwrap();
        nvlist.add_int16_arr("c", arr3).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_i32() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: Vec<i32>,
            b: Vec<i32>,
            c: Vec<i32>,
        }
        let expected = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let arr1 = &[1, 2, 3, 4, 5];
        let arr2 = &[6, 7, 8, 9, 10];
        let arr3 = &[11, 12, 13, 14, 15];
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_int32_arr("a", arr1).unwrap();
        nvlist.add_int32_arr("b", arr2).unwrap();
        nvlist.add_int32_arr("c", arr3).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_i64() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: Vec<i64>,
            b: Vec<i64>,
            c: Vec<i64>,
        }
        let expected = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let arr1: [i64; 5] = [1, 2, 3, 4, 5];
        let arr2: [i64; 5] = [6, 7, 8, 9, 10];
        let arr3: [i64; 5] = [11, 12, 13, 14, 15];
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_int64_arr("a", arr1).unwrap();
        nvlist.add_int64_arr("b", arr2).unwrap();
        nvlist.add_int64_arr("c", arr3).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_string() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: Vec<String>,
            b: Vec<String>,
            c: Vec<String>,
        }
        let expected = Test {
            a: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
            ],
            b: vec![
                "f".to_string(),
                "g".to_string(),
                "h".to_string(),
                "i".to_string(),
                "j".to_string(),
            ],
            c: vec![
                "k".to_string(),
                "l".to_string(),
                "m".to_string(),
                "n".to_string(),
                "o".to_string(),
            ],
        };
        let arr1: [String; 5] = [
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];
        let arr2: [String; 5] = [
            "f".to_string(),
            "g".to_string(),
            "h".to_string(),
            "i".to_string(),
            "j".to_string(),
        ];
        let arr3: [String; 5] = [
            "k".to_string(),
            "l".to_string(),
            "m".to_string(),
            "n".to_string(),
            "o".to_string(),
        ];
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_string_arr("a", arr1).unwrap();
        nvlist.add_string_arr("b", arr2).unwrap();
        nvlist.add_string_arr("c", arr3).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_bool() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: Vec<bool>,
            b: Vec<bool>,
            c: Vec<bool>,
        }
        let expected = Test {
            a: vec![true, true, true, true, true],
            b: vec![false, false, false, false, false],
            c: vec![true, true, false, true, true],
        };
        let arr1: [bool; 5] = [true, true, true, true, true];
        let arr2: [bool; 5] = [false, false, false, false, false];
        let arr3: [bool; 5] = [true, true, false, true, true];
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_boolean_arr("a", arr1).unwrap();
        nvlist.add_boolean_arr("b", arr2).unwrap();
        nvlist.add_boolean_arr("c", arr3).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_mix_vec() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: Vec<u8>,
            b: Vec<u16>,
            c: Vec<u32>,
            d: Vec<u64>,
            e: Vec<i8>,
            f: Vec<i16>,
            g: Vec<i32>,
            h: Vec<i64>,
            i: Vec<String>,
            j: Vec<bool>,
        }
        let expected = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
            d: vec![16, 17, 18, 19, 20],
            e: vec![21, 22, 23, 24, 25],
            f: vec![26, 27, 28, 29, 30],
            g: vec![31, 32, 33, 34, 35],
            h: vec![36, 37, 38, 39, 40],
            i: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
            ],
            j: vec![true, true, false, true, true],
        };
        let arr1 = &[1, 2, 3, 4, 5];
        let arr2 = &[6, 7, 8, 9, 10];
        let arr3 = &[11, 12, 13, 14, 15];
        let arr4 = &[16, 17, 18, 19, 20];
        let arr5 = &[21, 22, 23, 24, 25];
        let arr6 = &[26, 27, 28, 29, 30];
        let arr7 = &[31, 32, 33, 34, 35];
        let arr8 = &[36, 37, 38, 39, 40];
        let arr9 = &[
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];
        let arr10: [bool; 5] = [true, true, false, true, true];

        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_uint8_arr("a", arr1).unwrap();
        nvlist.add_uint16_arr("b", arr2).unwrap();
        nvlist.add_uint32_arr("c", arr3).unwrap();
        nvlist.add_uint64_arr("d", arr4).unwrap();
        nvlist.add_int8_arr("e", arr5).unwrap();
        nvlist.add_int16_arr("f", arr6).unwrap();
        nvlist.add_int32_arr("g", arr7).unwrap();
        nvlist.add_int64_arr("h", arr8).unwrap();
        nvlist.add_string_arr("i", arr9).unwrap();
        nvlist.add_boolean_arr("j", arr10).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_mix() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: Vec<u8>,
            b: Vec<u16>,
            c: Vec<u32>,
            d: Vec<u64>,
            e: Vec<i8>,
            f: Vec<i16>,
            g: Vec<i32>,
            h: Vec<i64>,
            i: Vec<String>,
            j: Vec<bool>,
            k: u8,
            l: u16,
            m: u32,
            n: u64,
            o: i8,
            p: i16,
            q: i32,
            r: i64,
            s: String,
            t: bool,
        }
        let expected = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
            d: vec![16, 17, 18, 19, 20],
            e: vec![21, 22, 23, 24, 25],
            f: vec![26, 27, 28, 29, 30],
            g: vec![31, 32, 33, 34, 35],
            h: vec![36, 37, 38, 39, 40],
            i: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
            ],
            j: vec![true, true, false, true, true],
            k: 3,
            l: 5,
            m: 7,
            n: 11,
            o: 13,
            p: 17,
            q: 19,
            r: 23,
            s: "test".to_string(),
            t: false,
        };
        let arr1 = &[1, 2, 3, 4, 5];
        let arr2 = &[6, 7, 8, 9, 10];
        let arr3 = &[11, 12, 13, 14, 15];
        let arr4 = &[16, 17, 18, 19, 20];
        let arr5 = &[21, 22, 23, 24, 25];
        let arr6 = &[26, 27, 28, 29, 30];
        let arr7 = &[31, 32, 33, 34, 35];
        let arr8 = &[36, 37, 38, 39, 40];
        let arr9 = &[
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];
        let arr10: [bool; 5] = [true, true, false, true, true];

        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist.add_uint8_arr("a", arr1).unwrap();
        nvlist.add_uint16_arr("b", arr2).unwrap();
        nvlist.add_uint32_arr("c", arr3).unwrap();
        nvlist.add_uint64_arr("d", arr4).unwrap();
        nvlist.add_int8_arr("e", arr5).unwrap();
        nvlist.add_int16_arr("f", arr6).unwrap();
        nvlist.add_int32_arr("g", arr7).unwrap();
        nvlist.add_int64_arr("h", arr8).unwrap();
        nvlist.add_string_arr("i", arr9).unwrap();
        nvlist.add_boolean_arr("j", arr10).unwrap();
        nvlist.add_uint8("k", 3).unwrap();
        nvlist.add_uint16("l", 5).unwrap();
        nvlist.add_uint32("m", 7).unwrap();
        nvlist.add_uint64("n", 11).unwrap();
        nvlist.add_int8("o", 13).unwrap();
        nvlist.add_int16("p", 17).unwrap();
        nvlist.add_int32("q", 19).unwrap();
        nvlist.add_int64("r", 23).unwrap();
        nvlist.add_string("s", "test").unwrap();
        nvlist.add_boolean("t", false).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_nested_depth_two() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Nested {
            a: u16,
            b: u16,
        }
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: u8,
            b: Nested,
            c: u16,
            d: Nested,
            e: u32,
            f: Nested,
        }
        let expected = Test {
            a: 3,
            b: Nested { a: 3, b: 5 },
            c: 6,
            d: Nested { a: 7, b: 9 },
            e: 9,
            f: Nested { a: 11, b: 13 },
        };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        let mut nvlist_nested_first = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        let mut nvlist_nested_second = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        let mut nvlist_nested_third = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist_nested_first.add_uint16("a", 3).unwrap();
        nvlist_nested_first.add_uint16("b", 5).unwrap();
        nvlist_nested_second.add_uint16("a", 7).unwrap();
        nvlist_nested_second.add_uint16("b", 9).unwrap();
        nvlist_nested_third.add_uint16("a", 11).unwrap();
        nvlist_nested_third.add_uint16("b", 13).unwrap();
        nvlist.add_uint8("a", 3).unwrap();
        nvlist.add_nvlist("b", &nvlist_nested_first).unwrap();
        nvlist.add_uint16("c", 6).unwrap();
        nvlist.add_nvlist("d", &nvlist_nested_second).unwrap();
        nvlist.add_uint32("e", 9).unwrap();
        nvlist.add_nvlist("f", &nvlist_nested_third).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_nested_depth_three() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct NestedDepthTwo {
            a: u16,
            b: u16,
        }
        #[derive(Debug, PartialEq, Deserialize)]
        struct NestedDepthOne {
            a: u16,
            b: NestedDepthTwo,
            c: u16,
        }
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: u8,
            b: NestedDepthOne,
            c: u16,
        }
        let expected = Test {
            a: 1,
            b: NestedDepthOne {
                a: 2,
                b: NestedDepthTwo { a: 3, b: 4 },
                c: 5,
            },
            c: 6,
        };
        let mut nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        let mut nvlist_nested_depth_one = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        let mut nvlist_nested_depth_two = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();
        nvlist_nested_depth_two.add_uint16("a", 3).unwrap();
        nvlist_nested_depth_two.add_uint16("b", 4).unwrap();
        nvlist_nested_depth_one.add_uint16("a", 2).unwrap();
        nvlist_nested_depth_one
            .add_nvlist("b", &nvlist_nested_depth_two)
            .unwrap();
        nvlist_nested_depth_one.add_uint16("c", 5).unwrap();
        nvlist.add_uint8("a", 1).unwrap();
        nvlist.add_nvlist("b", &nvlist_nested_depth_one).unwrap();
        nvlist.add_uint16("c", 6).unwrap();

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
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

        assert_eq!(expected, from_nvlist(&mut nvlist).unwrap());
    }
}
