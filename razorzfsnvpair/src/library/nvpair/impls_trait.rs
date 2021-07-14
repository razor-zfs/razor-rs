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

impl Pair for NvPair<u16> {
    type Value = u16;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_UINT16;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<u32> {
    type Value = u32;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_UINT32;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<u64> {
    type Value = u64;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_UINT64;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<i8> {
    type Value = i8;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_INT8;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<i16> {
    type Value = i16;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_INT16;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<i32> {
    type Value = i32;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_INT32;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<i64> {
    type Value = i64;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_INT64;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<bool> {
    type Value = sys::boolean_t;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_BOOLEAN;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        if self.value {
            &sys::boolean_t::B_TRUE
        } else {
            &sys::boolean_t::B_FALSE
        }
    }
}

impl Pair for NvPair<Vec<bool>> {
    type Value = Vec<bool>;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_BOOLEAN_ARRAY;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<String> {
    type Value = String;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_STRING;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<Vec<u8>> {
    type Value = Vec<u8>;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_UINT8_ARRAY;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<Vec<u16>> {
    type Value = Vec<u16>;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_UINT16_ARRAY;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<Vec<u32>> {
    type Value = Vec<u32>;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_UINT32_ARRAY;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<Vec<u64>> {
    type Value = Vec<u64>;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_UINT64_ARRAY;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<Vec<i8>> {
    type Value = Vec<i8>;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_INT8_ARRAY;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<Vec<i16>> {
    type Value = Vec<i16>;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_INT16_ARRAY;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<Vec<i32>> {
    type Value = Vec<i32>;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_INT32_ARRAY;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<Vec<i64>> {
    type Value = Vec<i64>;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_INT64_ARRAY;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<NvList> {
    type Value = NvList;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_NVLIST;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl Pair for NvPair<Vec<NvList>> {
    type Value = Vec<NvList>;

    const DATA_TYPE: sys::data_type_t = sys::data_type_t::DATA_TYPE_NVLIST_ARRAY;

    fn key(&self) -> &str {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}
