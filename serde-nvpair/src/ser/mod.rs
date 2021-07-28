use self::name_serializer::NameSerializer;
use super::*;
use libnvpair::{ContextType, NvFlag, NvList, NvListError};
use serde::{ser, Serialize};
use uuid::Uuid;

mod name_serializer;

#[derive(Clone, Debug, PartialEq)]
pub struct NvListSerializer {
    raw_nvlist: libnvpair::NvList,
    context_type: libnvpair::ContextType,
    name_serializer: NameSerializer,
    is_vec: bool,
    name: Option<String>,
}

impl NvListSerializer {
    fn render_name(&self) -> String {
        Uuid::new_v4().to_string()
    }

    fn _to_nvlist<T>(value: &T) -> Result<libnvpair::NvList>
    where
        T: Serialize,
    {
        let nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();

        let mut serializer = NvListSerializer {
            raw_nvlist: nvlist,
            name: None,
            context_type: libnvpair::ContextType::Empty,
            name_serializer: NameSerializer {
                name: String::new(),
            },
            is_vec: false,
        };
        value.serialize(&mut serializer)?;
        Ok(serializer.raw_nvlist)
    }
}

pub fn _to_nvlist<T>(value: &T) -> Result<libnvpair::NvList>
where
    T: Serialize,
{
    let nvlist = NvList::nvlist_alloc(NvFlag::UniqueName).unwrap();

    let mut serializer = NvListSerializer {
        raw_nvlist: nvlist,
        name: None,
        context_type: libnvpair::ContextType::Empty,
        name_serializer: NameSerializer {
            name: String::new(),
        },
        is_vec: false,
    };

    value.serialize(&mut serializer)?;
    Ok(serializer.raw_nvlist)
}

