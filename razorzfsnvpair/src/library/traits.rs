use super::*;

pub trait Pair {
    type Value;
    const DATA_TYPE: sys::data_type_t;

    fn key(&self) -> &str;
    fn value(&self) -> &Self::Value;
    fn r#type(&self) -> sys::data_type_t {
        Self::DATA_TYPE
    }
}
