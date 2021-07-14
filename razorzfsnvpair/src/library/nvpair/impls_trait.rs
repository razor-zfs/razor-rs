use super::*;

impl IntoNvPair for bool {
    fn into_nvpair(name: impl AsRef<str>, this: Self) -> Result<NvPair, NvListError> {
        NvPair::from_bool(name.as_ref(), this)
    }
}

impl IntoNvPair for u8 {
    fn into_nvpair(name: impl AsRef<str>, this: Self) -> Result<NvPair, NvListError> {
        NvPair::from_u8(name.as_ref(), this)
    }
}

impl IntoNvPair for u16 {
    fn into_nvpair(name: impl AsRef<str>, this: Self) -> Result<NvPair, NvListError> {
        NvPair::from_u16(name.as_ref(), this)
    }
}

impl IntoNvPair for u32 {
    fn into_nvpair(name: impl AsRef<str>, this: Self) -> Result<NvPair, NvListError> {
        NvPair::from_u32(name.as_ref(), this)
    }
}

impl IntoNvPair for u64 {
    fn into_nvpair(name: impl AsRef<str>, this: Self) -> Result<NvPair, NvListError> {
        NvPair::from_u64(name.as_ref(), this)
    }
}

impl IntoNvPair for i8 {
    fn into_nvpair(name: impl AsRef<str>, this: Self) -> Result<NvPair, NvListError> {
        NvPair::from_i8(name.as_ref(), this)
    }
}

impl IntoNvPair for i16 {
    fn into_nvpair(name: impl AsRef<str>, this: Self) -> Result<NvPair, NvListError> {
        NvPair::from_i16(name.as_ref(), this)
    }
}

impl IntoNvPair for i32 {
    fn into_nvpair(name: impl AsRef<str>, this: Self) -> Result<NvPair, NvListError> {
        NvPair::from_i32(name.as_ref(), this)
    }
}

impl IntoNvPair for i64 {
    fn into_nvpair(name: impl AsRef<str>, this: Self) -> Result<NvPair, NvListError> {
        NvPair::from_i64(name.as_ref(), this)
    }
}

impl IntoNvPair for &str {
    fn into_nvpair(name: impl AsRef<str>, this: Self) -> Result<NvPair, NvListError> {
        NvPair::from_str(name.as_ref(), this)
    }
}

impl IntoNvPair for String {
    fn into_nvpair(name: impl AsRef<str>, this: Self) -> Result<NvPair, NvListError> {
        NvPair::from_str(name.as_ref(), &this)
    }
}
