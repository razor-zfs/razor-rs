use super::*;

impl NvPair {
    #[inline]
    fn from_value(name: &str, value: Value) -> Result<Self, crate::NvListError> {
        let name = CString::new(name)?;
        Ok(Self { name, value })
    }

    pub(crate) fn from_bool(name: &str, value: bool) -> Result<Self, crate::NvListError> {
        let mut nval = Value::EMPTY;

        if value {
            nval = Value::BOOL(sys::boolean_t::B_TRUE);
        } else {
            nval = Value::BOOL(sys::boolean_t::B_FALSE);
        }

        Self::from_value(name, nval)
    }

    pub(crate) fn from_u8(name: &str, value: u8) -> Result<Self, crate::NvListError> {
        let value = Value::U8(value);
        Self::from_value(name, value)
    }

    pub(crate) fn from_u16(name: &str, value: u16) -> Result<Self, crate::NvListError> {
        let value = Value::U16(value);
        Self::from_value(name, value)
    }

    pub(crate) fn from_u32(name: &str, value: u32) -> Result<Self, crate::NvListError> {
        let value = Value::U32(value);
        Self::from_value(name, value)
    }

    pub(crate) fn from_u64(name: &str, value: u64) -> Result<Self, crate::NvListError> {
        let value = Value::U64(value);
        Self::from_value(name, value)
    }

    pub(crate) fn from_i8(name: &str, value: i8) -> Result<Self, crate::NvListError> {
        let value = Value::I8(value);
        Self::from_value(name, value)
    }

    pub(crate) fn from_i16(name: &str, value: i16) -> Result<Self, crate::NvListError> {
        let value = Value::I16(value);
        Self::from_value(name, value)
    }

    pub(crate) fn from_i32(name: &str, value: i32) -> Result<Self, crate::NvListError> {
        let value = Value::I32(value);
        Self::from_value(name, value)
    }

    pub(crate) fn from_i64(name: &str, value: i64) -> Result<Self, crate::NvListError> {
        let value = Value::I64(value);
        Self::from_value(name, value)
    }

    pub(crate) fn from_str(name: &str, value: &str) -> Result<Self, crate::NvListError> {
        let value = CString::new(value).map(Value::String)?;
        Self::from_value(name, value)
    }
}
