use razorzfsnvpair_sys as sys;

use traits::Pair;

mod nvlist;
mod nvpair;
mod traits;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nvpair_u8() {
        let nvp = nvpair::NvPair::new("key", 6_u8);
        assert_eq!(nvp.r#type(), sys::data_type_t::DATA_TYPE_UINT8);
        assert_eq!(nvp.key(), "key");
        assert_eq!(nvp.value(), &6);
    }
}
