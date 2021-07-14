use super::*;

impl Pair for NvPair<u8> {
    type Value = u8;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_UINT8;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}
