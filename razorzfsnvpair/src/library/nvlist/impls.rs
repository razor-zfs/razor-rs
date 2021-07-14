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
    (add_u32, u32, sys::nvlist_add_uint32)
);
