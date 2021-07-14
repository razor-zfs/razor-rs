use super::*;

macro_rules! basic_nvlist_functions {
    ($(($name: ident, $mytype: ty, $callback: path)),*) => {
        impl NvList {
            $(
                fn $name<T: Pair<Value = $mytype>>(&mut self, nvp: T) {
                    let name = nvp.key().as_ptr();
                    let value = *nvp.value();

                    unsafe { $callback(self.raw, name, value) };
                }
            )*
        }
    };
}

macro_rules! vector_nvlist_functions {
    ($(($name: ident, $mytype: ty, $callback: path)),*) => {
        impl NvList {
            $(
                fn $name<T: Pair<Value = $mytype>>(&mut self, nvp: T) {
                    let name = nvp.key().as_ptr();
                    let value = &*nvp.value();
                    let mut value = value.clone();

                    unsafe { $callback(self.raw, name, value.as_mut_ptr(), value.len() as u32) };
                }
            )*
        }
    };
}

basic_nvlist_functions!(
    (add_u8, u8, sys::nvlist_add_uint8),
    (add_u16, u16, sys::nvlist_add_uint16),
    (add_u32, u32, sys::nvlist_add_uint32),
    (add_u64, u64, sys::nvlist_add_uint64),
    (add_i8, i8, sys::nvlist_add_int8),
    (add_i16, i16, sys::nvlist_add_int16),
    (add_i32, i32, sys::nvlist_add_int32),
    (add_i64, i64, sys::nvlist_add_int64),
    (add_bool, sys::boolean_t, sys::nvlist_add_boolean_value)
);

vector_nvlist_functions!(
    (add_u8_arr, Vec<u8>, sys::nvlist_add_uint8_array),
    (add_u16_arr, Vec<u16>, sys::nvlist_add_uint16_array),
    (add_u132_arr, Vec<u32>, sys::nvlist_add_uint32_array),
    (add_u64_arr, Vec<u64>, sys::nvlist_add_uint64_array),
    (add_i8_arr, Vec<i8>, sys::nvlist_add_int8_array),
    (add_i16_arr, Vec<i16>, sys::nvlist_add_int16_array),
    (add_i132_arr, Vec<i32>, sys::nvlist_add_int32_array),
    (add_i64_arr, Vec<i64>, sys::nvlist_add_int64_array)
);
