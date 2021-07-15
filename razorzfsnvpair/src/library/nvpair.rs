use super::*;
use std::ffi::CString;

mod impls;
mod impls_trait;

pub struct NvPair {
    pub name: CString,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BOOL(sys::boolean_t),
    FLOAT(f64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    String(CString),
    EMPTY,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nvpair_u8() {
        let nvp = NvPair::from_u8("test_u8", 3_u8);
        if let Ok(nvpair) = nvp {
            assert_eq!(
                nvpair.name(),
                &CString::new("test_u8").expect("can't parse String to CString")
            );
            assert_eq!(nvpair.value(), &Value::U8(3_u8));
        } else {
            panic!("u8 returned error")
        }
    }

    #[test]
    fn nvpair_u16() {
        let nvp = NvPair::from_u16("test_u16", 3_u16);
        if let Ok(nvpair) = nvp {
            assert_eq!(
                nvpair.name(),
                &CString::new("test_u16").expect("can't parse String to CString")
            );
            assert_eq!(nvpair.value(), &Value::U16(3_u16));
        } else {
            panic!("u16 returned error")
        }
    }

    #[test]
    fn nvpair_u32() {
        let nvp = NvPair::from_u32("test_u32", 3_u32);
        if let Ok(nvpair) = nvp {
            assert_eq!(
                nvpair.name(),
                &CString::new("test_u32").expect("can't parse String to CString")
            );
            assert_eq!(nvpair.value(), &Value::U32(3_u32));
        } else {
            panic!("u32 returned error")
        }
    }

    #[test]
    fn nvpair_u64() {
        let nvp = NvPair::from_u64("test_u64", 3_u64);
        if let Ok(nvpair) = nvp {
            assert_eq!(
                nvpair.name(),
                &CString::new("test_u64").expect("can't parse String to CString")
            );
            assert_eq!(nvpair.value(), &Value::U64(3_u64));
        } else {
            panic!("u64 returned error")
        }
    }

    #[test]
    fn nvpair_i8() {
        let nvp = NvPair::from_i8("test_i8", 3_i8);
        if let Ok(nvpair) = nvp {
            assert_eq!(
                nvpair.name(),
                &CString::new("test_i8").expect("can't parse String to CString")
            );
            assert_eq!(nvpair.value(), &Value::I8(3_i8));
        } else {
            panic!("i8 returned error")
        }
    }

    #[test]
    fn nvpair_i16() {
        let nvp = NvPair::from_i16("test_i16", 3_i16);
        if let Ok(nvpair) = nvp {
            assert_eq!(
                nvpair.name(),
                &CString::new("test_i16").expect("can't parse String to CString")
            );
            assert_eq!(nvpair.value(), &Value::I16(3_i16));
        } else {
            panic!("i16 returned error")
        }
    }

    #[test]
    fn nvpair_i32() {
        let nvp = NvPair::from_i32("test_i32", 3_i32);
        if let Ok(nvpair) = nvp {
            assert_eq!(
                nvpair.name(),
                &CString::new("test_i32").expect("can't parse String to CString")
            );
            assert_eq!(nvpair.value(), &Value::I32(3_i32));
        } else {
            panic!("i32 returned error")
        }
    }

    #[test]
    fn nvpair_i64() {
        let nvp = NvPair::from_i64("test_i64", 3_i64);
        if let Ok(nvpair) = nvp {
            assert_eq!(
                nvpair.name(),
                &CString::new("test_i64").expect("can't parse String to CString")
            );
            assert_eq!(nvpair.value(), &Value::I64(3_i64));
        } else {
            panic!("i64 returned error")
        }
    }

    #[test]
    fn nvpair_bool_true() {
        let nvp = NvPair::from_bool("test_bool", true);
        if let Ok(nvpair) = nvp {
            assert_eq!(
                nvpair.name(),
                &CString::new("test_bool").expect("can't parse String to CString")
            );
            assert_eq!(nvpair.value(), &Value::BOOL(sys::boolean_t::B_TRUE));
        } else {
            panic!("bool returned error")
        }
    }

    #[test]
    fn nvpair_bool_false() {
        let nvp = NvPair::from_bool("test_bool", false);
        if let Ok(nvpair) = nvp {
            assert_eq!(
                nvpair.name(),
                &CString::new("test_bool").expect("can't parse String to CString")
            );
            assert_eq!(nvpair.value(), &Value::BOOL(sys::boolean_t::B_FALSE));
        } else {
            panic!("bool returned error")
        }
    }

    #[test]
    fn nvpair_f64() {
        let nvp = NvPair::from_f64("test_f64", 3.14);
        if let Ok(nvpair) = nvp {
            assert_eq!(
                nvpair.name(),
                &CString::new("test_f64").expect("can't parse String to CString")
            );
            assert_eq!(nvpair.value(), &Value::FLOAT(3.14));
        } else {
            panic!("f64 returned error")
        }
    }

    #[test]
    fn nvpair_cstring() {
        let nvp = NvPair::from_str("test_cstring", "random string");
        if let Ok(nvpair) = nvp {
            assert_eq!(
                nvpair.name(),
                &CString::new("test_cstring").expect("can't parse String name to CString")
            );
            assert_eq!(
                nvpair.value(),
                &Value::String(
                    CString::new("random string").expect("can't parse String value to CString")
                )
            );
        } else {
            panic!("f64 returned error")
        }
    }
}
