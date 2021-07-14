use crate::NvListError;

use super::*;

impl NvList {
    fn add(&mut self, nvp: NvPair) -> Result<(), NvListError> {
        let nvl = self.raw;
        let name = nvp.name.as_ptr();
        let rc = unsafe {
            match nvp.value {
                nvpair::Value::U8(value) => sys::nvlist_add_uint8(nvl, name, value),
                nvpair::Value::U16(value) => sys::nvlist_add_uint16(nvl, name, value),
                nvpair::Value::String(value) => sys::nvlist_add_string(nvl, name, value.as_ptr()),
                nvpair::Value::U32(_) => todo!(),
                nvpair::Value::U64(_) => todo!(),
                nvpair::Value::I8(_) => todo!(),
                nvpair::Value::I16(_) => todo!(),
                nvpair::Value::I32(_) => todo!(),
                nvpair::Value::I64(_) => todo!(),
                nvpair::Value::BOOL(_) => todo!(),
                nvpair::Value::EMPTY => todo!(),
            }
        };
        NvListError::from_nvlist_rc(rc)
    }
}
