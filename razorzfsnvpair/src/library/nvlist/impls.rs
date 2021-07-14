use super::*;

macro_rules! generate_func_call {
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

generate_func_call!(
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
