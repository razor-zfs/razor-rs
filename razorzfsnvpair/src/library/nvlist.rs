/*use super::*;

mod impls;

pub struct NvList {
    raw: *mut sys::nvlist_t,
}
*/

use super::*;
use serde::{ser, Serialize};
use std::ffi::CString;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, NvListError>;

#[derive(Clone, Debug, PartialEq)]
pub struct NvListSerializer {
    raw_nvlist: *mut sys::nvlist_t,
    pair: Option<NvPair>,
    name_serializer: NameSerializer,
}

impl NvListSerializer {
    fn try_render_name(&self) -> Result<CString> {
        let uuid = Uuid::new_v4().to_string();
        let pair_name = CString::new(uuid.as_str())?;

        Ok(pair_name)
    }
}

pub fn _to_nvlist<T>(value: &T) -> Result<*mut sys::nvlist_t>
where
    T: Serialize,
{
    let mut nvlist: *mut sys::nvlist_t = std::ptr::null_mut();
    let mut nvlist_ptr: *mut *mut sys::nvlist_t = &mut nvlist;
    let ret = unsafe { sys::nvlist_alloc(nvlist_ptr, sys::NV_UNIQUE_NAME, 0) };

    NvListError::from_nvlist_rc(ret)?;

    let mut serializer = NvListSerializer {
        raw_nvlist: unsafe { *nvlist_ptr },
        pair: None,
        name_serializer: NameSerializer {
            name: String::new(),
        },
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
    type Error = NvListError;

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
    // of the primitive types of the data model and map it to NvList data structure by adding it
    // via sys functions .
    fn serialize_bool(self, v: bool) -> Result<()> {
        let v = if v {
            sys::boolean_t::B_TRUE
        } else {
            sys::boolean_t::B_FALSE
        };

        match &mut self.pair {
            Some(nvpair) if nvpair.pair_value == ContextType::Empty => {
                nvpair.pair_value = ContextType::BooleanArr(vec![v]);
                Ok(())
            }
            Some(nvpair) => {
                if let ContextType::BooleanArr(x) = &mut nvpair.pair_value {
                    x.push(v);
                    Ok(())
                } else {
                    Err(NvListError::UnmatchingVariables)
                }
            }
            None => NvListError::from_nvlist_rc(unsafe {
                sys::nvlist_add_boolean_value(self.raw_nvlist, self.try_render_name()?.as_ptr(), v)
            }),
        }
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        match &mut self.pair {
            Some(nvpair) if nvpair.pair_value == ContextType::Empty => {
                nvpair.pair_value = ContextType::I8Arr(vec![v]);
                Ok(())
            }
            Some(nvpair) => {
                if let ContextType::I8Arr(x) = &mut nvpair.pair_value {
                    x.push(v);
                    Ok(())
                } else {
                    Err(NvListError::UnmatchingVariables)
                }
            }
            None => NvListError::from_nvlist_rc(unsafe {
                sys::nvlist_add_int8(self.raw_nvlist, self.try_render_name()?.as_ptr(), v)
            }),
        }
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        match &mut self.pair {
            Some(nvpair) if nvpair.pair_value == ContextType::Empty => {
                nvpair.pair_value = ContextType::I16Arr(vec![v]);
                Ok(())
            }
            Some(nvpair) => {
                if let ContextType::I16Arr(x) = &mut nvpair.pair_value {
                    x.push(v);
                    Ok(())
                } else {
                    Err(NvListError::UnmatchingVariables)
                }
            }
            None => NvListError::from_nvlist_rc(unsafe {
                sys::nvlist_add_int16(self.raw_nvlist, self.try_render_name()?.as_ptr(), v)
            }),
        }
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        match &mut self.pair {
            Some(nvpair) if nvpair.pair_value == ContextType::Empty => {
                nvpair.pair_value = ContextType::I32Arr(vec![v]);
                Ok(())
            }
            Some(nvpair) => {
                if let ContextType::I32Arr(x) = &mut nvpair.pair_value {
                    x.push(v);
                    Ok(())
                } else {
                    Err(NvListError::UnmatchingVariables)
                }
            }
            None => NvListError::from_nvlist_rc(unsafe {
                sys::nvlist_add_int32(self.raw_nvlist, self.try_render_name()?.as_ptr(), v)
            }),
        }
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        match &mut self.pair {
            Some(nvpair) if nvpair.pair_value == ContextType::Empty => {
                nvpair.pair_value = ContextType::I64Arr(vec![v]);
                Ok(())
            }
            Some(nvpair) => {
                if let ContextType::I64Arr(x) = &mut nvpair.pair_value {
                    x.push(v);
                    Ok(())
                } else {
                    Err(NvListError::UnmatchingVariables)
                }
            }
            None => NvListError::from_nvlist_rc(unsafe {
                sys::nvlist_add_int64(self.raw_nvlist, self.try_render_name()?.as_ptr(), v)
            }),
        }
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        match &mut self.pair {
            Some(nvpair) if nvpair.pair_value == ContextType::Empty => {
                nvpair.pair_value = ContextType::U8Arr(vec![v]);
                Ok(())
            }
            Some(nvpair) => {
                if let ContextType::U8Arr(x) = &mut nvpair.pair_value {
                    x.push(v);
                    Ok(())
                } else {
                    Err(NvListError::UnmatchingVariables)
                }
            }
            None => NvListError::from_nvlist_rc(unsafe {
                sys::nvlist_add_uint8(self.raw_nvlist, self.try_render_name()?.as_ptr(), v)
            }),
        }
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        match &mut self.pair {
            Some(nvpair) if nvpair.pair_value == ContextType::Empty => {
                nvpair.pair_value = ContextType::U16Arr(vec![v]);
                Ok(())
            }
            Some(nvpair) => {
                if let ContextType::U16Arr(x) = &mut nvpair.pair_value {
                    x.push(v);
                    Ok(())
                } else {
                    Err(NvListError::UnmatchingVariables)
                }
            }
            None => NvListError::from_nvlist_rc(unsafe {
                sys::nvlist_add_uint16(self.raw_nvlist, self.try_render_name()?.as_ptr(), v)
            }),
        }
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        match &mut self.pair {
            Some(nvpair) if nvpair.pair_value == ContextType::Empty => {
                nvpair.pair_value = ContextType::U32Arr(vec![v]);
                Ok(())
            }
            Some(nvpair) => {
                if let ContextType::U32Arr(x) = &mut nvpair.pair_value {
                    x.push(v);
                    Ok(())
                } else {
                    Err(NvListError::UnmatchingVariables)
                }
            }
            None => NvListError::from_nvlist_rc(unsafe {
                sys::nvlist_add_uint32(self.raw_nvlist, self.try_render_name()?.as_ptr(), v)
            }),
        }
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        match &mut self.pair {
            Some(nvpair) if nvpair.pair_value == ContextType::Empty => {
                nvpair.pair_value = ContextType::U64Arr(vec![v]);
                Ok(())
            }
            Some(nvpair) => {
                if let ContextType::U64Arr(x) = &mut nvpair.pair_value {
                    x.push(v);
                    Ok(())
                } else {
                    Err(NvListError::UnmatchingVariables)
                }
            }
            None => NvListError::from_nvlist_rc(unsafe {
                sys::nvlist_add_uint64(self.raw_nvlist, self.try_render_name()?.as_ptr(), v)
            }),
        }
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        match &mut self.pair {
            Some(nvpair) if nvpair.pair_value == ContextType::Empty => {
                nvpair.pair_value = ContextType::DoubleArr(vec![v]);
                Ok(())
            }
            Some(nvpair) => {
                if let ContextType::DoubleArr(x) = &mut nvpair.pair_value {
                    x.push(v);
                    Ok(())
                } else {
                    Err(NvListError::UnmatchingVariables)
                }
            }
            None => NvListError::from_nvlist_rc(unsafe {
                sys::nvlist_add_double(self.raw_nvlist, self.try_render_name()?.as_ptr(), v)
            }),
        }
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        let v = CString::new(v)?;

        match &mut self.pair {
            Some(nvpair) if nvpair.pair_value == ContextType::Empty => {
                nvpair.pair_value = ContextType::StrArr(vec![v]);
                Ok(())
            }
            Some(nvpair) => {
                if let ContextType::StrArr(x) = &mut nvpair.pair_value {
                    x.push(v);
                    Ok(())
                } else {
                    Err(NvListError::UnmatchingVariables)
                }
            }
            None => NvListError::from_nvlist_rc(unsafe {
                sys::nvlist_add_string(
                    self.raw_nvlist,
                    self.try_render_name()?.as_ptr(),
                    v.as_ptr(),
                )
            }),
        }
    }

    // Serialize a byte array as an array of bytes. Could also use a base64
    // string here. Binary formats will typically represent byte arrays more
    // compactly.
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    // An absent optional is represented as the JSON `null`.
    fn serialize_none(self) -> Result<()> {
        unimplemented!()
    }

    // A present optional is represented as just the contained value. Note that
    // this is a lossy representation. For example the values `Some(())` and
    // `None` both serialize as just `null`. Unfortunately this is typically
    // what people expect when working with JSON. Other formats are encouraged
    // to behave more intelligently if possible.
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
        if self.pair.is_none() {
            self.pair = Some(NvPair {
                pair_name: self.try_render_name()?,
                pair_value: ContextType::Empty,
            });
        }

        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        if self.pair.is_none() {
            self.pair = Some(NvPair {
                pair_name: self.try_render_name()?,
                pair_value: ContextType::Empty,
            });
        }

        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        variant.serialize(&mut *self)?;
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        if self.pair.is_none() {
            self.pair = Some(NvPair {
                pair_name: self.try_render_name()?,
                pair_value: ContextType::Empty,
            });
        }

        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        if self.pair.is_none() {
            self.pair = Some(NvPair {
                pair_name: self.try_render_name()?,
                pair_value: ContextType::Empty,
            });
        }

        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        variant.serialize(&mut *self)?;
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut NvListSerializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = NvListError;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut NvListSerializer {
    type Ok = ();
    type Error = NvListError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut NvListSerializer {
    type Ok = ();
    type Error = NvListError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut NvListSerializer {
    type Ok = ();
    type Error = NvListError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut NvListSerializer {
    type Ok = ();
    type Error = NvListError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let _name_ser = NameSerializer {
            name: String::new(),
        };

        key.serialize(&mut self.name_serializer)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut NvListSerializer {
    type Ok = ();
    type Error = NvListError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.pair = Some(NvPair {
            pair_name: CString::new(key)?,
            pair_value: ContextType::Empty,
        });

        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut NvListSerializer {
    type Ok = ();
    type Error = NvListError;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        //key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn vec_struct() {
        #[derive(Debug, Serialize, Deserialize)]
        struct VecStruct {
            vec_u32: Vec<u32>,
        }

        let test_struct = VecStruct {
            vec_u32: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        };

        _to_nvlist(&test_struct).unwrap();
    }

    #[test]
    fn primitive_struct() {
        #[derive(Debug, Serialize, Deserialize)]
        struct PrimitiveStruct {
            a: u8,
            b: u16,
            c: u32,
            d: u64,
            e: i8,
            f: i16,
            g: i32,
            h: i64,
            i: &'static str,
        }

        let test_struct = PrimitiveStruct {
            a: 3,
            b: 3,
            c: 3,
            d: 3,
            e: 3,
            f: 3,
            g: 3,
            h: 3,
            i: "test",
        };

        _to_nvlist(&test_struct).unwrap();
    }
}
