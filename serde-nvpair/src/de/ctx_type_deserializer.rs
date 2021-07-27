use super::*;
use libnvpair::ContextType;

pub struct CtxTypeDeserializer {
    input: ContextType,
}

impl<'de> CtxTypeDeserializer {
    pub fn from_ctx_type(input: ContextType) -> Self {
        CtxTypeDeserializer { input }
    }
}

impl<'de, 'a> de::Deserializer<'de> for CtxTypeDeserializer {
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
        dbg!("Deserializing bool in ctx_type");
        dbg!(&self.input);
        match self.input {
            ContextType::Boolean(x) => visitor.visit_bool(x),
            _ => Err(NvListError::UnmatchingVariables),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing i8 in ctx_type");
        dbg!(&self.input);
        match self.input {
            ContextType::I8(x) => visitor.visit_i8(x),
            _ => Err(NvListError::UnmatchingVariables),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing i16 in ctx_type");
        dbg!(&self.input);
        match self.input {
            ContextType::I16(x) => visitor.visit_i16(x),
            _ => Err(NvListError::UnmatchingVariables),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing i32 in ctx_type");
        dbg!(&self.input);
        match self.input {
            ContextType::I32(x) => visitor.visit_i32(x),
            _ => Err(NvListError::UnmatchingVariables),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing i64 in ctx_type");
        dbg!(&self.input);
        match self.input {
            ContextType::I64(x) => visitor.visit_i64(x),
            _ => Err(NvListError::UnmatchingVariables),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing u16 in ctx_type");
        dbg!(&self.input);
        match self.input {
            ContextType::U8(x) => visitor.visit_u8(x),
            _ => Err(NvListError::UnmatchingVariables),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing u16 in ctx_type");
        dbg!(&self.input);
        match self.input {
            ContextType::U16(x) => visitor.visit_u16(x),
            _ => Err(NvListError::UnmatchingVariables),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing u32 in ctx_type");
        dbg!(&self.input);
        match self.input {
            ContextType::U32(x) => visitor.visit_u32(x),
            _ => Err(NvListError::UnmatchingVariables),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing u64 in ctx_type");
        dbg!(&self.input);
        match self.input {
            ContextType::U64(x) => visitor.visit_u64(x),
            _ => Err(NvListError::UnmatchingVariables),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing f32 in ctx_type");
        self.deserialize_f64(visitor)
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
        todo!();
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("Deserializing string in ctx_type");
        dbg!(&self.input);
        match self.input {
            ContextType::Str(x) => visitor.visit_string(x),
            _ => Err(NvListError::UnmatchingVariables),
        }
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
        todo!();
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
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
        todo!();
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
        todo!();
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
        todo!();
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
    }
}

struct CommaSeparated<'a> {
    de: &'a mut CtxTypeDeserializer,
    iter: NvListIterator,
}

impl<'a, 'de> CommaSeparated<'a> {
    fn new(de: &'a mut CtxTypeDeserializer, iter: NvListIterator) -> Self {
        CommaSeparated { de, iter }
    }
}

struct NvSeqAnalyzer<'a> {
    de: &'a mut CtxTypeDeserializer,
    nvpair_iter: CtxIter<ContextType>,
}

impl<'a, 'de> NvSeqAnalyzer<'a> {
    fn new(de: &'a mut CtxTypeDeserializer, nvpair_iter: CtxIter<ContextType>) -> Self {
        NvSeqAnalyzer { de, nvpair_iter }
    }
}

impl<'de, 'a> SeqAccess<'de> for NvSeqAnalyzer<'a> {
    type Error = libnvpair::NvListError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        todo!();
    }
}

impl<'de, 'a> MapAccess<'de> for CommaSeparated<'a> {
    type Error = libnvpair::NvListError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        todo!();
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        todo!();
    }
}

struct Enum<'a> {
    _de: &'a mut CtxTypeDeserializer,
}

impl<'a, 'de> Enum<'a> {
    fn _new(_de: &'a mut CtxTypeDeserializer) -> Self {
        Enum { _de }
    }
}

impl<'de, 'a> EnumAccess<'de> for Enum<'a> {
    type Error = libnvpair::NvListError;
    type Variant = Self;

    fn variant_seed<V>(self, _seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        todo!();
    }
}

impl<'de, 'a> VariantAccess<'de> for Enum<'a> {
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
