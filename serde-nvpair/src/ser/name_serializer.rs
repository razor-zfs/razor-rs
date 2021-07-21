use super::*;
use serde::{ser, Serialize};

pub type Result<T> = std::result::Result<T, libnvpair::NvListError>;

#[derive(Clone, Debug, PartialEq)]
pub struct NameSerializer {
    pub name: String,
}

impl<'a> ser::Serializer for &'a mut NameSerializer {
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

    // Here we go with the simple methods. The following 12 methods receive one
    // of the primitive types of the data model and map it to JSON by appending
    // into the output string.
    fn serialize_bool(self, _v: bool) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_i8(self, _v: i8) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_i16(self, _v: i16) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_i32(self, _v: i32) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_i64(self, _v: i64) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_u8(self, _v: u8) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_u16(self, _v: u16) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_u32(self, _v: u32) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_u64(self, _v: u64) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.name += "\"";
        self.name += v;
        self.name += "\"";
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_none(self) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_unit(self) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(libnvpair::NvListError::NameTypeError)
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
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(libnvpair::NvListError::NameTypeError)
    }
}

impl<'a> ser::SerializeSeq for &'a mut NameSerializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = libnvpair::NvListError;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn end(self) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }
}

impl<'a> ser::SerializeTuple for &'a mut NameSerializer {
    type Ok = ();
    type Error = libnvpair::NvListError;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn end(self) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut NameSerializer {
    type Ok = ();
    type Error = libnvpair::NvListError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn end(self) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut NameSerializer {
    type Ok = ();
    type Error = libnvpair::NvListError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn end(self) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }
}

impl<'a> ser::SerializeMap for &'a mut NameSerializer {
    type Ok = ();
    type Error = libnvpair::NvListError;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn end(self) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }
}

impl<'a> ser::SerializeStruct for &'a mut NameSerializer {
    type Ok = ();
    type Error = libnvpair::NvListError;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn end(self) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut NameSerializer {
    type Ok = ();
    type Error = libnvpair::NvListError;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(libnvpair::NvListError::NameTypeError)
    }

    fn end(self) -> Result<()> {
        Err(libnvpair::NvListError::NameTypeError)
    }
}
