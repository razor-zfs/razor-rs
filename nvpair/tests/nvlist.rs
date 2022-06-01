use razor_nvpair as nvpair;

use nvpair::data_type_t::*;
use nvpair::NvList;

#[test]
fn add_assign() {
    let mut nvlist = NvList::new();
    // let arr = [1, 2, 3, 4, 5];
    nvlist += ("u8", 3u8);
    nvlist += ("u32", 5u32);
    nvlist += ("i16", -5i16);
    nvlist += ("label", "text");

    let mut iter = dbg!(nvlist).into_iter();
    let pair1 = dbg!(iter.next().unwrap());
    let pair2 = dbg!(iter.next().unwrap());
    let pair3 = dbg!(iter.next().unwrap());
    let pair4 = dbg!(iter.next().unwrap());
    assert_eq!(pair1.name(), "u8");
    assert_eq!(pair1.r#type(), DATA_TYPE_UINT8);
    assert_eq!(pair2.name(), "u32");
    assert_eq!(pair2.r#type(), DATA_TYPE_UINT32);
    assert_eq!(pair3.name(), "i16");
    assert_eq!(pair3.r#type(), DATA_TYPE_INT16);
    assert_eq!(pair4.name(), "label");
    assert_eq!(pair4.r#type(), DATA_TYPE_STRING);
    assert_eq!(None, iter.next());
}