impl<'a> ser::Serializer for &'a mut NvListSerializer {
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = libnvpair::NvListError;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        dbg!("Serializing bool");
        if self.is_vec {
            match &mut self.context_type {
                libnvpair::ContextType::Empty => {
                    dbg!("in empty context");
                    dbg!(&self.context_type);
                    self.context_type = libnvpair::ContextType::BooleanArr(vec![v]);
                    Ok(())
                }
                libnvpair::ContextType::BooleanArr(x) => {
                    x.push(v);
                    Ok(())
                }
                _ => Err(NvListError::UnmatchingVariables),
            }
        } else {
            if let Some(name) = &self.name {
                self.raw_nvlist.add_boolean(name, v)?;
                Ok(())
            } else {
                Err(NvListError::RestrictedOperation)
            }
        }
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        dbg!("Serializing i8");
        if self.is_vec {
            match &mut self.context_type {
                libnvpair::ContextType::Empty => {
                    dbg!("in empty context");
                    dbg!(&self.context_type);
                    self.context_type = libnvpair::ContextType::I8Arr(vec![v]);
                    Ok(())
                }
                libnvpair::ContextType::I8Arr(x) => {
                    x.push(v);
                    Ok(())
                }
                _ => Err(NvListError::UnmatchingVariables),
            }
        } else {
            if let Some(name) = &self.name {
                self.raw_nvlist.add_int8(name, v)?;
                Ok(())
            } else {
                Err(NvListError::RestrictedOperation)
            }
        }
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        dbg!("Serializing i16");
        if self.is_vec {
            match &mut self.context_type {
                libnvpair::ContextType::Empty => {
                    dbg!("in empty context");
                    dbg!(&self.context_type);
                    self.context_type = libnvpair::ContextType::I16Arr(vec![v]);
                    Ok(())
                }
                libnvpair::ContextType::I16Arr(x) => {
                    x.push(v);
                    Ok(())
                }
                _ => Err(NvListError::UnmatchingVariables),
            }
        } else {
            if let Some(name) = &self.name {
                self.raw_nvlist.add_int16(name, v)?;
                Ok(())
            } else {
                Err(NvListError::RestrictedOperation)
            }
        }
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        dbg!("Serializing i32");
        if self.is_vec {
            match &mut self.context_type {
                libnvpair::ContextType::Empty => {
                    dbg!("in empty context");
                    dbg!(&self.context_type);
                    self.context_type = libnvpair::ContextType::I32Arr(vec![v]);
                    Ok(())
                }
                libnvpair::ContextType::I32Arr(x) => {
                    x.push(v);
                    Ok(())
                }
                _ => Err(NvListError::UnmatchingVariables),
            }
        } else {
            if let Some(name) = &self.name {
                self.raw_nvlist.add_int32(name, v)?;
                Ok(())
            } else {
                Err(NvListError::RestrictedOperation)
            }
        }
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        dbg!("Serializing i64");
        if self.is_vec {
            match &mut self.context_type {
                libnvpair::ContextType::Empty => {
                    dbg!("in empty context");
                    dbg!(&self.context_type);
                    self.context_type = libnvpair::ContextType::I64Arr(vec![v]);
                    Ok(())
                }
                libnvpair::ContextType::I64Arr(x) => {
                    x.push(v);
                    Ok(())
                }
                _ => Err(NvListError::UnmatchingVariables),
            }
        } else {
            if let Some(name) = &self.name {
                self.raw_nvlist.add_int64(name, v)?;
                Ok(())
            } else {
                Err(NvListError::RestrictedOperation)
            }
        }
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        dbg!("Serializing u8");
        if self.is_vec {
            match &mut self.context_type {
                libnvpair::ContextType::Empty => {
                    dbg!("in empty context");
                    dbg!(&self.context_type);
                    self.context_type = libnvpair::ContextType::U8Arr(vec![v]);
                    Ok(())
                }
                libnvpair::ContextType::U8Arr(x) => {
                    x.push(v);
                    Ok(())
                }
                _ => Err(NvListError::UnmatchingVariables),
            }
        } else {
            if let Some(name) = &self.name {
                self.raw_nvlist.add_uint8(name, v)?;
                Ok(())
            } else {
                Err(NvListError::RestrictedOperation)
            }
        }
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        dbg!("Serializing u16");
        if self.is_vec {
            match &mut self.context_type {
                libnvpair::ContextType::Empty => {
                    dbg!("in empty context");
                    dbg!(&self.context_type);
                    self.context_type = libnvpair::ContextType::U16Arr(vec![v]);
                    Ok(())
                }
                libnvpair::ContextType::U16Arr(x) => {
                    x.push(v);
                    Ok(())
                }
                _ => Err(NvListError::UnmatchingVariables),
            }
        } else {
            if let Some(name) = &self.name {
                self.raw_nvlist.add_uint16(name, v)?;
                Ok(())
            } else {
                Err(NvListError::RestrictedOperation)
            }
        }
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        dbg!("Serializing u32");
        if self.is_vec {
            match &mut self.context_type {
                libnvpair::ContextType::Empty => {
                    dbg!("in empty context");
                    dbg!(&self.context_type);
                    self.context_type = libnvpair::ContextType::U32Arr(vec![v]);
                    Ok(())
                }
                libnvpair::ContextType::U32Arr(x) => {
                    x.push(v);
                    Ok(())
                }
                _ => Err(NvListError::UnmatchingVariables),
            }
        } else {
            if let Some(name) = &self.name {
                self.raw_nvlist.add_uint32(name, v)?;
                Ok(())
            } else {
                Err(NvListError::RestrictedOperation)
            }
        }
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        dbg!("Serializing u64");
        if self.is_vec {
            match &mut self.context_type {
                libnvpair::ContextType::Empty => {
                    dbg!("in empty context");
                    dbg!(&self.context_type);
                    self.context_type = libnvpair::ContextType::U64Arr(vec![v]);
                    Ok(())
                }
                libnvpair::ContextType::U64Arr(x) => {
                    x.push(v);
                    Ok(())
                }
                _ => Err(NvListError::UnmatchingVariables),
            }
        } else {
            if let Some(name) = &self.name {
                self.raw_nvlist.add_uint64(name, v)?;
                Ok(())
            } else {
                Err(NvListError::RestrictedOperation)
            }
        }
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        dbg!("Serializing float32");
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        dbg!("Serializing f64");
        if self.is_vec {
            match &mut self.context_type {
                libnvpair::ContextType::Empty => {
                    dbg!("in empty context");
                    dbg!(&self.context_type);
                    self.context_type = libnvpair::ContextType::DoubleArr(vec![v]);
                    Ok(())
                }
                libnvpair::ContextType::DoubleArr(x) => {
                    x.push(v);
                    Ok(())
                }
                _ => Err(NvListError::UnmatchingVariables),
            }
        } else {
            if let Some(name) = &self.name {
                self.raw_nvlist.add_float64(name, v)?;
                Ok(())
            } else {
                Err(NvListError::RestrictedOperation)
            }
        }
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        dbg!("Serializing string");
        if self.is_vec {
            match &mut self.context_type {
                libnvpair::ContextType::Empty => {
                    dbg!("in empty context");
                    dbg!(&self.context_type);
                    self.context_type = libnvpair::ContextType::StrArr(vec![v.to_string()]);
                    Ok(())
                }
                libnvpair::ContextType::StrArr(x) => {
                    x.push(v.to_string());
                    Ok(())
                }
                _ => Err(NvListError::UnmatchingVariables),
            }
        } else {
            if let Some(name) = &self.name {
                self.raw_nvlist.add_string(name, &v.to_string())?;
                Ok(())
            } else {
                Err(NvListError::RestrictedOperation)
            }
        }
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        dbg!("Serializing bytes");
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        dbg!("Serializing seq");
        if !self.is_vec {
            self.is_vec = true;
        } else {
            return Err(NvListError::RestrictedOperation);
        }
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        dbg!("Serializing tuple");
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        dbg!("Serializing tuple struct");
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        dbg!("Serializing tuple variant");
        variant.serialize(&mut *self)?;
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        dbg!("Serializing map");
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        dbg!("Serializing struct");
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        dbg!("Serializing struct variant");
        variant.serialize(&mut *self)?;
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut NvListSerializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = libnvpair::NvListError;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        dbg!("Serializing In serealize seq");
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        self.is_vec = false;
        match &self.name {
            Some(name) => match &self.context_type {
                ContextType::U8Arr(arr) => Ok(self.raw_nvlist.add_uint8_arr(name, arr)?),
                ContextType::U16Arr(arr) => Ok(self.raw_nvlist.add_uint16_arr(name, arr)?),
                ContextType::U32Arr(arr) => Ok(self.raw_nvlist.add_uint32_arr(name, arr)?),
                ContextType::U64Arr(arr) => Ok(self.raw_nvlist.add_uint64_arr(name, arr)?),
                ContextType::I8Arr(arr) => Ok(self.raw_nvlist.add_int8_arr(name, arr)?),
                ContextType::I16Arr(arr) => Ok(self.raw_nvlist.add_int16_arr(name, arr)?),
                ContextType::I32Arr(arr) => Ok(self.raw_nvlist.add_int32_arr(name, arr)?),
                ContextType::I64Arr(arr) => Ok(self.raw_nvlist.add_int64_arr(name, arr)?),
                ContextType::BooleanArr(arr) => Ok(self.raw_nvlist.add_boolean_arr(name, arr)?),
                ContextType::StrArr(arr) => Ok(self.raw_nvlist.add_string_arr(name, arr)?),
                ContextType::DoubleArr(_) => todo!(),
                ContextType::NvListArr(_) => todo!(),
                _ => Err(NvListError::RestrictedOperation),
            },
            None => Err(NvListError::RestrictedOperation),
        }
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut NvListSerializer {
    type Ok = ();
    type Error = libnvpair::NvListError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        dbg!("Serializing In serealize tuple");
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut NvListSerializer {
    type Ok = ();
    type Error = libnvpair::NvListError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        dbg!("Serializing In serealize tuple struct");
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut NvListSerializer {
    type Ok = ();
    type Error = libnvpair::NvListError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        dbg!("Serializing In serealize tuple variant");
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut NvListSerializer {
    type Ok = ();
    type Error = libnvpair::NvListError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        dbg!("Serializing In serealize map key");
        let _name_ser = NameSerializer {
            name: String::new(),
        };

        key.serialize(&mut self.name_serializer)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        dbg!("Serializing In serealize map value");
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut NvListSerializer {
    type Ok = ();
    type Error = libnvpair::NvListError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        dbg!("Serializing In serealize struct field");
        self.context_type = ContextType::Empty;
        self.name = Some(key.to_string());
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.name = None;
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut NvListSerializer {
    type Ok = ();
    type Error = libnvpair::NvListError;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        dbg!("Serializing In serealize struct variant");
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[test]
    fn struct_u8() {
        #[derive(Debug, Serialize)]
        struct Test {
            a: u8,
            b: u8,
            c: u8,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };

        _to_nvlist(&test_struct).unwrap();
    }

    #[test]
    fn struct_u16() {
        #[derive(Debug, Serialize)]
        struct Test {
            a: u16,
            b: u16,
            c: u16,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };

        _to_nvlist(&test_struct).unwrap();
    }

    #[test]
    fn struct_u32() {
        #[derive(Debug, Serialize)]
        struct Test {
            a: u32,
            b: u32,
            c: u32,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };

        _to_nvlist(&test_struct).unwrap();
    }

    #[test]
    fn struct_u64() {
        #[derive(Debug, Serialize)]
        struct Test {
            a: u64,
            b: u64,
            c: u64,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };

        _to_nvlist(&test_struct).unwrap();
    }

    #[test]
    fn struct_i8() {
        #[derive(Debug, Serialize)]
        struct Test {
            a: i8,
            b: i8,
            c: i8,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };

        _to_nvlist(&test_struct).unwrap();
    }

    #[test]
    fn struct_i16() {
        #[derive(Debug, Serialize)]
        struct Test {
            a: i16,
            b: i16,
            c: i16,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };

        _to_nvlist(&test_struct).unwrap();
    }

    #[test]
    fn struct_i32() {
        #[derive(Debug, Serialize)]
        struct Test {
            a: i32,
            b: i32,
            c: i32,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };

        _to_nvlist(&test_struct).unwrap();
    }

    #[test]
    fn struct_i64() {
        #[derive(Debug, Serialize)]
        struct Test {
            a: i64,
            b: i64,
            c: i64,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };

        _to_nvlist(&test_struct).unwrap();
    }

    #[test]
    fn struct_string() {
        #[derive(Debug, Serialize)]
        struct Test {
            a: String,
            b: String,
            c: String,
        }

        let test_struct = Test {
            a: "a".to_string(),
            b: "b".to_string(),
            c: "c".to_string(),
        };

        _to_nvlist(&test_struct).unwrap();
    }

    #[test]
    fn struct_bool() {
        #[derive(Debug, Serialize)]
        struct Test {
            a: bool,
            b: bool,
            c: bool,
        }

        let test_struct = Test {
            a: true,
            b: false,
            c: true,
        };

        _to_nvlist(&test_struct).unwrap();
    }

    #[test]
    fn struct_mix_basic() {
        #[derive(Debug, Serialize)]
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

        let test_struct = Test {
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

        _to_nvlist(&test_struct).unwrap();
    }

    #[test]
    fn struct_vec_u8() {
        #[derive(Debug, PartialEq, Serialize)]
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

        _to_nvlist(&expected).unwrap();
    }

    #[test]
    fn struct_vec_u16() {
        #[derive(Debug, PartialEq, Serialize)]
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

        _to_nvlist(&expected).unwrap();
    }

    #[test]
    fn struct_vec_u32() {
        #[derive(Debug, PartialEq, Serialize)]
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

        _to_nvlist(&expected).unwrap();
    }

    #[test]
    fn struct_vec_u64() {
        #[derive(Debug, PartialEq, Serialize)]
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

        _to_nvlist(&expected).unwrap();
    }

    #[test]
    fn struct_vec_i8() {
        #[derive(Debug, PartialEq, Serialize)]
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

        _to_nvlist(&expected).unwrap();
    }

    #[test]
    fn struct_vec_i16() {
        #[derive(Debug, PartialEq, Serialize)]
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

        _to_nvlist(&expected).unwrap();
    }

    #[test]
    fn struct_vec_i32() {
        #[derive(Debug, PartialEq, Serialize)]
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

        _to_nvlist(&expected).unwrap();
    }

    #[test]
    fn struct_vec_i64() {
        #[derive(Debug, PartialEq, Serialize)]
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

        _to_nvlist(&expected).unwrap();
    }
}
